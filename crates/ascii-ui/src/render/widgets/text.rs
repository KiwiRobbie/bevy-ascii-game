use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        system::{Commands, Query, ResMut},
    },
    math::IVec2,
};
use glyph_render::{glyph_render_plugin::GlyphTexture, glyph_sprite::GlyphSprite};
use spatial_grid::{depth::Depth, position::Position};

use crate::{layout::positioned::Positioned, widgets::text::Text};

pub fn text_render(
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut commands: Commands,
    q_text: Query<(Entity, &Positioned, &Text)>,
) {
    for (entity, positioned, text) in q_text.iter() {
        commands.entity(entity).insert((
            Position(positioned.offset * IVec2::new(1, -1) - IVec2::Y * positioned.size.y as i32),
            GlyphSprite {
                texture: glyph_textures.add(GlyphTexture::from(vec![text.text.clone()])),
                offset: IVec2::ZERO,
            },
            Depth(0.0),
        ));
    }
}
