use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Debug, Resource)]
pub(super) struct TilesetPanelState {
    pub enabled: bool,
    pub root_widget: Option<Entity>,
    pub save_button: Option<Entity>,
}

impl Default for TilesetPanelState {
    fn default() -> Self {
        TilesetPanelState {
            enabled: true,
            root_widget: None,
            save_button: None,
        }
    }
}
