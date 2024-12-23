use std::sync::Arc;

use ascii_ui::{
    attachments::{self, MainAxisAlignment},
    mouse::InteractableMarker,
    widget_builder::{WidgetBuilder, WidgetBuilderFn, WidgetSaver},
    widgets::{self, Column, Container, Divider},
};
use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Res, ResMut},
    },
    math::{IVec2, UVec2},
};

use bevy_ascii_game::{
    physics_grids::UiPhysicsGridMarker,
    tileset::asset::TilesetSource,
    widgets::{DebugOptions, InfoCounts},
};
use glyph_render::glyph_render_plugin::GlyphTextureSource;

use crate::list_builder_widget::ListBuilderWidget;

use super::{state::TilesetPanelState, update::TilesetHandles};

#[derive(Debug, Component)]
pub(crate) struct ItemMutateButton {
    pub(crate) target: Entity,
    pub(crate) mode: MutateMode,
}

#[derive(Debug)]
pub(crate) enum MutateMode {
    Add,
    Remove,
}

pub(super) fn setup_ui(
    mut commands: Commands,
    mut menu_state: ResMut<TilesetPanelState>,
    server: Res<AssetServer>,
) {
    let menu_state = &mut *menu_state;

    let settings_tab = Column::build(vec![
        InfoCounts::build(),
        Divider::build('-'),
        DebugOptions::build(),
    ])(&mut commands);

    let list_builder_tab = {
        let mut rows = vec![];
        let mut list_builder = Entity::PLACEHOLDER;
        rows.push(
            ListBuilderWidget::build::<widgets::Column>(
                Box::new(|_, i: &usize| widgets::Text::build(format!("{}", i))),
                vec![0, 2, 5],
                (),
            )
            .save_id(&mut list_builder)
            .apply(&mut commands),
        );

        rows.push(widgets::Divider::build('-'));
        rows.push(
            widgets::Row::build(vec![
                widgets::Button::build("+".into()).with(ItemMutateButton {
                    target: list_builder,
                    mode: MutateMode::Add,
                }),
                widgets::Button::build("-".into()).with(ItemMutateButton {
                    target: list_builder,
                    mode: MutateMode::Remove,
                }),
            ])
            .with(MainAxisAlignment::SpaceAround),
        );
        rows.push(widgets::Divider::build('-'));

        widgets::Column::build(rows)
    }(&mut commands);

    let tileset_tab = {
        widgets::Column::build(vec![
            widgets::Button::build("Save".into()).with(SaveTilemapButton),
            ListBuilderWidget::<(TilesetSource, Handle<TilesetSource>)>::build::<widgets::Column>(
                Box::new(|_, (source, handle)| build_tileset_ui(source, handle.clone())),
                vec![],
                (),
            )
            .with(TilesetHandles {
                handles: vec![server.load("tilesets/bridge.tileset.ron")],
            }),
        ])
    }(&mut commands);

    Container::build(Some(widgets::TabView::build(vec![
        ("Settings", settings_tab),
        ("List Builder", list_builder_tab),
        ("Tileset", tileset_tab),
    ])))
    .with((
        attachments::Root {
            enabled: true,
            position: IVec2 { x: 0, y: -16 },
            size: UVec2 { x: 32, y: 32 },
        },
        UiPhysicsGridMarker,
        attachments::Border::symmetric(
            Some('|'),
            Some('-'),
            [Some(','), Some('.'), Some('`'), Some('\'')],
        )
        .padded(),
        attachments::RenderBundle::default(),
        DebugMenuMarker,
        InteractableMarker,
    ))
    .save_id(&mut menu_state.root_widget)(&mut commands);
}

#[derive(Debug, Component)]
pub(crate) struct SaveTilemapButton;

#[derive(Debug, Component)]
pub(crate) struct DebugMenuMarker;

fn build_tileset_ui<'a>(
    source: &TilesetSource,
    handle: Handle<TilesetSource>,
) -> WidgetBuilderFn<'a> {
    let tile_size = source.tile_size;
    widgets::Column::build(vec![
        widgets::Text::build(source.display_name.clone()),
        widgets::Text::build(format!(
            "id: '{}', size: {}x{}",
            source.id.clone(),
            source.tile_size.x,
            source.tile_size.y
        )),
        widgets::Divider::build('-'),
        widgets::Container::build(Some(
            widgets::ScrollingView::build(vec![ListBuilderWidget::build::<widgets::Grid>(
                Box::new(move |index, item: &Arc<GlyphTextureSource>| {
                    if item.data.len() == (tile_size.x * tile_size.y) as usize {
                        widgets::Texture::build(item.data.clone(), tile_size).with((
                            InteractableMarker,
                            TilesetTileId {
                                tile: index as u32,
                                tileset: handle.clone(),
                            },
                        ))
                    } else {
                        widgets::Text::build("???")
                    }
                }),
                source.tiles.clone(),
                source.tile_size,
            )])
            .with((attachments::SizedBox::vertical(26),)),
        )),
    ])
}
#[derive(Debug, Component, Clone)]
pub(crate) struct TilesetTileId {
    pub(crate) tileset: Handle<TilesetSource>,
    pub(crate) tile: u32,
}
