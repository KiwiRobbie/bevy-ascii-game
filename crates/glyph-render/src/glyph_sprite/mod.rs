use bevy::prelude::*;

use self::loader::GlyphTextureLoader;
use crate::glyph_render_plugin::GlyphTexture;

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
