use bevy_ecs::{component::Component, entity::Entity};
use bevy_math::{IVec2, UVec2};

#[derive(Component, Clone)]
pub struct PhysicsGridMember {
    pub grid: Entity,
}

#[derive(Component, Clone)]
pub struct SpatialGrid {
    pub step: UVec2,
}
