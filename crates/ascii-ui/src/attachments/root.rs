use bevy::{
    ecs::{component::Component, reflect::ReflectComponent},
    math::{IVec2, UVec2},
    reflect::Reflect,
};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Root {
    pub position: IVec2,
    pub size: UVec2,
}
