use bevy::{
    app::{Plugin, PreUpdate},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    input::gamepad::GamepadButton,
    prelude::{Gamepad, IntoSystemConfigs},
};

use crate::player::PlayerMarker;

use super::{player_inputs, PlayerInputMarker, PlayerInputSet};

#[derive(Debug, Component)]
pub struct PlayerInputController(pub Entity);

fn player_controller_input_movement(
    q_gamepads: Query<&Gamepad>,
    mut q_player_movement: Query<
        (&mut player_inputs::Movement, &PlayerInputController),
        (With<PlayerMarker>, With<PlayerInputMarker>),
    >,
) {
    for (mut movement, PlayerInputController(gamepad_entity)) in q_player_movement.iter_mut() {
        let Ok(gamepad) = q_gamepads.get(*gamepad_entity) else {
            continue;
        };
        let horizontal = gamepad.left_stick().x;
        let vertical = gamepad.left_stick().y;

        *movement = player_inputs::Movement {
            horizontal,
            vertical,
        };
    }
}

fn player_controller_input_buttons(
    q_gamepads: Query<&Gamepad>,
    mut commands: Commands,
    q_players: Query<
        (Entity, &PlayerInputController),
        (With<PlayerMarker>, With<PlayerInputMarker>),
    >,
) {
    for (entity, PlayerInputController(gamepad_entity)) in q_players.iter() {
        let Ok(gamepad) = q_gamepads.get(*gamepad_entity) else {
            continue;
        };
        commands
            .entity(entity)
            .remove::<player_inputs::MarkerResetBundle>();

        if gamepad.pressed(GamepadButton::South) {
            commands.entity(entity).insert(player_inputs::JumpMarker);
        }

        if gamepad.pressed(GamepadButton::West) {
            commands.entity(entity).insert(player_inputs::LungeMarker);
        }

        if gamepad.pressed(GamepadButton::Start) {
            commands.entity(entity).insert(player_inputs::ResetMarker);
        }
    }
}
pub(crate) struct PlayerControllerInputPlugin;

impl Plugin for PlayerControllerInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PreUpdate,
            (
                player_controller_input_movement,
                player_controller_input_buttons,
            )
                .in_set(PlayerInputSet),
        );
    }
}
