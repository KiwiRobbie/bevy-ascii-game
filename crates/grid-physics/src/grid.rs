use bevy::{
    ecs::{component::Component, entity::Entity},
    math::UVec2,
};

#[derive(Component, Clone)]
pub struct PhysicsGridMember {
    pub grid: Entity,
}

#[derive(Component, Clone)]
pub struct PhysicsGrid {
    pub size: UVec2,
}
