use std::sync::Arc;

use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        system::{Commands, Query, ResMut},
    },
    math::IVec2,
};
use glyph_render::{
    glyph_render_plugin::{GlyphTexture, GlyphTextureSource},
    glyph_sprite::GlyphSprite,
};
use spatial_grid::position::Position;

use crate::{layout::positioned::Positioned, widgets::Texture};

pub(crate) fn texture_render(
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut commands: Commands,
    q_text: Query<(Entity, &Positioned, &Texture)>,
) {
    for (entity, positioned, text) in q_text.iter() {
        commands.entity(entity).insert((GlyphSprite {
            texture: glyph_textures.add(GlyphTexture::new(Arc::new(GlyphTextureSource::new(
                text.size.x as usize,
                text.size.y as usize,
                text.data.clone(),
            )))),
            offset: IVec2::ZERO,
        },));
    }
}
