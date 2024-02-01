use bevy::{ecs::component::Component, math::Vec2, reflect::Reflect};

#[derive(Component, Default, Clone, Reflect)]
pub struct Velocity {
    pub velocity: Vec2,
}
