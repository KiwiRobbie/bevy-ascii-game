use bevy::{asset::Asset, prelude::Deref, reflect::TypePath};

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub struct TilemapMeta {
    pub chunk_size: (u32, u32),
    pub tilesets: Vec<String>,
    pub chunk_dir: String,
    pub chunks: Vec<(i32, i32)>,
}

#[derive(serde::Deserialize, Asset, TypePath, Clone, Deref)]
pub struct ChunkMeta(pub Vec<Vec<(String, String)>>);
