use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Debug, Resource)]
pub(super) struct EditorPanelState {
    pub(crate) enabled: bool,
    pub(crate) root_widget: Option<Entity>,
}

impl Default for EditorPanelState {
    fn default() -> Self {
        EditorPanelState {
            enabled: true,
            root_widget: None,
        }
    }
}
