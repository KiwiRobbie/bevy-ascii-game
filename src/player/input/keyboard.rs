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

use super::{
    PlayerInputJump, PlayerInputLunge, PlayerInputMarker, PlayerInputMovement, PlayerInputReset,
};

#[derive(Debug, Component, Clone)]
pub struct PlayerInputKeyboardMarker;

fn player_keyboard_input_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut q_player_movement: Query<
        &mut PlayerInputMovement,
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

    let input_movement = PlayerInputMovement {
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
        if keyboard.pressed(KeyCode::KeyC) {
            commands.entity(entity).insert(PlayerInputJump);
        } else {
            commands.entity(entity).remove::<PlayerInputJump>();
        }

        if keyboard.pressed(KeyCode::KeyX) {
            commands.entity(entity).insert(PlayerInputLunge);
        } else {
            commands.entity(entity).remove::<PlayerInputLunge>();
        }

        if keyboard.pressed(KeyCode::KeyR) {
            commands.entity(entity).insert(PlayerInputReset);
        } else {
            commands.entity(entity).remove::<PlayerInputReset>();
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
