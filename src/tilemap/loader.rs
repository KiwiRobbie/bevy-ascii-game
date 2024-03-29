use std::path::PathBuf;

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, Handle},
    math::{IVec2, UVec2},
    utils::hashbrown::HashMap,
};

use crate::tileset::asset::TilesetSource;

use super::meta::TilemapMeta;

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

            let mut chunk_handles: HashMap<IVec2, Handle<TilemapChunk>> = HashMap::new();

            for pos in meta.chunks.iter() {
                let pos: IVec2 = (*pos).into();
                let path =
                    PathBuf::from(&meta.chunk_dir).join(format!("{}_{}.chunk.bin", pos.x, pos.y));

                let chunk = ChunkLoader
                    .load(
                        &mut load_context
                            .read_asset_bytes(path)
                            .await
                            .unwrap()
                            .as_slice(),
                        &ChunkSettings {
                            size: Some(meta.chunk_size.into()),
                        },
                        load_context,
                    )
                    .await
                    .unwrap();

                let chunk_handle =
                    load_context.add_labeled_asset(format!("{}-{}", pos.x, pos.y), chunk);

                chunk_handles.insert(pos, chunk_handle);
            }

            let mut tileset_handles = Vec::new();

            for tileset in meta.tilesets.iter() {
                tileset_handles.push(load_context.load(tileset));
            }

            Ok(TilemapSource {
                chunk_size: meta.chunk_size.into(),
                tile_size: meta.tile_size.into(),
                tileset_names,
                tilesets: tileset_handles,
                chunk_handles,
            })
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct ChunkSettings {
    size: Option<UVec2>,
}

#[derive(Default)]
pub struct ChunkLoader;
impl AssetLoader for ChunkLoader {
    type Asset = TilemapChunk;
    type Error = anyhow::Error;
    type Settings = ChunkSettings;

    fn extensions(&self) -> &[&str] {
        &["chunk.bin"]
    }
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let data = bytes
                .array_chunks::<4>()
                .array_chunks::<2>()
                .map(|[tileset, tile]| (u32::from_le_bytes(*tileset), u32::from_le_bytes(*tile)))
                .collect::<Box<[(u32, u32)]>>();

            if let Some(size) = settings.size {
                if data.len() != size.x as usize * size.y as usize {
                    return Ok(TilemapChunk::empty(size));
                }
            }

            Ok(TilemapChunk { data })
        })
    }
}
