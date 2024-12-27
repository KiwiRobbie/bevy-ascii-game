use bevy::prelude::*;

use self::{
    animation::set_animation_target,
    input::{PlayerInputBundle, PlayerInputPlugin},
    movement::{PlayerMovementBundle, PlayerMovementPlugin},
    reset::player_reset_system,
};
use grid_physics::actor::ActorPhysicsBundle;
use interaction::{InteractionSource, PlayerInteractionPlugin};

pub(crate) mod animation;
pub mod input;
pub mod interaction;
pub(crate) mod movement;
pub mod reset;
pub(crate) mod sword;

#[derive(Component, Default, Clone)]
pub(crate) struct PlayerMarker;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            PlayerInputPlugin,
            PlayerMovementPlugin,
            PlayerInteractionPlugin,
        ))
        .add_systems(Update, (player_reset_system, set_animation_target));
    }
}

#[derive(Bundle, Default, Clone)]
pub(crate) struct PlayerBundle {
    pub(crate) marker: PlayerMarker,
    pub(crate) input: PlayerInputBundle,
    pub(crate) movement: PlayerMovementBundle,
    pub(crate) actor: ActorPhysicsBundle,
    pub(crate) interaction_source: InteractionSource,
}
