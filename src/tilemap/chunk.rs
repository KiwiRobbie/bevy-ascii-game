use bevy::math::UVec2;

#[derive(Debug)]
pub struct TilemapChunk {
    pub data: Box<[Option<(u32, u32)>]>,
}
impl TilemapChunk {
    pub fn empty(size: UVec2) -> Self {
        Self {
            data: vec![None; size.x as usize * size.y as usize].into_boxed_slice(),
        }
    }
}
