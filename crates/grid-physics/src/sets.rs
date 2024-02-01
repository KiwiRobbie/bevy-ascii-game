use bevy::{
    ecs::{
        schedule::SystemSet,
        system::{Res, Resource},
    },
    prelude::{Deref, DerefMut},
};
#[derive(Debug, SystemSet, PartialEq, Eq, Hash, Clone)]
pub struct PhysicsPostUpdateSet;

pub fn physics_systems_enabled(enabled: Res<EnablePhysicsSystems>) -> bool {
    **enabled
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct EnablePhysicsSystems(pub bool);

impl Default for EnablePhysicsSystems {
    fn default() -> Self {
        Self(true)
    }
}
