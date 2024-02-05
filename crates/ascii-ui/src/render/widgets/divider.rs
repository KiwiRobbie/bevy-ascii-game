use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        system::{Commands, Query, ResMut},
    },
    math::{IVec2, Vec2},
};
use glyph_render::glyph_render_plugin::{GlyphSprite, GlyphTextureSource};
use spatial_grid::position::Position;

use crate::{layout::positioned::Positioned, widgets::divider::Divider};

pub fn divider_render(
    mut glyph_textures: ResMut<Assets<GlyphTextureSource>>,
    mut commands: Commands,
    q_text: Query<(Entity, &Positioned, &Divider)>,
) {
    for (entity, positioned, divider) in q_text.iter() {
        commands.entity(entity).insert((
            Position {
                position: positioned.offset * IVec2::new(1, -1)
                    - IVec2::Y * positioned.size.y as i32,
                remainder: Vec2::ZERO,
            },
            GlyphSprite {
                texture: glyph_textures.add(GlyphTextureSource {
                    data: vec![divider
                        .character
                        .to_string()
                        .repeat(positioned.size.x as usize)],
                }),
                offset: IVec2::ZERO,
            },
        ));
    }
}
