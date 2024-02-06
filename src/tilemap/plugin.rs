use bevy::{app::Plugin, asset::AssetApp};

use super::{asset::TilemapSource, loader::TilemapLoader};

pub struct TilemapPlugin;
impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<TilemapSource>()
            .init_asset_loader::<TilemapLoader>();
    }
}
