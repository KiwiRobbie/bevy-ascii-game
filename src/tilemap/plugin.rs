use bevy::{
    app::Plugin,
    asset::AssetApp,
    render::{ExtractSchedule, RenderApp},
};

use super::{asset::TilemapSource, extract::extract_tilemaps, loader::TilemapLoader};

pub struct TilemapPlugin;
impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<TilemapSource>()
            .init_asset_loader::<TilemapLoader>();

        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .add_systems(ExtractSchedule, extract_tilemaps);
    }
}
