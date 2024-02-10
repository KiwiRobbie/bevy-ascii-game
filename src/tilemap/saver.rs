use crate::{tilemap::meta::ChunkMeta, tileset::asset::TilesetSource};

use super::{
    asset::TilemapSource,
    chunk::TilemapChunk,
    loader::{ChunkLoader, ChunkSettings},
};
use super::{loader::TilemapLoader, meta::TilemapMeta};
use bevy::asset::{saver::AssetSaver, AssetLoader, AsyncWriteExt};

#[derive(Default)]
pub struct TilemapSaver;

impl AssetSaver for TilemapSaver {
    type OutputLoader = TilemapLoader;
    type Asset = TilemapSource;
    type Error = anyhow::Error;
    type Settings = ();

    fn save<'a>(
        &'a self,
        writer: &'a mut bevy::asset::io::Writer,
        asset: bevy::asset::saver::SavedAsset<'a, Self::Asset>,
        _settings: &'a Self::Settings,
    ) -> bevy::utils::BoxedFuture<
        'a,
        Result<<Self::OutputLoader as AssetLoader>::Settings, Self::Error>,
    > {
        dbg!(&asset.iter_labels().collect::<Vec<_>>());

        Box::pin(async move {
            let mut tilesets = vec![];
            let mut chunks = vec![];

            dbg!(asset.iter_labels().collect::<Vec<_>>());

            for (chunk_id, chunk) in asset.chunk_handles.iter() {
                chunks.push((*chunk_id).into());
            }

            let meta = TilemapMeta {
                chunk_size: asset.chunk_size.into(),
                tile_size: asset.tile_size.into(),
                chunk_dir: "chunks".into(),
                tilesets,
                chunks,
            };

            let string = ron::ser::to_string_pretty(&meta, Default::default()).unwrap();
            dbg!(&string);
            writer.write_all(string.as_bytes()).await.unwrap();

            Ok(())
        })
    }
}

pub struct ChunkSaver;
impl AssetSaver for ChunkSaver {
    type OutputLoader = ChunkLoader;
    type Asset = TilemapChunk;
    type Error = anyhow::Error;
    type Settings = ChunkSettings;

    fn save<'a>(
        &'a self,
        writer: &'a mut bevy::asset::io::Writer,
        asset: bevy::asset::saver::SavedAsset<'a, Self::Asset>,
        settings: &'a Self::Settings,
    ) -> bevy::utils::BoxedFuture<
        'a,
        Result<<Self::OutputLoader as AssetLoader>::Settings, Self::Error>,
    > {
        Box::pin(async move {
            let mut data = vec![];

            let mut tilesets: Vec<&TilesetSource> = vec![];
            for tileset in settings.tilesets.iter() {
                tilesets.push(
                    asset
                        .get_labeled::<TilesetSource>(tileset.clone())
                        .unwrap()
                        .get(),
                );
            }
            for row in asset.data.into_iter() {
                let mut data_row = vec![];
                for cell in row.into_iter() {
                    let tileset = settings.tilesets[cell.0 as usize].clone();
                    let tile = tilesets[cell.0 as usize].tile_labels[cell.1 as usize].clone();
                    data_row.push((tileset, tile));
                }
                data.push(data_row);
            }

            let meta = ChunkMeta(data);

            let string = ron::ser::to_string_pretty(&meta, Default::default()).unwrap();
            dbg!(&string);
            writer.write_all(string.as_bytes()).await.unwrap();

            Ok(settings.clone())
        })
    }
}
