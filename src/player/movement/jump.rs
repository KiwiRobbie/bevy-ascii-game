use crate::player::input::player_inputs::JumpMarker;
use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    reflect::Reflect,
};
use grid_physics::{
    free::{FreeGrounded, FreeMarker},
    velocity::Velocity,
};

use super::MovementFilter;

#[derive(Component, Debug, Clone, Reflect)]
pub struct PlayerJumpVelocity {
    pub velocity: f32,
}

impl Default for PlayerJumpVelocity {
    fn default() -> Self {
        Self { velocity: 50.0 }
    }
}

pub fn player_jump_system(
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
