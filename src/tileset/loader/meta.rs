use bevy::{asset::Asset, reflect::TypePath};

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub(crate) struct TilesetMeta {
    pub(crate) display_name: String,
    pub(crate) id: String,
    pub(crate) size: (u32, u32),
    pub(crate) assets: Vec<TileSourceMeta>,
}

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub(crate) struct TileSourceMeta {
    pub(crate) asset: String,
    pub(crate) tiles: AssetTiles,
}

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub(crate) enum AssetTiles {
    All(String),
}
