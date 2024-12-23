use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    schedule::SystemSet,
    system::{Res, Resource},
};

#[derive(Debug, SystemSet, PartialEq, Eq, Hash, Clone)]
pub(crate) struct PhysicsPostUpdateSet;

pub fn physics_systems_enabled(enabled: Res<EnablePhysicsSystems>) -> bool {
    **enabled
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct EnablePhysicsSystems(pub(crate) bool);

impl Default for EnablePhysicsSystems {
    fn default() -> Self {
        Self(true)
    }
}
