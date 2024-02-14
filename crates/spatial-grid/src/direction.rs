use bevy_ecs::component::Component;

#[derive(Debug, Component, Clone, Copy)]
pub enum Direction {
    X,
    Y,
    NegX,
    NegY,
}
