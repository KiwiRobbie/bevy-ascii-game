use bevy::{
    asset::{Asset, Handle},
    math::{IVec2, UVec2},
    reflect::TypePath,
    utils::HashMap,
};

use crate::tileset::asset::TilesetSource;

use super::chunk::TilemapChunk;

#[derive(Debug, Asset, TypePath)]
pub struct TilemapSource {
    pub chunk_size: UVec2,
    pub tile_size: UVec2,
    pub tileset_names: HashMap<String, usize>,
    pub tilesets: Vec<Handle<TilesetSource>>,
    pub chunk_data: HashMap<IVec2, TilemapChunk>,
}

impl TilemapSource {
    pub fn chunk_id_index(&self, pos: IVec2) -> (IVec2, u32) {
        let chunk_sub_pos = pos.rem_euclid(self.chunk_size.as_ivec2()).as_uvec2();
        (
            pos.div_euclid(self.chunk_size.as_ivec2()),
            chunk_sub_pos.x + chunk_sub_pos.y * self.chunk_size.x,
        )
    }

    pub fn insert_tile(
        &mut self,
        pos: IVec2,
        tileset_id: String,
        tileset_tile_index: u32,
        tileset: Handle<TilesetSource>,
    ) {
        let (chunk_id, chunk_tile_index) = self.chunk_id_index(pos);
        let chunk = self
            .chunk_data
            .entry(chunk_id)
            .or_insert(TilemapChunk::empty(self.chunk_size));

        let tileset_index = {
            let new_index = self.tileset_names.len();
            if let Some(index) = self.tileset_names.get(&tileset_id) {
                *index
            } else {
                self.tileset_names.insert(tileset_id, new_index);
                self.tilesets.push(tileset);
                new_index
            }
        };

        chunk.data[chunk_tile_index as usize] = Some((tileset_index as u32, tileset_tile_index));
    }
    pub fn clear_tile(&mut self, pos: IVec2) -> Option<(u32, u32)> {
        let (chunk_id, chunk_tile_index) = self.chunk_id_index(pos);
        let chunk = self.chunk_data.get_mut(&chunk_id)?;
        chunk.data[chunk_tile_index as usize].take()
    }
}
