use super::asset::TilemapSource;
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
        Box::pin(async move {
            let mut tilesets = vec![];
            let mut chunks = vec![];

            for (chunk_id, chunk) in asset.chunk_data.iter() {
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

    // fn load<'a>(
    //     &'a self,
    //     reader: &'a mut Reader,
    //     _settings: &'a Self::Settings,
    //     load_context: &'a mut bevy::asset::LoadContext,
    // ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
    //     Box::pin(async move {
    //         let mut bytes = Vec::new();
    //         reader.read_to_end(&mut bytes).await?;
    //         let meta = ron::de::from_bytes::<TilemapMeta>(&bytes)?;

    //         let mut tilesets = Vec::new();
    //         for tileset in meta.tilesets.iter() {
    //             let value: TilesetSource = load_context
    //                 .load_direct(tileset)
    //                 .await
    //                 .unwrap()
    //                 .take()
    //                 .unwrap();
    //             fn extensions(&self) -> &[&str] {
    //                 &["tilemap.ron"]
    //             }t mut tileset_names: HashMap<String, usize> = HashMap::new();
    //         for (i, tileset) in tilesets.iter().enumerate() {
    //             tileset_names.insert(tileset.id.clone(), i);
    //         }

    //         let mut chunk_data: HashMap<IVec2, TilemapChunk> = HashMap::new();

    //         for pos in meta.chunks.iter() {
    //             let pos: IVec2 = (*pos).into();
    //             let path =
    //                 PathBuf::from(&meta.chunk_dir).join(format!("{}_{}.chunk.ron", pos.x, pos.y));
    //             let data = load_context.read_asset_bytes(path).await.unwrap();
    //             let Some(meta) = ron::de::from_bytes::<ChunkMeta>(&data).ok() else {
    //                 continue;
    //             };
    //             let mut data = vec![];

    //             for row in meta.iter() {
    //                 for tile in row.iter() {
    //                     if let Some((tileset, tile)) = tile {
    //                         let tileset = *tileset_names.get(tileset).unwrap();
    //                         let tile = *tilesets[tileset].tile_names.get(tile).unwrap();
    //                         data.push(Some((tileset as u32, tile as u32)));
    //                     } else {
    //                         data.push(None)
    //                     }
    //                 }
    //             }

    //             chunk_data.insert(
    //                 pos,
    //                 TilemapChunk {
    //                     data: data.into_boxed_slice(),
    //                 },
    //             );
    //         }

    //         let mut tileset_handles = Vec::new();

    //         for tileset in tilesets.into_iter() {
    //             tileset_handles.push(load_context.add_labeled_asset(tileset.id.clone(), tileset));
    //         }

    //         Ok(TilemapSource {
    //             chunk_size: meta.chunk_size.into(),
    //             tile_size: meta.tile_size.into(),
    //             tileset_names,
    //             tilesets: tileset_handles,
    //             chunk_data,
    //         })
    //     })
    // }
}
