use bevy::{
    ecs::{component::Component, reflect::ReflectComponent},
    math::{IVec2, UVec2},
    reflect::Reflect,
};

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Root {
    pub enabled: bool,
    pub position: IVec2,
    pub size: UVec2,
}

impl Default for Root {
    fn default() -> Self {
        Self {
            enabled: true,
            position: Default::default(),
            size: Default::default(),
        }
    }
}
