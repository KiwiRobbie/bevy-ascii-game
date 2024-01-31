use bevy::{
    ecs::component::Component,
    math::{IVec2, UVec2},
};

#[derive(Component)]
pub struct Root {
    pub position: IVec2,
    pub size: UVec2,
}
