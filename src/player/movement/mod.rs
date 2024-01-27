use bevy::{
    app::{Plugin, Update},
    ecs::{bundle::Bundle, component::Component},
};

use self::walk::{player_walk_system, PlayerWalkSpeed};

pub mod walk;

pub struct PlayerMovementPlugin;
impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, player_walk_system);
    }
}

#[derive(Component, Default)]
pub struct PlayerMovementMarker;

#[derive(Bundle, Default)]
pub struct PlayerMovementBundle {
    pub marker: PlayerMovementMarker,
    pub walk_speed: PlayerWalkSpeed,
}
