use ascii_ui::{
    attachments::{self, Flex, MainAxisAlignment},
    mouse::InteractableMarker,
    widget_builder::{WidgetBuilder, WidgetBuilderFn, WidgetSaver},
    widgets::{self, Divider, FlexWidget, SingleChildWidget},
    FlexDirection,
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

    let settings_tab = FlexWidget::column(vec![
        InfoCounts::build(),
        Divider::build('-'),
        DebugOptions::build(),
    ])(&mut commands);

    let list_builder_tab = {
        let mut rows = vec![];
        let mut list_builder = Entity::PLACEHOLDER;
        rows.push(
            widgets::ListBuilderWidget::build::<widgets::FlexWidget>(
                Box::new(|_, i: &usize| widgets::Text::build(format!("{}", i))),
                vec![0, 2, 5],
                FlexDirection::Vertical,
            )
            .save_id(&mut list_builder)
            .apply(&mut commands),
        );

        rows.push(widgets::Divider::build('-'));
        rows.push(
            widgets::FlexWidget::row(vec![
                widgets::Button::build("+").with(ItemMutateButton {
                    target: list_builder,
                    mode: MutateMode::Add,
                }),
                widgets::Button::build("-").with(ItemMutateButton {
                    target: list_builder,
                    mode: MutateMode::Remove,
                }),
            ])
            .with(MainAxisAlignment::SpaceAround),
        );
        rows.push(widgets::Divider::build('-'));

        widgets::FlexWidget::column(rows)
    }(&mut commands);

    let tileset_tab = {
        widgets::FlexWidget::column(vec![
            widgets::Button::build("Save").with(SaveTilemapButton),
            widgets::ListBuilderWidget::<(TilesetSource, Handle<TilesetSource>)>::build::<
                widgets::FlexWidget,
            >(
                Box::new(|_, (source, handle)| build_tileset_ui(source, handle.clone())),
                vec![],
                FlexDirection::Vertical,
            )
            .with(TilesetHandles {
                handles: vec![server.load("tilesets/cave.tileset.ron")],
            }),
        ])
    }(&mut commands);

    SingleChildWidget::build(Some(widgets::TabView::build(vec![
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
    widgets::FlexWidget::column(vec![
        widgets::FlexWidget::row(vec![
            widgets::Divider::build('=').with(Flex::new(1)),
            widgets::Text::build(" Layers "),
            widgets::Divider::build('=').with(Flex::new(1)),
        ]),
        widgets::SingleChildWidget::build(None).with(attachments::SizedBox::vertical(1)),
        widgets::FlexWidget::row(vec![
            widgets::Checkbox::build(),
            widgets::SingleChildWidget::build(None).with(attachments::SizedBox::horizontal(1)),
            widgets::Text::build("background").with(Flex::new(1)),
            widgets::Button::build_raw("^"),
            widgets::SingleChildWidget::build(None).with(attachments::SizedBox::horizontal(1)),
            widgets::Button::build_raw("v"),
        ]),
        widgets::FlexWidget::row(vec![
            widgets::Checkbox::build(),
            widgets::SingleChildWidget::build(None).with(attachments::SizedBox::horizontal(1)),
            widgets::Text::build("layer 1").with(Flex::new(1)),
            widgets::Button::build_raw("^"),
            widgets::SingleChildWidget::build(None).with(attachments::SizedBox::horizontal(1)),
            widgets::Button::build_raw("v"),
        ])
        .with(attachments::MainAxisAlignment::SpaceBetween),
        widgets::FlexWidget::row(vec![
            widgets::Checkbox::build(),
            widgets::SingleChildWidget::build(None).with(attachments::SizedBox::horizontal(1)),
            widgets::Text::build("layer 2").with(Flex::new(1)),
            widgets::Button::build_raw("^"),
            widgets::SingleChildWidget::build(None).with(attachments::SizedBox::horizontal(1)),
            widgets::Button::build_raw("v"),
        ])
        .with(attachments::MainAxisAlignment::SpaceBetween),
        widgets::SingleChildWidget::build(None).with(attachments::SizedBox::vertical(1)),
        widgets::FlexWidget::row(vec![
            widgets::SingleChildWidget::build(Some(widgets::Text::build("New Layer")))
                .with(attachments::Flex::new(1))
                .with(attachments::Padding::symmetric(1, 0)),
            widgets::Button::build("Create"),
        ]),
        widgets::Divider::build('-'),
    ])
}
#[derive(Debug, Component, Clone)]
pub(crate) struct TilesetTileId {
    pub(crate) tileset: Handle<TilesetSource>,
    pub(crate) tile: u32,
}
