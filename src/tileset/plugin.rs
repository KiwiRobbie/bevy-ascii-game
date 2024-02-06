use bevy::{app::Plugin, asset::AssetApp};

use super::{asset::TilesetSource, loader::TilesetLoader};

pub struct TilesetPlugin;
impl Plugin for TilesetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<TilesetSource>()
            .init_asset_loader::<TilesetLoader>();
    }
}
