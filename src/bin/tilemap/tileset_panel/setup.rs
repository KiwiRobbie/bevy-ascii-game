use ascii_ui::{
    attachments::{self, MainAxisAlignment},
    widget_builder::{WidgetBuilder, WidgetSaver},
    widgets::{self, Checkbox, Column, Container, Divider, Text},
};
use bevy::{
    asset::{Asset, Assets},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Res, ResMut},
    },
    math::{IVec2, UVec2},
};

use bevy_ascii_game::{physics_grids::UiPhysicsGridMarker, tileset::asset::TilesetSource};

use crate::{list_builder_widget::ListBuilderWidget, tileset_widget::widget::TilesetWidget};

use super::state::TilesetPanelState;

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
    tileset_source: Res<Assets<TilesetSource>>,
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
    ]);
    let mut rows = vec![settings_tab];

    rows.push(widgets::Divider::build('.'));

    let mut list_builder = Entity::PLACEHOLDER;
    rows.push(
        ListBuilderWidget::build(
            Box::new(|i: &usize| widgets::Text::build(format!("{}", i).into())),
            vec![0, 2, 5],
        )
        .save_id(&mut list_builder)
        .apply(&mut commands),
    );

    rows.push(widgets::Divider::build('.'));
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
    rows.push(widgets::Divider::build('.'));

    for tileset in menu_state.tilesets.iter() {
        let Some(tileset) = tileset_source.get(tileset.id()) else {
            continue;
        };
        rows.push(TilesetWidget::build(tileset));
    }

    Container::build(Some(widgets::Column::build(rows)))
        .with((
            attachments::Root {
                enabled: true,
                position: IVec2 { x: 0, y: -16 },
                size: UVec2 { x: 32, y: 24 },
            },
            UiPhysicsGridMarker,
            attachments::Border::symmetric(Some('|'), Some('-'), Some([',', '.', '`', '\'']))
                .padded(),
            attachments::RenderBundle::default(),
            DebugMenuMarker,
        ))
        .save_id(&mut menu_state.root_widget)(&mut commands);
}

#[derive(Debug, Component)]
pub struct DebugMenuMarker;
