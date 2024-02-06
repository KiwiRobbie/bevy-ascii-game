use bevy::{asset::Asset, reflect::TypePath};

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub struct TilesetMeta {
    pub display_name: String,
    pub id: String,
    pub size: (u32, u32),
    pub assets: Vec<TileSourceMeta>,
}

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub struct TileSourceMeta {
    pub asset: String,
    pub tiles: AssetTiles,
}

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub enum AssetTiles {
    All(String),
}
