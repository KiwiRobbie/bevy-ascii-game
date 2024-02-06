use bevy::{
    asset::Handle,
    ecs::{entity::Entity, system::Resource},
};
use bevy_ascii_game::tileset::asset::TilesetSource;

#[derive(Debug, Resource)]
pub struct TilesetPanelState {
    pub enabled: bool,
    pub root_widget: Option<Entity>,

    pub fps_text: Option<Entity>,
    pub player_count_text: Option<Entity>,
    pub actor_count_text: Option<Entity>,
    pub solid_count_text: Option<Entity>,
    pub position_checkbox: Option<Entity>,
    pub colliders_checkbox: Option<Entity>,
    pub ui_checkbox: Option<Entity>,
    pub pause_checkbox: Option<Entity>,

    pub tilesets: Vec<Handle<TilesetSource>>,
}

impl Default for TilesetPanelState {
    fn default() -> Self {
        TilesetPanelState {
            enabled: true,
            root_widget: None,

            fps_text: None,
            player_count_text: None,
            actor_count_text: None,
            solid_count_text: None,

            colliders_checkbox: None,
            position_checkbox: None,
            ui_checkbox: None,
            pause_checkbox: None,

            tilesets: vec![],
        }
    }
}
