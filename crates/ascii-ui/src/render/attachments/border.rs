use bevy::prelude::*;

use glyph_render::{
    glyph_render_plugin::{GlyphTexture, SolidColor},
    glyph_sprite::GlyphSprite,
};

use crate::{attachments::border::Border, layout::positioned::WidgetSize, theme::UiTheme};

pub(crate) fn border_render(
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut commands: Commands,
    q_text: Query<(Entity, &WidgetSize, &Border, Has<SolidColor>)>,
    theme: Res<UiTheme>,
) {
    for (entity, size, border, has_color) in q_text.iter() {
        let data = border.create_data(**size);
        let mut entity_commands = commands.entity(entity);

        entity_commands.insert((GlyphSprite {
            texture: glyph_textures.add(GlyphTexture::from(data)),
            offset: IVec2::ZERO,
        },));

        if !has_color {
            entity_commands.insert(SolidColor {
                color: border.style.get_style(&theme).color,
            });
        }
    }
}
