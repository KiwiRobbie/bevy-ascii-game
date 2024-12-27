use bevy::{asset::Asset, math::UVec2, reflect::TypePath, utils::HashMap};

use glyph_render::glyph_render_plugin::GlyphTextureSource;
use std::sync::Arc;

#[derive(Debug, Asset, TypePath, Clone)]
pub struct TilesetSource {
    pub display_name: String,
    pub id: String,
    pub tile_size: UVec2,
    pub(crate) _tile_ids: HashMap<String, usize>,
    pub(crate) _tile_labels: Vec<String>,
    pub tiles: Vec<Arc<GlyphTextureSource>>,
}
