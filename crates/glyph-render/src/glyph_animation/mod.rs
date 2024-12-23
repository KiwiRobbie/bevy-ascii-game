use std::sync::Arc;

use bevy::{
    app::Plugin,
    asset::{Asset, AssetApp, Handle},
    ecs::component::Component,
    math::{IVec2, UVec2},
    reflect::TypePath,
};

use self::loader::GlyphAnimationAssetLoader;
use crate::glyph_render_plugin::GlyphTextureSource;

mod loader;
pub mod player;

#[derive(Asset, TypePath)]
pub struct GlyphAnimationSource {
    pub(crate) name: String,
    pub(crate) size: UVec2,
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
