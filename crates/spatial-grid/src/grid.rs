use bevy_ecs::{component::Component, entity::Entity};
use bevy_math::UVec2;

#[derive(Component, Clone)]
pub struct PhysicsGridMember {
    pub grid: Entity,
}

#[derive(Component, Clone)]
pub struct SpatialGrid {
    pub size: UVec2,
}
