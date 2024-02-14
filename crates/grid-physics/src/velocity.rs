use bevy_derive::{Deref, DerefMut};
use bevy_ecs::component::Component;
use bevy_math::Vec2;
use bevy_reflect::Reflect;

#[derive(Component, Default, Clone, Reflect, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2 { x, y })
    }
}
