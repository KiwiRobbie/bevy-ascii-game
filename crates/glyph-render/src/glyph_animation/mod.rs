use bevy::prelude::*;

use self::loader::GlyphAnimationAssetLoader;
use crate::glyph_render_plugin::GlyphTextureSource;
use std::sync::Arc;

mod loader;
pub mod player;

#[derive(Debug, Asset, TypePath)]
pub struct GlyphAnimationSource {
    pub name: String,
    pub size: UVec2,
    pub(crate) frames: Vec<(GlyphAnimationFrame, Option<GlyphAnimationFrame>)>,
}

#[derive(Clone, Debug)]
pub(crate) struct GlyphAnimationFrame {
    pub(crate) source: Arc<GlyphTextureSource>,
    pub(crate) offset: IVec2,
}

impl GlyphAnimationFrame {
    pub(crate) fn new(data: Vec<String>, offset: IVec2) -> Self {
        Self {
            source: Arc::new((&data).into()),
            offset,
        }
    }
}

#[derive(Component, Clone)]
pub struct GlyphAnimation {
    pub source: Handle<GlyphAnimationSource>,
    pub frame: u32,
}

pub struct GlyphAnimationPlugin;
impl Plugin for GlyphAnimationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<GlyphAnimationSource>()
            .init_asset_loader::<GlyphAnimationAssetLoader>();
    }
}
