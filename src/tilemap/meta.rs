use bevy::{asset::Asset, prelude::Deref, reflect::TypePath};

#[derive(serde::Deserialize, serde::Serialize, Asset, TypePath, Clone)]
pub(crate) struct TilemapMeta {
    pub(crate) chunk_size: (u32, u32),
    pub(crate) tile_size: (u32, u32),
    pub(crate) tilesets: Vec<String>,
    pub(crate) chunk_dir: String,
    pub(crate) chunks: Vec<(i32, i32)>,
}

#[derive(serde::Deserialize, Asset, TypePath, Clone, Deref, serde::Serialize)]
pub(crate) struct ChunkMeta(pub(crate) Vec<Vec<(u32, u32)>>);
