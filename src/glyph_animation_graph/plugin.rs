use bevy::{
    app::{Plugin, PostUpdate},
    asset::AssetApp,
    ecs::schedule::IntoSystemConfigs,
};

use super::{
    player::{animation_graph_player, animation_graph_traverse},
    GlyphAnimationGraphAssetLoader, GlyphAnimationGraphSource,
};

pub struct GlyphAnimationGraphPlugin;

impl Plugin for GlyphAnimationGraphPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<GlyphAnimationGraphSource>()
            .init_asset_loader::<GlyphAnimationGraphAssetLoader>()
            .add_systems(
                PostUpdate,
                (animation_graph_traverse, animation_graph_player).chain(),
            );
    }
}
