use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Debug, Resource)]
pub(crate) struct DebugMenuState {
    pub(crate) enabled: bool,
    pub(crate) root_widget: Option<Entity>,
}

impl Default for DebugMenuState {
    fn default() -> Self {
        DebugMenuState {
            enabled: true,
            root_widget: None,
        }
    }
}
