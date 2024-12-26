use bevy_derive::{Deref, DerefMut};
use bevy_ecs::component::Component;
use bevy_ecs::prelude::ReflectComponent;
use bevy_math::Vec2;
use bevy_reflect::Reflect;

#[derive(Component, Default, Debug, Clone, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct Remainder(pub Vec2);
impl From<Vec2> for Remainder {
    fn from(value: Vec2) -> Self {
        Self(value.fract())
    }
}
