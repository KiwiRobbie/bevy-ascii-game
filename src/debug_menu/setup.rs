use ascii_ui::{
    attachments::{self, border::Border},
    widgets,
};
use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res, ResMut},
    math::{IVec2, UVec2},
    prelude::Entity,
};

use super::state::DebugMenuState;

pub fn setup_ui(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut menu_state: ResMut<DebugMenuState>,
) {
    let font = &server.load("FiraCode-Regular.ttf");

    let player_count: Entity = widgets::TextBundle::spawn(&mut commands, "Text A".into(), font, ());
    let solid_count: Entity = widgets::TextBundle::spawn(&mut commands, "Text B".into(), font, ());
    let actor_count: Entity = widgets::TextBundle::spawn(&mut commands, "Text c".into(), font, ());

    menu_state.player_count_text = Some(player_count);
    menu_state.solid_count_text = Some(solid_count);
    menu_state.actor_count_text = Some(actor_count);

    let divider: Entity = widgets::DividerBundle::spawn(&mut commands, '=', font);

    let debug_position =
        widgets::CheckboxBuilder::spawn(&mut commands, "Debug Position".into(), font);
    let debug_colliders =
        widgets::CheckboxBuilder::spawn(&mut commands, "Debug Colliders".into(), font);
    let debug_ui = widgets::CheckboxBuilder::spawn(&mut commands, "Debug ECS UI".into(), font);

    menu_state.position_checkbox = Some(debug_position);
    menu_state.colliders_checkbox = Some(debug_colliders);
    menu_state.ui_checkbox = Some(debug_ui);

    let column = widgets::ColumnBundle::spawn(
        &mut commands,
        vec![
            player_count,
            solid_count,
            actor_count,
            divider,
            debug_position,
            debug_colliders,
            debug_ui,
        ],
        (),
    );

    let root = widgets::ContainerBundle::spawn(
        &mut commands,
        Some(column),
        (
            attachments::Root {
                enabled: false,
                position: IVec2::ZERO,
                size: UVec2 { x: 30, y: 10 },
            },
            attachments::BorderBundle::new(Border::symmetric(
                Some('|'),
                Some('-'),
                Some([',', '.', '`', '\'']),
            )),
            attachments::RenderBundle::from_font(font),
        ),
    );

    menu_state.root_widget = Some(root);
}
