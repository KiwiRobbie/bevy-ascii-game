use bevy::ecs::{
    component::Component,
    entity::Entity,
    query::With,
    system::{Commands, Query},
};

use crate::{
    physics::{free::FreeMarker, velocity::Velocity},
    player::input::PlayerInputJump,
};

use super::MovementFilter;

#[derive(Component, Debug)]
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
        (MovementFilter, With<PlayerInputJump>),
    >,
) {
    for (entity, mut velocity, jump_velocity) in q_player.iter_mut() {
        velocity.velocity.y = jump_velocity.velocity;

        commands.entity(entity).insert(FreeMarker);
    }
}
