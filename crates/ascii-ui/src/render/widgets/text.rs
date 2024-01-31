use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        system::{Commands, Query, ResMut},
    },
    math::{IVec2, Vec2},
};
use glyph_render::glyph_render_plugin::{GlyphSprite, GlyphTexture};
use grid_physics::position::Position;

use crate::{layout::positioned::Positioned, widgets::text::Text};

pub fn text_render(
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut commands: Commands,
    q_text: Query<(Entity, &Positioned, &Text)>,
) {
    for (entity, positioned, text) in q_text.iter() {
        dbg!(entity, positioned);
        commands.entity(entity).insert((
            Position {
                position: positioned.offset * IVec2::new(1, -1)
                    - IVec2::Y * positioned.size.y as i32,
                remainder: Vec2::ZERO,
            },
            GlyphSprite {
                texture: glyph_textures.add(GlyphTexture {
                    data: vec![text.text.clone()],
                }),
                offset: IVec2::ZERO,
            },
        ));
    }
}
