use super::{direction::PlayerDirection, MovementFilter};
use crate::player::input::PlayerInputMovement;
use bevy::{
    ecs::{
        component::Component,
        system::{Query, Res},
    },
    math::Vec2,
    time::Time,
};
use grid_physics::{free::FreeGrounded, movement::Movement, velocity::Velocity};

#[derive(Component, Debug, Default, Clone)]
pub struct PlayerWalkSpeed {
    pub speed: f32,
}
pub fn player_walk_system(
    mut q_player: Query<
        (
            &mut Movement,
            &mut Velocity,
            &mut PlayerDirection,
            &PlayerInputMovement,
            &PlayerWalkSpeed,
            Option<&FreeGrounded>,
        ),
        MovementFilter,
    >,
    time: Res<Time>,
) {
    for (mut movement, mut velocity, mut direction, input, settings, grounded) in
        q_player.iter_mut()
    {
        let horizontal = if input.horizontal < -0.5 {
            direction.set_x(-1);
            -1.0
        } else if input.horizontal > 0.5 {
            direction.set_x(1);
            1.0
        } else {
            0.0
        };

        movement.add(Vec2::X * horizontal * settings.speed * time.delta_seconds());
        if grounded.is_some() {
            **velocity = Vec2::ZERO;
        }
    }
}
