use bevy::{
    app::{Plugin, Update},
    ecs::{bundle::Bundle, component::Component, schedule::IntoSystemSetConfigs},
};

use crate::physics::{
    actor::ActorPhysicsBundle,
    position::{Position, PositionBundle},
};

use self::{
    input::{PlayerInputBundle, PlayerInputPlugin},
    movement::{PlayerMovementBundle, PlayerMovementPlugin},
    system_sets::{PlayerPostUpdate, PlayerPrepare, PlayerUpdate},
};

pub mod input;
pub mod movement;
pub mod system_sets;

#[derive(Component, Default)]
pub struct PlayerMarker;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_sets(
            Update,
            (PlayerPrepare, PlayerUpdate, PlayerPostUpdate).chain(),
        )
        .add_plugins((PlayerInputPlugin, PlayerMovementPlugin));
    }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub marker: PlayerMarker,
    pub input: PlayerInputBundle,
    pub movement: PlayerMovementBundle,
    pub actor: ActorPhysicsBundle,
}
