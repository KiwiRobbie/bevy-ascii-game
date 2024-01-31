use bevy::{
    app::{Plugin, Update},
    ecs::{bundle::Bundle, component::Component},
};

use grid_physics::actor::ActorPhysicsBundle;

use self::{
    animation::set_animation_target,
    input::{PlayerInputBundle, PlayerInputPlugin},
    movement::{PlayerMovementBundle, PlayerMovementPlugin},
    reset::player_reset_system,
};

pub mod animation;
pub mod input;
pub mod movement;
pub mod reset;

#[derive(Component, Default, Clone)]
pub struct PlayerMarker;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((PlayerInputPlugin, PlayerMovementPlugin))
            .add_systems(Update, (player_reset_system, set_animation_target));
    }
}

#[derive(Bundle, Default, Clone)]
pub struct PlayerBundle {
    pub marker: PlayerMarker,
    pub input: PlayerInputBundle,
    pub movement: PlayerMovementBundle,
    pub actor: ActorPhysicsBundle,
}
