use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    gizmos::gizmos::Gizmos,
    math::Vec2,
    render::color::Color,
};
use grid_physics::position::GridSize;
use layout::positioned::Positioned;
use mouse::ActiveMarker;

pub mod attachments;
pub mod layout;
pub mod mouse;
pub mod plugin;
pub mod render;
pub mod widgets;

pub fn debug_positions(
    mut gizmos: Gizmos,
    q_positioned: Query<&Positioned, With<ActiveMarker>>,
    grid_size: Res<GridSize>,
) {
    for positioned in q_positioned.iter() {
        let offset = positioned.offset.as_vec2() * grid_size.as_vec2() * Vec2::new(1.0, -1.0);
        let size = positioned.size.as_vec2() * grid_size.as_vec2() * Vec2::new(1.0, -1.0);
        let center = offset + 0.5 * size;

        gizmos.rect_2d(center, 0.0, size, Color::ORANGE);
    }
}
