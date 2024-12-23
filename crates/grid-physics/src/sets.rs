use bevy_derive::{Deref, DerefMut};
use bevy_ecs::system::{Res, Resource};

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
