use bevy::{
    asset::Handle,
    ecs::{entity::Entity, system::Resource},
};
use bevy_ascii_game::tileset::asset::TilesetSource;

#[derive(Debug, Resource)]
pub struct TilesetPanelState {
    pub enabled: bool,
    pub root_widget: Option<Entity>,

    pub tilesets: Vec<Handle<TilesetSource>>,
}

impl Default for TilesetPanelState {
    fn default() -> Self {
        TilesetPanelState {
            enabled: true,
            root_widget: None,
            tilesets: vec![],
        }
    }
}
