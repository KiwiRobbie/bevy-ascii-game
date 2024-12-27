use bevy::prelude::*;

use crate::widgets::text::Text;
use glyph_render::{glyph_render_plugin::GlyphTexture, glyph_sprite::GlyphSprite};
use spatial_grid::depth::Depth;

pub(crate) fn text_render(
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut commands: Commands,
    q_text: Query<(Entity, &Text)>,
) {
    for (entity, text) in q_text.iter() {
        commands.entity(entity).insert((
            GlyphSprite {
                texture: glyph_textures.add(GlyphTexture::from(vec![text.text.clone()])),
                offset: IVec2::ZERO,
            },
            Depth(0.0),
        ));
    }
}
