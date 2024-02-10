pub mod meta;

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt},
    math::UVec2,
    utils::hashbrown::{HashMap, HashSet},
};

use self::meta::TilesetMeta;

use super::asset::TilesetSource;

#[derive(Default)]
pub struct TilesetLoader {}
impl TilesetLoader {
    fn parse_bytes(bytes: Vec<u8>) -> Vec<String> {
        bytes
            .split(|&b| b == b'\n')
            .map(|line| {
                String::from_utf8(line.strip_suffix(b"\r").unwrap_or(line).to_vec())
                    .unwrap()
                    .replace('·', " ")
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

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
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

                                    if x_end >= source_data[y].len() {
                                        break 'add_x;
                                    }

                                    tile.push(source_data[y][x_start..x_end].to_string());
                                }

                                let label = format!("{}-{}-{}", name, tile_x, tile_y);
                                tile_ids.insert(label.clone(), tiles.len());
                                tile_labels.push(label);
                                tiles.push(tile);

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
                tile_ids,
                tile_labels,
                tiles,
            })
        })
    }
}
