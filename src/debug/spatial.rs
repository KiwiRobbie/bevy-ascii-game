use bevy::{
    app::{Plugin, PostUpdate},
    color::palettes::css::{BLUE, GREEN, RED},
    ecs::{
        schedule::IntoSystemConfigs,
        system::{Query, Res, Resource},
    },
    gizmos::gizmos::Gizmos,
    prelude::{Deref, DerefMut},
    transform::components::Transform,
};

use spatial_grid::{
    grid::{PhysicsGridMember, SpatialGrid},
    position::Position,
    remainder::Remainder,
};

use grid_physics::{actor::Actor, collision::Collider, solid::Solid, velocity::Velocity};

use crate::physics_grids::grid_translate;

pub(crate) struct SpatialDebugPlugin;
impl Plugin for SpatialDebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (
                debug_position_system.run_if(|enabled: Res<DebugPositions>| **enabled),
                debug_collision_system.run_if(|enabled: Res<DebugCollisions>| **enabled),
            ),
        )
        .init_resource::<DebugCollisions>()
        .init_resource::<DebugPositions>();
    }
}

#[derive(Debug, Resource, Default, DerefMut, Deref)]
pub(crate) struct DebugCollisions(pub(crate) bool);

#[derive(Debug, Resource, DerefMut, Deref, Default)]
pub(crate) struct DebugPositions(pub(crate) bool);

fn debug_collision_system(
    mut gizmos: Gizmos,
    q_colliders: Query<(
        &Collider,
        &Position,
        &PhysicsGridMember,
        Option<&Solid>,
        Option<&Actor>,
    )>,
    q_physics_grid: Query<(&SpatialGrid, &Transform, &Position)>,
) {
    for (collider, position, grid_member, solid, actor) in q_colliders.iter() {
        let Ok((grid, transform, grid_position)) = q_physics_grid.get(grid_member.grid) else {
            continue;
        };

        for shape in collider.shape.shapes.iter() {
            let min = (**position + shape.start - **grid_position).as_vec2() * grid.step.as_vec2()
                + transform.translation.truncate();
            let size = shape.size.as_vec2() * grid.step.as_vec2();

            let center = min + 0.5 * size;

            if solid.is_some() {
                gizmos.rect_2d(center, size, GREEN);
            } else if actor.is_some() {
                gizmos.rect_2d(center, size, RED);
            }
        }
    }
}

fn debug_position_system(
    mut gizmos: Gizmos,
    q_position: Query<(
        &Position,
        Option<&Remainder>,
        Option<&Velocity>,
        &PhysicsGridMember,
    )>,
    q_physics_grid: Query<(&SpatialGrid, &Transform, &Position)>,
) {
    for (position, remainder, velocity, grid_member) in q_position.iter() {
        let Ok((grid, transform, grid_position)) = q_physics_grid.get(grid_member.grid) else {
            continue;
        };
        let position = (**position - **grid_position) * grid.step.as_ivec2();
        let position = position.as_vec2() + transform.translation.truncate();

        gizmos.circle_2d(position, 5.0, BLUE);

        let position = if let Some(remainder) = remainder {
            let remainder = **remainder * grid.step.as_vec2();
            gizmos.circle_2d(position + remainder, 2.0, RED);
            position + remainder
        } else {
            position
        };

        if let Some(velocity) = velocity {
            gizmos.line_2d(position, position + **velocity, GREEN);
        }
    }
}
