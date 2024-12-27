use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use bevy::utils::ConditionalSendFuture;

use super::meta::TilemapMeta;
use super::{asset::TilemapSource, chunk::TilemapChunk};
use crate::tileset::asset::TilesetSource;
use std::path::PathBuf;

#[derive(Default)]
pub struct TilemapLoader {}

impl AssetLoader for TilemapLoader {
    type Asset = TilemapSource;
    type Error = anyhow::Error;
    type Settings = ();

    fn extensions(&self) -> &[&str] {
        &["tilemap.ron"]
    }

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let meta = ron::de::from_bytes::<TilemapMeta>(&bytes)?;

            let mut tilesets = Vec::new();
            for tileset in meta.tilesets.iter() {
                let loaded_asset: bevy::asset::LoadedAsset<TilesetSource> =
                    load_context.loader().immediate().load(tileset).await?;
                let value: TilesetSource = loaded_asset.get().clone();

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
                        &mut VecReader::new(load_context.read_asset_bytes(path).await.unwrap()),
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
    fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let data = bytes
                .chunks_exact(8)
                .map(|chunks| {
                    let mut tileset_bytes = [0u8; 4];
                    let mut tile_bytes = [0u8; 4];
                    tileset_bytes.copy_from_slice(&chunks[0..4]);
                    tile_bytes.copy_from_slice(&chunks[4..8]);

                    (
                        u32::from_le_bytes(tileset_bytes),
                        u32::from_le_bytes(tile_bytes),
                    )
                })
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
