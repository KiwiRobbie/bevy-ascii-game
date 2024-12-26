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

use crate::{layout::positioned::Positioned, widgets::divider::Divider};

pub(crate) fn divider_render(
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut commands: Commands,
    q_text: Query<(Entity, &Positioned, &Divider)>,
) {
    for (entity, positioned, divider) in q_text.iter() {
        commands.entity(entity).insert((GlyphSprite {
            texture: glyph_textures.add(GlyphTexture::from(vec![divider
                .character
                .to_string()
                .repeat(positioned.size.x as usize)])),
            offset: IVec2::ZERO,
        },));
    }
}
