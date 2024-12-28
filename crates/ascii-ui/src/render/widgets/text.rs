use bevy::prelude::*;

use crate::{theme::UiTheme, widgets::text::Text};
use glyph_render::{
    glyph_render_plugin::{GlyphTexture, SolidColor},
    glyph_sprite::GlyphSprite,
};
use spatial_grid::depth::Depth;

pub(crate) fn text_render(
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut commands: Commands,
    q_text: Query<(Entity, &Text, Has<SolidColor>)>,
    theme: Res<UiTheme>,
) {
    for (entity, text, color) in q_text.iter() {
        let mut entity_commands = commands.entity(entity);
        entity_commands.insert((
            GlyphSprite {
                texture: glyph_textures.add(GlyphTexture::from(vec![text.text.clone()])),
                offset: IVec2::ZERO,
            },
            Depth(0.0),
        ));
        if !color {
            entity_commands.insert(SolidColor {
                color: text.style.get_style(&theme).color,
            });
        }
    }
}
