use bevy::{
    app::{Plugin, PreUpdate},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::With,
        system::{Commands, Query, Res, Resource},
    },
    input::{
        gamepad::{
            Gamepad, GamepadAxis, GamepadAxisChangedEvent, GamepadAxisType, GamepadButton,
            GamepadButtonChangedEvent, GamepadButtonType, GamepadEvent, Gamepads,
        },
        keyboard::KeyCode,
        Axis, Input,
    },
};

use crate::player::PlayerMarker;

use super::{PlayerInputJump, PlayerInputMarker, PlayerInputMovement};

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

fn player_controller_input_jump(
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
    }
}
pub struct PlayerControllerInputPlugin;

impl Plugin for PlayerControllerInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PreUpdate,
            (
                player_controller_input_movement,
                player_controller_input_jump,
            ),
        );
    }
}
