use ascii_ui::{
    attachments::{self, MainAxisAlignment},
    mouse::IntractableMarker,
    widget_builder::{WidgetBuilder, WidgetBuilderFn, WidgetSaver},
    widgets::{self, Checkbox, Column, Container, Divider, Text},
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

use bevy_ascii_game::{physics_grids::UiPhysicsGridMarker, tileset::asset::TilesetSource};

use crate::list_builder_widget::ListBuilderWidget;

use super::{state::TilesetPanelState, update::TilesetHandles};

#[derive(Debug, Component)]
pub struct ItemMutateButton {
    pub target: Entity,
    pub mode: MutateMode,
}

#[derive(Debug)]
pub enum MutateMode {
    Add,
    Remove,
}

pub fn setup_ui(
    mut commands: Commands,
    mut menu_state: ResMut<TilesetPanelState>,
    server: Res<AssetServer>,
) {
    let menu_state = &mut *menu_state;

    let settings_tab = Column::build(vec![
        Text::build("".into()).save_id(&mut menu_state.fps_text),
        Text::build("".into()).save_id(&mut menu_state.player_count_text),
        Text::build("".into()).save_id(&mut menu_state.actor_count_text),
        Text::build("".into()).save_id(&mut menu_state.solid_count_text),
        Divider::build('-'),
        Checkbox::build("Debug Position".into()).save_id(&mut menu_state.position_checkbox),
        Checkbox::build("Debug Colliders".into()).save_id(&mut menu_state.colliders_checkbox),
        Checkbox::build("Debug ECS UI".into()).save_id(&mut menu_state.ui_checkbox),
        Checkbox::build("Pause Physics".into()).save_id(&mut menu_state.pause_checkbox),
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
        ListBuilderWidget::<(TilesetSource, Handle<TilesetSource>)>::build::<widgets::Column>(
            Box::new(|_, (source, handle)| build_tileset_ui(source, handle.clone())),
            vec![],
            (),
        )
        .with(TilesetHandles {
            handles: vec![server.load("tilesets/cave.tileset.ron")],
        })
    }(&mut commands);

    Container::build(Some(widgets::TabView::build(vec![
        ("Settings".into(), settings_tab),
        ("List Builder".into(), list_builder_tab),
        ("Tileset".into(), tileset_tab),
    ])))
    .with((
        attachments::Root {
            enabled: true,
            position: IVec2 { x: 0, y: -16 },
            size: UVec2 { x: 32, y: 32 },
        },
        UiPhysicsGridMarker,
        attachments::Border::symmetric(Some('|'), Some('-'), Some([',', '.', '`', '\''])).padded(),
        attachments::RenderBundle::default(),
        DebugMenuMarker,
        IntractableMarker,
    ))
    .save_id(&mut menu_state.root_widget)(&mut commands);
}

#[derive(Debug, Component)]
pub struct DebugMenuMarker;

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
                Box::new(move |index, item: &Vec<String>| {
                    widgets::Texture::build(item.clone(), tile_size).with((
                        IntractableMarker,
                        TilesetTileId {
                            tile: index as u32,
                            tileset: handle.clone(),
                        },
                    ))
                }),
                source.tiles.clone(),
                source.tile_size,
            )])
            .with(attachments::SizedBox::vertical(26)),
        )),
    ])
}
#[derive(Debug, Component, Clone)]
pub struct TilesetTileId {
    pub tileset: Handle<TilesetSource>,
    pub tile: u32,
}
