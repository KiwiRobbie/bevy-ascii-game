use bevy::prelude::*;

use ascii_ui::{
    attachments::{self},
    mouse::InteractableMarker,
    widget_builder::{WidgetBuilder, WidgetBuilderFn, WidgetSaver},
    widgets::{self, Divider, FlexWidget, SingleChildWidget},
    FlexDirection,
};
use std::sync::Arc;

use bevy_ascii_game::{
    physics_grids::UiPhysicsGridMarker,
    tileset::asset::TilesetSource,
    widgets::{DebugOptions, InfoCounts},
};
use glyph_render::glyph_render_plugin::GlyphTextureSource;

use crate::list_builder_widget::ListBuilderWidget;

use super::state::TilesetPanelState;

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

pub(super) fn setup_ui(mut commands: Commands, mut menu_state: ResMut<TilesetPanelState>) {
    let menu_state = &mut *menu_state;

    let settings_tab = Box::new(|commands: &mut Commands| {
        FlexWidget::column(vec![
            InfoCounts::build(),
            Divider::build('-'),
            DebugOptions::build(),
        ])(commands)
    });

    let tileset_tab = Box::new(|commands: &mut Commands| {
        // let server = world.resource::<AssetServer>();

        widgets::FlexWidget::column(vec![
            widgets::Button::build("Save").with(SaveTilemapButton),
            ListBuilderWidget::<(TilesetSource, Handle<TilesetSource>)>::build::<widgets::FlexWidget>(
                Box::new(|_, (source, handle)| build_tileset_ui(source, handle.clone())),
                vec![],
                FlexDirection::Vertical,
            ), // .with(TilesetHandles {
               //     handles: vec![server.load("tilesets/bridge.tileset.ron")],
               // }),
        ])(commands)
    });

    SingleChildWidget::build(Some(widgets::TabView::build(vec![
        ("Settings", settings_tab),
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
    widgets::FlexWidget::column(vec![
        widgets::Text::build(source.display_name.clone()),
        widgets::Text::build(format!(
            "id: '{}', size: {}x{}",
            source.id.clone(),
            source.tile_size.x,
            source.tile_size.y
        )),
        widgets::Divider::build('-'),
        widgets::SingleChildWidget::build(Some(
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
