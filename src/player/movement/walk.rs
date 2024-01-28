use bevy::{
    ecs::{component::Component, system::Query},
    math::Vec2,
};

use crate::{physics::movement::Movement, player::input::keyboard::PlayerInputMovement};

use super::MovementFilter;

#[derive(Component, Debug, Default)]
pub struct PlayerWalkSpeed {
    pub speed: f32,
}
pub fn player_walk_system(
    mut q_player: Query<(&mut Movement, &PlayerInputMovement, &PlayerWalkSpeed), MovementFilter>,
) {
    for (mut movement, input, settings) in q_player.iter_mut() {
        movement.add(Vec2::X * input.horizontal * settings.speed);
    }
}
