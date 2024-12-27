use bevy::prelude::*;

use glyph_render::{glyph_render_plugin::GlyphTexture, glyph_sprite::GlyphSprite};

use crate::{attachments::border::Border, layout::positioned::WidgetSize};

pub(crate) fn border_render(
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut commands: Commands,
    q_text: Query<(Entity, &WidgetSize, &Border)>,
) {
    for (entity, size, border) in q_text.iter() {
        let data = border.create_data(**size);
        commands.entity(entity).insert((GlyphSprite {
            texture: glyph_textures.add(GlyphTexture::from(data)),
            offset: IVec2::ZERO,
        },));
    }
}
