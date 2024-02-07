use bevy::{
    ecs::component::Component,
    math::Vec2,
    prelude::{Deref, DerefMut},
    reflect::Reflect,
};

#[derive(Component, Default, Debug, Clone, Reflect, Deref, DerefMut)]
pub struct Remainder(pub Vec2);
