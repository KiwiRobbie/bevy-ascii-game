use bevy::{asset::Asset, math::UVec2, reflect::TypePath, utils::HashMap};

#[derive(Debug, Asset, TypePath)]
pub struct TilesetSource {
    pub display_name: String,
    pub id: String,
    pub tile_size: UVec2,
    pub tile_names: HashMap<String, usize>,
    pub tiles: Vec<Vec<String>>,
}
