use bevy::{
    prelude::*,
    render::{ExtractSchedule, RenderApp},
};

use super::{
    asset::TilemapSource,
    chunk::TilemapChunk,
    extract::extract_tilemaps,
    loader::{ChunkLoader, TilemapLoader},
};

pub struct TilemapPlugin;
impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<TilemapSource>()
            .init_asset::<TilemapChunk>()
            .init_asset_loader::<TilemapLoader>()
            .init_asset_loader::<ChunkLoader>();

        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .add_systems(ExtractSchedule, extract_tilemaps);
    }
}
