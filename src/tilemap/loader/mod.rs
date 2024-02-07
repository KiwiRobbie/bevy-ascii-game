pub mod meta;

use std::path::PathBuf;

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt},
    math::IVec2,
    utils::hashbrown::HashMap,
};

use crate::tileset::asset::TilesetSource;

use self::meta::{ChunkMeta, TilemapMeta};

use super::{asset::TilemapSource, chunk::TilemapChunk};

#[derive(Default)]
pub struct TilemapLoader {}

impl AssetLoader for TilemapLoader {
    type Asset = TilemapSource;
    type Error = anyhow::Error;
    type Settings = ();

    fn extensions(&self) -> &[&str] {
        &["tilemap.ron"]
    }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let meta = ron::de::from_bytes::<TilemapMeta>(&bytes)?;

            let mut tilesets = Vec::new();
            for tileset in meta.tilesets.iter() {
                let value: TilesetSource = load_context
                    .load_direct(tileset)
                    .await
                    .unwrap()
                    .take()
                    .unwrap();
                tilesets.push(value);
            }

            let mut tileset_names: HashMap<String, usize> = HashMap::new();
            for (i, tileset) in tilesets.iter().enumerate() {
                tileset_names.insert(tileset.id.clone(), i);
            }

            let mut chunk_data: HashMap<IVec2, TilemapChunk> = HashMap::new();

            for pos in meta.chunks.iter() {
                let pos: IVec2 = (*pos).into();
                let path =
                    PathBuf::from(&meta.chunk_dir).join(format!("{}_{}.chunk.ron", pos.x, pos.y));
                let data = load_context.read_asset_bytes(path).await.unwrap();
                let Some(meta) = ron::de::from_bytes::<ChunkMeta>(&data).ok() else {
                    continue;
                };
                let mut data = vec![];

                for row in meta.iter() {
<<<<<<< HEAD
                    for (tileset, tile) in row.iter() {
                        let tileset = *tileset_names.get(tileset).unwrap();
                        let tile = *tilesets[tileset]
                            .tile_names
                            .get(tile)
                            .expect(format!("Tile map missing {}", tile).as_str());
                        data.push((tileset as u32, tile as u32));
=======
                    for tile in row.iter() {
                        if let Some((tileset, tile)) = tile {
                            let tileset = *tileset_names.get(tileset).unwrap();
                            let tile = *tilesets[tileset].tile_names.get(tile).unwrap();
                            data.push(Some((tileset as u32, tile as u32)));
                        } else {
                            data.push(None)
                        }
>>>>>>> main
                    }
                }

                chunk_data.insert(
                    pos,
                    TilemapChunk {
                        data: data.into_boxed_slice(),
                    },
                );
            }

            let mut tileset_handles = Vec::new();

            for tileset in tilesets.into_iter() {
                tileset_handles.push(load_context.add_labeled_asset(tileset.id.clone(), tileset));
            }

            Ok(TilemapSource {
                chunk_size: meta.chunk_size.into(),
                tile_size: meta.tile_size.into(),
                tileset_names,
                tilesets: tileset_handles,
                chunk_data,
            })
        })
    }
}
<<<<<<< HEAD
=======
fn load_chunk<'a>(
    directory: PathBuf,
    chunk_entry: DirEntry,
    load_context: &'a mut bevy::asset::LoadContext,
) -> bevy::utils::BoxedFuture<'a, Option<(IVec2, ChunkMeta)>> {
    Box::pin(async move {
        let file_name = chunk_entry.file_name();
        let name = file_name.to_str().unwrap();
        let (coords, suffix) = name.split_once('.')?;
        if suffix != "chunk.ron" {
            return None;
        }
        let (x, y) = coords.split_once('_')?;

        let x = x.parse::<i32>().ok()?;
        let y = y.parse::<i32>().ok()?;
        let pos = IVec2 { x, y };

        let data = load_context
            .read_asset_bytes(directory.join(file_name))
            .await
            .unwrap();
        let meta = ron::de::from_bytes::<ChunkMeta>(&data).ok()?;
        Some((pos, meta))
    })
}
>>>>>>> main
