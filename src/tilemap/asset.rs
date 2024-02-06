use bevy::{
    asset::{Asset, Handle},
    math::UVec2,
    reflect::TypePath,
    utils::HashMap,
};

use crate::tileset::asset::TilesetSource;

use super::chunk::TilemapChunk;

#[derive(Debug, Asset, TypePath)]
pub struct TilemapSource {
    pub chunk_size: UVec2,
    pub tileset_names: HashMap<String, usize>,
    pub tilesets: Vec<Handle<TilesetSource>>,
    pub chunk_data: HashMap<UVec2, TilemapChunk>,
}
