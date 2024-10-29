use super::{
    asset::TilemapSource,
    chunk::TilemapChunk,
    loader::{ChunkLoader, ChunkSettings},
};
use super::{loader::TilemapLoader, meta::TilemapMeta};
use bevy::{
    asset::{saver::AssetSaver, AssetLoader, AsyncWriteExt},
    utils::ConditionalSendFuture,
};

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
    ) -> impl ConditionalSendFuture<
        Output = Result<<Self::OutputLoader as AssetLoader>::Settings, Self::Error>,
    > {
        Box::pin(async move {
            let mut chunks = vec![];

            for (chunk_id, _) in asset.chunk_handles.iter() {
                chunks.push((*chunk_id).into());
            }

            let meta = TilemapMeta {
                chunk_size: asset.chunk_size.into(),
                tile_size: asset.tile_size.into(),
                chunk_dir: "tilemaps/output".into(),
                tilesets: asset
                    .tilesets
                    .iter()
                    .map(|h| h.path().unwrap().to_string())
                    .collect(),
                chunks,
            };

            let string = ron::ser::to_string_pretty(&meta, Default::default()).unwrap();
            dbg!(&string);
            dbg!(writer.write_all(string.as_bytes()).await).unwrap();
            writer.flush().await.unwrap();
            writer.close().await.unwrap();
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
    ) -> impl ConditionalSendFuture<
        Output = Result<<Self::OutputLoader as AssetLoader>::Settings, Self::Error>,
    > {
        Box::pin(async move {
            let data = asset
                .data
                .iter()
                .flat_map(|(tileset, tile)| {
                    tileset.to_le_bytes().into_iter().chain(tile.to_le_bytes())
                })
                .collect::<Box<[_]>>();

            dbg!(writer.write_all(&data).await).unwrap();
            writer.flush().await.unwrap();
            writer.close().await.unwrap();
            Ok(settings.clone())
        })
    }
}
