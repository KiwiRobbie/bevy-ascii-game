use crate::glyph_render_plugin::GlyphTexture;
use bevy::{
    app::Plugin,
    asset::{AssetApp, Handle},
    ecs::component::Component,
    math::IVec2,
};

use self::loader::GlyphTextureLoader;

pub(crate) mod loader;

#[derive(Component, Clone)]
pub struct GlyphSprite {
    pub texture: Handle<GlyphTexture>,
    pub offset: IVec2,
}

pub struct GlyphTexturePlugin;

impl Plugin for GlyphTexturePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_asset_loader(GlyphTextureLoader);
    }
}
