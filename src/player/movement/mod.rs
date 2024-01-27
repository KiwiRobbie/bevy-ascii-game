use bevy::{
    app::{Plugin, Update},
    ecs::{bundle::Bundle, component::Component, schedule::IntoSystemConfigs},
};

use self::walk::{player_walk_system, PlayerWalkSpeed};

use super::system_sets::PlayerUpdate;

pub mod walk;

pub struct PlayerMovementPlugin;
impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (player_walk_system).in_set(PlayerUpdate));
    }
}

#[derive(Component, Default)]
pub struct PlayerMovementMarker;

#[derive(Bundle, Default)]
pub struct PlayerMovementBundle {
    pub marker: PlayerMovementMarker,
    pub walk_speed: PlayerWalkSpeed,
}
