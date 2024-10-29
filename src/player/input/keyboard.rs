use bevy::{
    app::{Plugin, PreUpdate},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, ButtonInput},
};

use crate::player::PlayerMarker;

use super::{player_inputs, PlayerInputMarker};

#[derive(Debug, Component, Clone)]
pub struct PlayerInputKeyboardMarker;

fn player_keyboard_input_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut q_player_movement: Query<
        &mut player_inputs::Movement,
        (
            With<PlayerMarker>,
            With<PlayerInputMarker>,
            With<PlayerInputKeyboardMarker>,
        ),
    >,
) {
    let mut horizontal = 0.0;
    let mut vertical = 0.0;

    if keyboard.pressed(KeyCode::ArrowRight) {
        horizontal += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        horizontal -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        vertical += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        vertical -= 1.0;
    }

    let input_movement = player_inputs::Movement {
        horizontal,
        vertical,
    };

    for mut movement in q_player_movement.iter_mut() {
        *movement = input_movement.clone();
    }
}

fn player_keyboard_input_buttons(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    q_players: Query<
        Entity,
        (
            With<PlayerMarker>,
            With<PlayerInputMarker>,
            With<PlayerInputKeyboardMarker>,
        ),
    >,
) {
    for entity in q_players.iter() {
        let mut commands = commands.entity(entity);

        commands.remove::<player_inputs::MarkerResetBundle>();

        if keyboard.pressed(KeyCode::KeyC) {
            commands.insert(player_inputs::JumpMarker);
        }
        if keyboard.pressed(KeyCode::KeyX) {
            commands.insert(player_inputs::LungeMarker);
        }
        if keyboard.pressed(KeyCode::KeyR) {
            commands.insert(player_inputs::ResetMarker);
        }
        if keyboard.pressed(KeyCode::KeyF) {
            commands.insert(player_inputs::InteractMarker);
        }
    }
}
pub struct PlayerKeyboardInputPlugin;

impl Plugin for PlayerKeyboardInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PreUpdate,
            (
                player_keyboard_input_movement,
                player_keyboard_input_buttons,
            ),
        );
    }
}
