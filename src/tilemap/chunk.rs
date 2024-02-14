use bevy::{asset::Asset, math::UVec2, reflect::TypePath};

#[derive(Debug, Clone, Asset, TypePath)]
pub struct TilemapChunk {
    pub data: Box<[(u32, u32)]>,
}
impl TilemapChunk {
    pub fn empty(size: UVec2) -> Self {
        Self {
            data: vec![EMPTY_TILE; size.x as usize * size.y as usize].into_boxed_slice(),
        }
    }
}
pub const EMPTY_TILE: (u32, u32) = (u32::MAX, u32::MAX);
