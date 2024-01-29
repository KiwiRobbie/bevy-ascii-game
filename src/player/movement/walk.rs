use bevy::{
    ecs::{component::Component, system::Query},
    math::Vec2,
};

use crate::{physics::movement::Movement, player::input::PlayerInputMovement};

use super::MovementFilter;

#[derive(Component, Debug, Default, Clone)]
pub struct PlayerWalkSpeed {
    pub speed: f32,
}
pub fn player_walk_system(
    mut q_player: Query<(&mut Movement, &PlayerInputMovement, &PlayerWalkSpeed), MovementFilter>,
) {
    for (mut movement, input, settings) in q_player.iter_mut() {
        let horizontal = if input.horizontal < -0.5 {
            -1.0
        } else if input.horizontal > 0.5 {
            1.0
        } else {
            0.0
        };

        movement.add(Vec2::X * horizontal * settings.speed);
    }
}
