use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Debug, Resource)]
pub(super) struct TilesetPanelState {
    pub(crate) enabled: bool,
    pub(crate) root_widget: Option<Entity>,
    pub(crate) _save_button: Option<Entity>,
}

impl Default for TilesetPanelState {
    fn default() -> Self {
        TilesetPanelState {
            enabled: true,
            root_widget: None,
            _save_button: None,
        }
    }
}
