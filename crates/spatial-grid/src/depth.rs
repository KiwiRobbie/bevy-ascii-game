use bevy_derive::{Deref, DerefMut};
use bevy_ecs::component::Component;

#[derive(Debug, Default, Component, Clone, Deref, DerefMut)]
pub struct Depth(pub f32);
