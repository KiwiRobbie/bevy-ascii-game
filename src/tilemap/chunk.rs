use bevy::prelude::*;

#[derive(Debug, Clone, Asset, TypePath)]
pub struct TilemapChunk {
    pub(crate) data: Box<[(u32, u32)]>,
}
impl TilemapChunk {
    pub(crate) fn empty(size: UVec2) -> Self {
        Self {
            data: vec![EMPTY_TILE; size.x as usize * size.y as usize].into_boxed_slice(),
        }
    }
}
pub(crate) const EMPTY_TILE: (u32, u32) = (u32::MAX, u32::MAX);
