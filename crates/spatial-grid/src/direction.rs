use bevy_ecs::component::Component;

#[derive(Debug, Component, Clone, Copy)]
pub enum Direction {
    PosX,
    PosY,
    NegX,
    NegY,
}
