use bevy::{
    ecs::component::Component,
    math::{IVec2, UVec2},
};

#[derive(Component, Clone, Debug)]
pub struct Positioned {
    pub offset: IVec2,
    pub size: UVec2,
}
