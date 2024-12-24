use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Debug, Resource)]
pub(super) struct EditorPanelState {
    pub(crate) enabled: bool,
    pub(crate) root_widget: Option<Entity>,
    pub(crate) _save_button: Option<Entity>,
    pub(crate) tool_container: Option<Entity>,
}

impl Default for EditorPanelState {
    fn default() -> Self {
        EditorPanelState {
            enabled: true,
            root_widget: None,
            _save_button: None,
            tool_container: None,
        }
    }
}
