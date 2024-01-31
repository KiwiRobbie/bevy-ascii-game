use bevy::{
    app::{Plugin, Update},
    ecs::{bundle::Bundle, component::Component, query::With, schedule::IntoSystemConfigs},
};

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

pub mod direction;
pub mod jump;
pub mod lunge;
pub mod walk;

pub struct PlayerMovementPlugin;
impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                player_walk_system,
                player_jump_system,
                player_lunge_start_system,
                player_lunge_update_system,
                player_lunge_cooldown_update,
                player_update_sprite_mirror,
            )
                .chain(),
        );
    }
}

#[derive(Component, Default, Clone)]
pub struct PlayerMovementMarker;

#[derive(Bundle, Default, Clone)]
pub struct PlayerMovementBundle {
    pub marker: PlayerMovementMarker,
    pub walk_speed: PlayerWalkSpeed,
    pub jump_velocity: PlayerJumpVelocity,
    pub lunge_settings: PlayerLungeSettings,
    pub direction: PlayerDirection,
}
type MovementFilter = (With<PlayerMarker>, With<PlayerMovementMarker>);
