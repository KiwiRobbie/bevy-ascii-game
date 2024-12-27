use bevy::prelude::*;

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
