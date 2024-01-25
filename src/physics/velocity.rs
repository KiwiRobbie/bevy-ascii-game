use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component, Default)]
pub struct Velocity {
    pub velocity: Vec2,
}
