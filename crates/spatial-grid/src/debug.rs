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
};

#[derive(Debug, Resource, Default, DerefMut, Deref)]
pub struct DebugCollisions(pub bool);

#[derive(Debug, Resource, DerefMut, Deref, Default)]
pub struct DebugPositions(pub bool);

pub fn debug_position_system(
    mut gizmos: Gizmos,
    q_position: Query<(&Position, &PhysicsGridMember)>,
    q_physics_grid: Query<(&SpatialGrid, &Transform)>,
    enabled: Res<DebugPositions>,
) {
    if !**enabled {
        return;
    }

    for (position, grid_member) in q_position.iter() {
        let Ok((grid, transform)) = q_physics_grid.get(grid_member.grid) else {
            continue;
        };
        let remainder = position.remainder * grid.size.as_vec2();
        let position = position.position * grid.size.as_ivec2();
        let position = position.as_vec2() + transform.translation.truncate();

        gizmos.circle_2d(position, 5.0, Color::BLUE);
        gizmos.circle_2d(position + remainder, 2.0, Color::RED);
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
