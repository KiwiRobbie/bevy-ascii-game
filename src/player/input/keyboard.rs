use bevy::{
    app::{Plugin, PreUpdate},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, Input},
};

use crate::player::PlayerMarker;

use super::{PlayerInputJump, PlayerInputMarker};

#[derive(Debug, Default, Component, Clone)]
pub struct PlayerInputMovement {
    pub horizontal: f32,
    pub vertical: f32,
}

fn player_keyboard_input_movement(
    keyboard: Res<Input<KeyCode>>,
    mut q_player_movement: Query<
        &mut PlayerInputMovement,
        (With<PlayerMarker>, With<PlayerInputMarker>),
    >,
) {
    let mut horizontal = 0.0;
    let mut vertical = 0.0;

    if keyboard.pressed(KeyCode::D) {
        horizontal += 1.0;
    }
    if keyboard.pressed(KeyCode::A) {
        horizontal -= 1.0;
    }
    if keyboard.pressed(KeyCode::W) {
        vertical += 1.0;
    }
    if keyboard.pressed(KeyCode::S) {
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

fn player_keyboard_input_jump(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    q_jump_inputs: Query<Entity, With<PlayerInputJump>>,
    q_players: Query<Entity, (With<PlayerMarker>, With<PlayerInputMarker>)>,
) {
    for entity in q_players.iter() {
        commands.entity(entity).remove::<PlayerInputJump>();
    }
    if keyboard.pressed(KeyCode::Space) {
        for entity in q_players.iter() {
            commands.entity(entity).insert(PlayerInputJump);
        }
    }
}
pub struct PlayerKeyboardInputPlugin;

impl Plugin for PlayerKeyboardInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PreUpdate,
            (player_keyboard_input_movement, player_keyboard_input_jump),
        );
    }
}
