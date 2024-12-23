use super::{direction::PlayerDirection, MovementFilter};
use crate::player::input::player_inputs;
use bevy::{
    ecs::{component::Component, system::Query},
    prelude::Has,
};
use grid_physics::{free::FreeGrounded, velocity::Velocity};

#[derive(Component, Debug, Default, Clone)]
pub struct PlayerWalkSpeed {
    pub speed: f32,
}
pub fn player_walk_system(
    mut q_player: Query<
        (
            // &mut Movement,
            &mut Velocity,
            &mut PlayerDirection,
            &player_inputs::Movement,
            &PlayerWalkSpeed,
            Has<FreeGrounded>,
        ),
        MovementFilter,
    >,
) {
    for (mut velocity, mut direction, input, settings, grounded) in q_player.iter_mut() {
        let horizontal = if input.horizontal < -0.5 {
            direction.set_x(-1);
            -1.0
        } else if input.horizontal > 0.5 {
            direction.set_x(1);
            1.0
        } else {
            0.0
        };

        if grounded {
            velocity.x = horizontal * settings.speed;
            velocity.y = 0.;
        } else {
        }
    }
}
