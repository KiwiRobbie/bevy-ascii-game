use bevy::{
    ecs::{component::Component, query::With, system::Query},
    math::Vec2,
};

use crate::{
    physics::movement::Movement,
    player::{input::keyboard::PlayerInputMovement, PlayerMarker},
};

use super::PlayerMovementMarker;

#[derive(Component, Debug, Default)]
pub struct PlayerWalkSpeed {
    pub speed: f32,
}

type MovementFilter = (With<PlayerMarker>, With<PlayerMovementMarker>);
pub fn player_walk_system(
    mut q_player: Query<(&mut Movement, &PlayerInputMovement, &PlayerWalkSpeed), MovementFilter>,
) {
    for (mut movement, input, settings) in q_player.iter_mut() {
        movement.add(Vec2::X * input.horizontal * settings.speed);
    }
}
