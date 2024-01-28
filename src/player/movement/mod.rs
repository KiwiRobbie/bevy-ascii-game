use bevy::{
    app::{Plugin, Update},
    ecs::{bundle::Bundle, component::Component, query::With},
};

use self::{
    jump::{player_jump_system, PlayerJumpVelocity},
    walk::{player_walk_system, PlayerWalkSpeed},
};

use super::PlayerMarker;

pub mod jump;
pub mod walk;

pub struct PlayerMovementPlugin;
impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (player_walk_system, player_jump_system));
    }
}

#[derive(Component, Default)]
pub struct PlayerMovementMarker;

#[derive(Bundle, Default)]
pub struct PlayerMovementBundle {
    pub marker: PlayerMovementMarker,
    pub walk_speed: PlayerWalkSpeed,
    pub jump_velocity: PlayerJumpVelocity,
}
type MovementFilter = (With<PlayerMarker>, With<PlayerMovementMarker>);
