use bevy::{
    app::{Plugin, PreUpdate},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    input::{
        gamepad::{Gamepad, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType},
        Axis, ButtonInput,
    },
};

use crate::player::PlayerMarker;

use super::{player_inputs, PlayerInputMarker};

#[derive(Debug, Component)]
pub struct PlayerInputController(pub Gamepad);

fn player_controller_input_movement(
    axis: Res<Axis<GamepadAxis>>,
    mut q_player_movement: Query<
        (&mut player_inputs::Movement, &PlayerInputController),
        (With<PlayerMarker>, With<PlayerInputMarker>),
    >,
) {
    for (mut movement, PlayerInputController(gamepad)) in q_player_movement.iter_mut() {
        let horizontal = axis
            .get(GamepadAxis {
                gamepad: *gamepad,
                axis_type: GamepadAxisType::LeftStickX,
            })
            .unwrap_or(0.0);
        let vertical = axis
            .get(GamepadAxis {
                gamepad: *gamepad,
                axis_type: GamepadAxisType::LeftStickY,
            })
            .unwrap_or(0.0);

        *movement = player_inputs::Movement {
            horizontal,
            vertical,
        };
    }
}

fn player_controller_input_buttons(
    mut commands: Commands,
    buttons: Res<ButtonInput<GamepadButton>>,
    q_players: Query<
        (Entity, &PlayerInputController),
        (With<PlayerMarker>, With<PlayerInputMarker>),
    >,
) {
    for (entity, PlayerInputController(gamepad)) in q_players.iter() {
        commands
            .entity(entity)
            .remove::<player_inputs::MarkerResetBundle>();

        if buttons.pressed(GamepadButton {
            gamepad: *gamepad,
            button_type: GamepadButtonType::South,
        }) {
            commands.entity(entity).insert(player_inputs::JumpMarker);
        }

        if buttons.pressed(GamepadButton {
            gamepad: *gamepad,
            button_type: GamepadButtonType::West,
        }) {
            commands.entity(entity).insert(player_inputs::LungeMarker);
        }

        if buttons.pressed(GamepadButton {
            gamepad: *gamepad,
            button_type: GamepadButtonType::Start,
        }) {
            commands.entity(entity).insert(player_inputs::ResetMarker);
        }
    }
}
pub struct PlayerControllerInputPlugin;

impl Plugin for PlayerControllerInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PreUpdate,
            (
                player_controller_input_movement,
                player_controller_input_buttons,
            ),
        );
    }
}
