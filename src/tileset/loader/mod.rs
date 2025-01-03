pub(crate) mod meta;

use std::sync::Arc;

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    math::UVec2,
    utils::{
        hashbrown::{HashMap, HashSet},
        ConditionalSendFuture,
    },
};
use glyph_render::glyph_render_plugin::GlyphTextureSource;

use self::meta::TilesetMeta;

use super::asset::TilesetSource;

#[derive(Default)]
pub(crate) struct TilesetLoader {}
impl TilesetLoader {
    fn parse_bytes(bytes: Vec<u8>) -> Vec<String> {
        bytes
            .split(|&b| b == b'\n')
            .map(|line| {
                String::from_utf8(line.strip_suffix(b"\r").unwrap_or(line).to_vec()).unwrap()
            })
            .collect::<Vec<_>>()
    }
}

impl AssetLoader for TilesetLoader {
    type Asset = TilesetSource;
    type Error = anyhow::Error;
    type Settings = ();

    fn extensions(&self) -> &[&str] {
        &["tileset.ron"]
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
            let meta = ron::de::from_bytes::<TilesetMeta>(&bytes)?;
            let unique_assets: HashSet<String> = meta
                .assets
                .iter()
                .map(|asset| asset.asset.clone())
                .collect();

            let mut artwork_data = HashMap::new();
            for asset_path in unique_assets.iter() {
                artwork_data.insert(
                    asset_path,
                    Self::parse_bytes(load_context.read_asset_bytes(asset_path).await.unwrap()),
                );
            }

            let tile_size: UVec2 = meta.size.into();
            let mut tiles = vec![];
            let mut tile_ids = HashMap::new();
            let mut tile_labels = vec![];

            for asset in meta.assets.iter() {
                match &asset.tiles {
                    meta::AssetTiles::All(name) => {
                        let source_data = artwork_data.get(&asset.asset).unwrap();
                        let tiles_y = source_data.len().div_euclid(tile_size.y as usize);

                        for tile_y in 0..tiles_y {
                            let mut tile_x = 0;
                            'add_x: loop {
                                let mut tile = Vec::new();
                                for dy in 0..tile_size.y as usize {
                                    let y = tile_y * tile_size.y as usize + dy;

                                    let x_start = tile_x * tile_size.x as usize;
                                    let x_end = x_start + tile_size.x as usize;
                                    if x_end > source_data[y].chars().count() {
                                        break 'add_x;
                                    }

                                    tile.push(
                                        source_data[y]
                                            .chars()
                                            .skip(x_start)
                                            .take(x_end - x_start)
                                            .collect::<String>(),
                                    );
                                }

                                let label = format!("{}-{}-{}", name, tile_x, tile_y);
                                tile_ids.insert(label.clone(), tiles.len());
                                tile_labels.push(label);
                                tiles.push(Arc::new(GlyphTextureSource::from(&tile)));

                                let mirrored_label = format!("{}-{}-{}-m", name, tile_x, tile_y);
                                let mirrored_data = text_util::text_mirror::mirror_lines(&tile);
                                tile_ids.insert(mirrored_label.clone(), tiles.len());
                                tile_labels.push(mirrored_label);
                                tiles.push(Arc::new(GlyphTextureSource::from(&mirrored_data)));

                                tile_x += 1;
                            }
                        }
                    }
                }
            }

            Ok(TilesetSource {
                display_name: meta.display_name,
                id: meta.id,
                tile_size,
                _tile_ids: tile_ids,
                _tile_labels: tile_labels,
                tiles,
            })
        })
    }
}
