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
        Axis, Input,
    },
};

use crate::player::PlayerMarker;

use super::{
    PlayerInputJump, PlayerInputLunge, PlayerInputMarker, PlayerInputMovement, PlayerInputReset,
};

#[derive(Debug, Component)]
pub struct PlayerInputController(pub Gamepad);

fn player_controller_input_movement(
    axis: Res<Axis<GamepadAxis>>,
    mut q_player_movement: Query<
        (&mut PlayerInputMovement, &PlayerInputController),
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

        *movement = PlayerInputMovement {
            horizontal,
            vertical,
        };
    }
}

fn player_controller_input_buttons(
    mut commands: Commands,
    buttons: Res<Input<GamepadButton>>,
    q_players: Query<
        (Entity, &PlayerInputController),
        (With<PlayerMarker>, With<PlayerInputMarker>),
    >,
) {
    for (entity, PlayerInputController(gamepad)) in q_players.iter() {
        if buttons.pressed(GamepadButton {
            gamepad: *gamepad,
            button_type: GamepadButtonType::South,
        }) {
            commands.entity(entity).insert(PlayerInputJump);
        } else {
            commands.entity(entity).remove::<PlayerInputJump>();
        }

        if buttons.pressed(GamepadButton {
            gamepad: *gamepad,
            button_type: GamepadButtonType::West,
        }) {
            commands.entity(entity).insert(PlayerInputLunge);
        } else {
            commands.entity(entity).remove::<PlayerInputLunge>();
        }

        if buttons.pressed(GamepadButton {
            gamepad: *gamepad,
            button_type: GamepadButtonType::Start,
        }) {
            commands.entity(entity).insert(PlayerInputReset);
        } else {
            commands.entity(entity).remove::<PlayerInputReset>();
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
