use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Debug, Resource)]
pub struct DebugMenuState {
    pub enabled: bool,
    pub root_widget: Option<Entity>,
}

impl Default for DebugMenuState {
    fn default() -> Self {
        DebugMenuState {
            enabled: true,
            root_widget: None,
        }
    }
}
