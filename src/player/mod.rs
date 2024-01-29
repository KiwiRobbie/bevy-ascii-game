use bevy::{
    app::Plugin,
    ecs::{bundle::Bundle, component::Component},
};

use crate::physics::actor::ActorPhysicsBundle;

use self::{
    input::{PlayerInputBundle, PlayerInputPlugin},
    movement::{PlayerMovementBundle, PlayerMovementPlugin},
};

pub mod animation;
pub mod input;
pub mod movement;

#[derive(Component, Default, Clone)]
pub struct PlayerMarker;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((PlayerInputPlugin, PlayerMovementPlugin));
    }
}

#[derive(Bundle, Default, Clone)]
pub struct PlayerBundle {
    pub marker: PlayerMarker,
    pub input: PlayerInputBundle,
    pub movement: PlayerMovementBundle,
    pub actor: ActorPhysicsBundle,
}
