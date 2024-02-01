use ascii_ui::{
    attachments::{self, border::Border},
    widgets,
};
use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        system::{Commands, Res, ResMut},
    },
    math::{IVec2, UVec2},
    prelude::Entity,
};

use super::{inspector::InspectorTab, state::DebugMenuState};

pub fn setup_ui(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut menu_state: ResMut<DebugMenuState>,
) {
    let f3: Entity = widgets::TextBundle::spawn(&mut commands, "[F3 Debug Menu]".into(), ());

    let divider_a: Entity = widgets::DividerBundle::spawn(&mut commands, '=');

    let player_count: Entity = widgets::TextBundle::spawn(&mut commands, "Text A".into(), ());
    let solid_count: Entity = widgets::TextBundle::spawn(&mut commands, "Text B".into(), ());
    let actor_count: Entity = widgets::TextBundle::spawn(&mut commands, "Text c".into(), ());
    menu_state.player_count_text = Some(player_count);
    menu_state.solid_count_text = Some(solid_count);
    menu_state.actor_count_text = Some(actor_count);

    let divider_b: Entity = widgets::DividerBundle::spawn(&mut commands, '-');

    let debug_position = widgets::CheckboxBuilder::spawn(&mut commands, "Debug Position".into());
    let debug_colliders = widgets::CheckboxBuilder::spawn(&mut commands, "Debug Colliders".into());
    let debug_ui = widgets::CheckboxBuilder::spawn(&mut commands, "Debug ECS UI".into());
    menu_state.position_checkbox = Some(debug_position);
    menu_state.colliders_checkbox = Some(debug_colliders);
    menu_state.ui_checkbox = Some(debug_ui);

    let settings_column = widgets::ColumnBundle::spawn(
        &mut commands,
        vec![
            player_count,
            solid_count,
            actor_count,
            divider_b,
            debug_position,
            debug_colliders,
            debug_ui,
        ],
        (),
    );

    let inspector_column =
        widgets::ColumnBundle::spawn(&mut commands, vec![], InspectorTab::default());

    let container_inner = widgets::tab_view::TabViewBuilder::spawn(
        &mut commands,
        vec![
            ("Settings".into(), settings_column),
            ("Inspector".into(), inspector_column),
        ],
    );

    let column =
        widgets::ColumnBundle::spawn(&mut commands, vec![f3, divider_a, container_inner], ());

    let root = widgets::ContainerBundle::spawn(
        &mut commands,
        Some(column),
        (
            attachments::Root {
                enabled: true,
                position: IVec2 { x: 0, y: -1 },
                size: UVec2 { x: 32, y: 11 },
            },
            attachments::BorderBundle::new(Border::symmetric(
                Some('|'),
                Some('-'),
                Some([',', '.', '`', '\'']),
            )),
            attachments::RenderBundle::default(),
            DebugMenuMarker,
        ),
    );

    menu_state.root_widget = Some(root);
}

#[derive(Debug, Component)]
pub struct DebugMenuMarker;
