use bevy_derive::{Deref, DerefMut};
use bevy_ecs::component::Component;
use bevy_math::Vec2;
use bevy_reflect::Reflect;

#[derive(Component, Default, Debug, Clone, Reflect, Deref, DerefMut)]
pub struct Remainder(pub Vec2);
