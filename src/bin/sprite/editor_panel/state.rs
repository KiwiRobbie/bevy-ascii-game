use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Debug, Resource)]
pub(super) struct EditorPanelState {
    pub(crate) enabled: bool,
    pub(crate) root_widget: Option<Entity>,
    pub(crate) isolate_selected: bool,
}

impl Default for EditorPanelState {
    fn default() -> Self {
        EditorPanelState {
            enabled: true,
            root_widget: None,
            isolate_selected: false,
        }
    }
}
