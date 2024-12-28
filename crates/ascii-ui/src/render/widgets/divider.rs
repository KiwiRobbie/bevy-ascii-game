use bevy::prelude::*;

use glyph_render::{
    glyph_render_plugin::{GlyphTexture, SolidColor},
    glyph_sprite::GlyphSprite,
};

use crate::{layout::positioned::WidgetSize, theme::UiTheme, widgets::divider::Divider};

pub(crate) fn divider_render(
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut commands: Commands,
    q_divider: Query<(Entity, &WidgetSize, &Divider, Has<SolidColor>)>,
    theme: Res<UiTheme>,
) {
    for (entity, size, divider, has_color) in q_divider.iter() {
        let mut entity_commands = commands.entity(entity);

        entity_commands.insert((GlyphSprite {
            texture: glyph_textures.add(GlyphTexture::from(vec![divider
                .character
                .to_string()
                .repeat(size.x as usize)])),
            offset: IVec2::ZERO,
        },));
        if !has_color {
            entity_commands.insert(SolidColor {
                color: divider.style.get_style(&theme).color,
            });
        }
    }
}
