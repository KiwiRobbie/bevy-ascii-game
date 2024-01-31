use attachments::border::Border;
use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    gizmos::gizmos::Gizmos,
    math::{IVec2, UVec2, Vec2},
    render::color::Color,
};
use glyph_render::glyph_render_plugin::{GlyphSprite, GlyphTexture};
use grid_physics::position::{GridSize, Position};
use layout::positioned::Positioned;

pub mod attachments;
pub mod layout;
pub mod plugin;
pub mod render;
pub mod widgets;

pub fn debug_positions(
    mut gizmos: Gizmos,
    q_positioned: Query<&Positioned>,
    grid_size: Res<GridSize>,
) {
    for positioned in q_positioned.iter() {
        let offset = positioned.offset.as_vec2() * grid_size.as_vec2() * Vec2::new(1.0, -1.0);
        let size = positioned.size.as_vec2() * grid_size.as_vec2() * Vec2::new(1.0, -1.0);
        let center = offset + 0.5 * size;

        gizmos.rect_2d(center, 0.0, size, Color::ORANGE);
    }
}
