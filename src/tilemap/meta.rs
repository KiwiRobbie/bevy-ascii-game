use bevy::{asset::Asset, prelude::Deref, reflect::TypePath};

#[derive(serde::Deserialize, serde::Serialize, Asset, TypePath, Clone)]
pub struct TilemapMeta {
    pub chunk_size: (u32, u32),
    pub tile_size: (u32, u32),
    pub tilesets: Vec<String>,
    pub chunk_dir: String,
    pub chunks: Vec<(i32, i32)>,
}

// #[derive(serde::Deserialize, Asset, TypePath, Clone)]
// pub enum ChunkDataLocation {
//     Relative(String),
//     Dir(String),
// }

#[derive(serde::Deserialize, Asset, TypePath, Clone, Deref, serde::Serialize)]
pub struct ChunkMeta(pub Vec<Vec<(u32, u32)>>);
