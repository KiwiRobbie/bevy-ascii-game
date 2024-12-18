use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        system::{Commands, Query, ResMut},
    },
    math::IVec2,
};
use glyph_render::{glyph_render_plugin::GlyphTexture, glyph_sprite::GlyphSprite};
use spatial_grid::position::Position;

use crate::{attachments::border::Border, layout::positioned::Positioned};

pub fn border_render(
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut commands: Commands,
    q_text: Query<(Entity, &Positioned, &Border)>,
) {
    for (entity, positioned, border) in q_text.iter() {
        let data = border.create_data(positioned.size);
        let position = positioned.offset * IVec2::new(1, -1) - IVec2::Y * positioned.size.y as i32;
        commands.entity(entity).insert((
            Position(position),
            GlyphSprite {
                texture: glyph_textures.add(GlyphTexture::from(data)),
                offset: IVec2::ZERO,
            },
        ));
    }
}
