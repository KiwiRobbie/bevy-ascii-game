use bevy::{
    app::{Plugin, PostUpdate},
    ecs::system::{Query, Res, Resource},
    gizmos::gizmos::Gizmos,
    prelude::{Deref, DerefMut},
    render::color::Color,
    transform::components::Transform,
};

use crate::{
    grid::{PhysicsGridMember, SpatialGrid},
    position::Position,
    remainder::Remainder,
};

#[derive(Debug, Resource, Default, DerefMut, Deref)]
pub struct DebugCollisions(pub bool);

#[derive(Debug, Resource, DerefMut, Deref, Default)]
pub struct DebugPositions(pub bool);

pub fn debug_position_system(
    mut gizmos: Gizmos,
    q_position: Query<(&Position, Option<&Remainder>, &PhysicsGridMember)>,
    q_physics_grid: Query<(&SpatialGrid, &Transform)>,
    enabled: Res<DebugPositions>,
) {
    if !**enabled {
        return;
    }

    for (position, remainder, grid_member) in q_position.iter() {
        let Ok((grid, transform)) = q_physics_grid.get(grid_member.grid) else {
            continue;
        };
        let position = **position * grid.size.as_ivec2();
        let position = position.as_vec2() + transform.translation.truncate();

        gizmos.circle_2d(position, 5.0, Color::BLUE);
        if let Some(remainder) = remainder.map(|r| **r * grid.size.as_vec2()) {
            gizmos.circle_2d(position + remainder, 2.0, Color::RED);
        }
    }
}

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostUpdate, debug_position_system)
            .init_resource::<DebugCollisions>()
            .init_resource::<DebugPositions>();
    }
}
