use bevy::prelude::*;

use crate::player::input::player_inputs::JumpMarker;
use grid_physics::{
    free::{FreeGrounded, FreeMarker},
    velocity::Velocity,
};

use super::MovementFilter;

#[derive(Component, Debug, Clone, Reflect)]
pub(crate) struct PlayerJumpVelocity {
    pub(crate) velocity: f32,
}

impl Default for PlayerJumpVelocity {
    fn default() -> Self {
        Self { velocity: 50.0 }
    }
}

pub(crate) fn player_jump_system(
    mut commands: Commands,
    mut q_player: Query<
        (Entity, &mut Velocity, &PlayerJumpVelocity),
        (
            MovementFilter,
            With<JumpMarker>,
            With<FreeGrounded>,
            With<FreeMarker>,
        ),
    >,
) {
    for (entity, mut velocity, jump_velocity) in q_player.iter_mut() {
        velocity.y = jump_velocity.velocity;

        commands.entity(entity).insert(FreeMarker);
    }
}
