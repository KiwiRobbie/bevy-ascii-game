use bevy::prelude::*;

use glyph_render::{glyph_render_plugin::GlyphTexture, glyph_sprite::GlyphSprite};

use crate::{layout::positioned::WidgetSize, widgets::divider::Divider};

pub(crate) fn divider_render(
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut commands: Commands,
    q_text: Query<(Entity, &WidgetSize, &Divider)>,
) {
    for (entity, size, divider) in q_text.iter() {
        commands.entity(entity).insert((GlyphSprite {
            texture: glyph_textures.add(GlyphTexture::from(vec![divider
                .character
                .to_string()
                .repeat(size.x as usize)])),
            offset: IVec2::ZERO,
        },));
    }
}
