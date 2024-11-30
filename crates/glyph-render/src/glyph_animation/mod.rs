use std::sync::Arc;

use bevy::{
    app::Plugin,
    asset::{Asset, AssetApp, Handle},
    ecs::component::Component,
    math::{IVec2, UVec2},
    reflect::TypePath,
};

use crate::glyph_render_plugin::GlyphTextureSource;

use self::loader::GlyphAnimationAssetLoader;

mod loader;
pub mod player;
#[derive(Asset, TypePath)]
pub struct GlyphAnimationSource {
    pub name: String,
    pub size: UVec2,
    pub frames: Vec<(GlyphAnimationFrame, Option<GlyphAnimationFrame>)>,
}

#[derive(Clone, Debug)]
pub struct GlyphAnimationFrame {
    pub source: Arc<GlyphTextureSource>,
    pub offset: IVec2,
}

impl GlyphAnimationFrame {
    pub fn new(data: Vec<String>, offset: IVec2) -> Self {
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
