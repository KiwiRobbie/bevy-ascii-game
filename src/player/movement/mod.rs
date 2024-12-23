use bevy::{
    app::{Plugin, Update},
    ecs::{bundle::Bundle, component::Component, query::With, schedule::IntoSystemConfigs},
};
use grid_physics::sets::physics_systems_enabled;

use self::{
    direction::{player_update_sprite_mirror, PlayerDirection},
    jump::{player_jump_system, PlayerJumpVelocity},
    lunge::{
        player_lunge_cooldown_update, player_lunge_start_system, player_lunge_update_system,
        PlayerLungeSettings,
    },
    walk::{player_walk_system, PlayerWalkSpeed},
};

use super::PlayerMarker;

pub(crate) mod direction;
pub(crate) mod jump;
pub(crate) mod lunge;
pub(crate) mod walk;

pub(crate) struct PlayerMovementPlugin;
impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            ((
                player_walk_system,
                player_jump_system,
                player_lunge_start_system,
                player_lunge_update_system,
                player_lunge_cooldown_update,
                player_update_sprite_mirror,
            )
                .chain()
                .run_if(physics_systems_enabled),)
                .chain(),
        );
    }
}

#[derive(Component, Default, Clone)]
pub(crate) struct PlayerMovementMarker;

#[derive(Bundle, Default, Clone)]
pub(crate) struct PlayerMovementBundle {
    pub(crate) marker: PlayerMovementMarker,
    pub(crate) walk_speed: PlayerWalkSpeed,
    pub(crate) jump_velocity: PlayerJumpVelocity,
    pub(crate) lunge_settings: PlayerLungeSettings,
    pub(crate) direction: PlayerDirection,
}
type MovementFilter = (With<PlayerMarker>, With<PlayerMovementMarker>);
