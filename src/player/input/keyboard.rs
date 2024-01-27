use bevy::{
    app::{Plugin, PreUpdate},
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res},
    },
    input::{keyboard::KeyCode, Input},
};

use crate::player::PlayerMarker;

use super::PlayerInputMarker;

#[derive(Debug, Default, Component, Clone)]
pub struct PlayerInputMovement {
    pub horizontal: f32,
    pub vertical: f32,
}

fn player_keyboard_movement_input(
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

pub struct PlayerKeyboardInputPlugin;

impl Plugin for PlayerKeyboardInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreUpdate, player_keyboard_movement_input);
    }
}
