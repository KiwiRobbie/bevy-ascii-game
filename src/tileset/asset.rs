use bevy::{asset::Asset, math::UVec2, reflect::TypePath, utils::HashMap};

#[derive(Debug, Asset, TypePath, Clone)]
pub struct TilesetSource {
    pub display_name: String,
    pub id: String,
    pub tile_size: UVec2,
    pub tile_ids: HashMap<String, usize>,
    pub tile_labels: Vec<String>,
    pub tiles: Vec<Vec<String>>,
}
