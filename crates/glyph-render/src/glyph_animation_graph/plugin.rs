use bevy::prelude::*;

use super::{
    player::{animation_graph_player, animation_graph_traverse},
    GlyphAnimationGraphAssetLoader, GlyphAnimationGraphSource,
};
use crate::glyph_animation::player::loop_animation_player;

pub struct GlyphAnimationGraphPlugin;

impl Plugin for GlyphAnimationGraphPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<GlyphAnimationGraphSource>()
            .init_asset_loader::<GlyphAnimationGraphAssetLoader>()
            .add_systems(
                PostUpdate,
                (
                    (animation_graph_traverse, animation_graph_player).chain(),
                    loop_animation_player,
                ),
            );
    }
}
