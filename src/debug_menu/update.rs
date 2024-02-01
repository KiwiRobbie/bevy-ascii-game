use ascii_ui::{
    attachments::Root,
    widgets::{
        self,
        checkbox::{Checkbox, CheckboxEnabledMarker},
    },
};
use bevy::{
    ecs::{
        query::With,
        system::{Query, Res, ResMut},
    },
    input::{
        gamepad::{GamepadButton, GamepadButtonType, Gamepads},
        keyboard::KeyCode,
        Input,
    },
};
use grid_physics::{
    actor::Actor,
    debug::{DebugCollisions, DebugPositions},
    solid::Solid,
};

use crate::player::PlayerMarker;

use super::state::DebugMenuState;

pub fn toggle_menu(
    keyboard: Res<Input<KeyCode>>,
    mut state: ResMut<DebugMenuState>,
    gamepad_button: Res<Input<GamepadButton>>,
    gamepads: Res<Gamepads>,
    mut q_root: Query<&mut Root>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        state.enabled = !state.enabled;
    }
    for gamepad in gamepads.iter() {
        if gamepad_button.just_pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Select,
        }) {
            state.enabled = !state.enabled;
        }
    }

    if let Some(root) = state.root_widget {
        let mut root = q_root.get_mut(root).unwrap();
        root.enabled = state.enabled;
    }
}

pub fn update(
    state: Res<DebugMenuState>,
    mut collisions: ResMut<DebugCollisions>,
    mut positions: ResMut<DebugPositions>,
    mut q_text: Query<&mut widgets::text::Text>,
    q_checkbox: Query<Option<&CheckboxEnabledMarker>, With<Checkbox>>,
    q_player: Query<(), With<PlayerMarker>>,
    q_solid: Query<(), With<Solid>>,
    q_actor: Query<(), With<Actor>>,
) {
    if !state.enabled {
        return;
    }

    if let Some(entity) = state.colliders_checkbox {
        let state = q_checkbox.get(entity).unwrap().is_some();
        **collisions = state;
    }
    if let Some(entity) = state.position_checkbox {
        let state = q_checkbox.get(entity).unwrap().is_some();
        **positions = state;
    }

    if let Some(entity) = state.player_count_text {
        q_text.get_mut(entity).unwrap().text = format!("Player Count: {}", q_player.iter().count());
    }
    if let Some(entity) = state.solid_count_text {
        q_text.get_mut(entity).unwrap().text = format!("Solid  Count: {}", q_solid.iter().count());
    }
    if let Some(entity) = state.actor_count_text {
        q_text.get_mut(entity).unwrap().text = format!("Actor  Count: {}", q_actor.iter().count());
    }
}
