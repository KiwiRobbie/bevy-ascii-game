use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    gizmos::gizmos::Gizmos,
    math::Vec2,
    render::color::Color,
    transform::components::Transform,
};
use grid_physics::grid::{PhysicsGrid, PhysicsGridMember};
use layout::positioned::Positioned;
use mouse::ActiveMarker;

pub mod attachments;
pub mod layout;
pub mod mouse;
pub mod plugin;
pub mod render;
pub mod widget_builder;
pub mod widgets;

pub fn debug_positions(
    mut gizmos: Gizmos,
    q_positioned: Query<(&Positioned, &PhysicsGridMember), With<ActiveMarker>>,
    q_physics_grid: Query<(&PhysicsGrid, &Transform)>,
) {
    for (positioned, grid_member) in q_positioned.iter() {
        let Ok((grid, transform)) = q_physics_grid.get(grid_member.grid) else {
            continue;
        };

        let offset = positioned.offset.as_vec2() * grid.size.as_vec2() * Vec2::new(1.0, -1.0)
            + transform.translation.truncate();
        let size = positioned.size.as_vec2() * grid.size.as_vec2() * Vec2::new(1.0, -1.0);
        let center = offset + 0.5 * size;

        gizmos.rect_2d(center, 0.0, size, Color::ORANGE);
    }
}
