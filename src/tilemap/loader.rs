use std::path::PathBuf;

use bevy::{
    asset::{io::Reader, meta::Settings, AssetLoader, AsyncReadExt, Handle},
    math::IVec2,
    utils::hashbrown::HashMap,
};

use crate::tileset::asset::TilesetSource;

use super::meta::{ChunkMeta, TilemapMeta};

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
                    PathBuf::from(&meta.chunk_dir).join(format!("{}_{}.chunk.ron", pos.x, pos.y));

                let tileset_names = tileset_names.clone();
                let tilesets = meta.tilesets.clone();

                // let chunk_handle =
                //     load_context.load_with_settings(path, move |settings: &mut ChunkSettings| {
                //         settings.tileset_names = tileset_names.clone();
                //         settings.tilesets = tilesets.clone();
                //     });

                let chunk = ChunkLoader
                    .load(
                        &mut load_context
                            .read_asset_bytes(path)
                            .await
                            .unwrap()
                            .as_slice(),
                        &ChunkSettings {
                            tileset_names: tileset_names.clone(),
                            tilesets: tilesets.clone(),
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

            for tileset in tilesets.into_iter() {
                tileset_handles.push(load_context.add_labeled_asset(tileset.id.clone(), tileset));
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

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
pub struct ChunkSettings {
    pub tileset_names: HashMap<String, usize>,
    pub tilesets: Vec<String>,
}

#[derive(Default)]
pub struct ChunkLoader;
impl AssetLoader for ChunkLoader {
    type Asset = TilemapChunk;
    type Error = anyhow::Error;
    type Settings = ChunkSettings;

    fn extensions(&self) -> &[&str] {
        &["chunk.ron"]
    }
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let meta = ron::de::from_bytes::<ChunkMeta>(&bytes)?;

            let mut tilesets = Vec::new();
            for tileset in settings.tilesets.iter() {
                let value: TilesetSource = load_context
                    .load_direct(tileset)
                    .await
                    .unwrap()
                    .take()
                    .unwrap();
                tilesets.push(value);
            }

            let mut tilesets = Vec::new();
            for tileset in settings.tilesets.iter() {
                let value: TilesetSource = load_context
                    .load_direct(tileset)
                    .await
                    .unwrap()
                    .take()
                    .unwrap();
                tilesets.push(value);
            }

            let mut data = vec![];
            for row in meta.iter() {
                for (tileset, tile) in row.iter() {
                    if let Some(tileset) = settings.tileset_names.get(tileset) {
                        if let Some(tile) = tilesets[*tileset].tile_ids.get(tile) {
                            data.push(Some((*tileset as u32, *tile as u32)));
                        } else {
                            data.push(None)
                        }
                    } else {
                        data.push(None)
                    };
                }
            }

            Ok(TilemapChunk {
                data: data.into_boxed_slice(),
            })
        })
    }
}
