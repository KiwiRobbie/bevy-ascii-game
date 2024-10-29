use bevy::{
    app::{Plugin, PostUpdate},
    ecs::{
        schedule::IntoSystemConfigs,
        system::{Query, Res, Resource},
    },
    gizmos::gizmos::Gizmos,
    prelude::{Deref, DerefMut},
    render::color::Color,
    transform::components::Transform,
};

use spatial_grid::{
    grid::{PhysicsGridMember, SpatialGrid},
    position::Position,
    remainder::Remainder,
};

use grid_physics::{actor::Actor, collision::Collider, solid::Solid, velocity::Velocity};

pub struct SpatialDebugPlugin;
impl Plugin for SpatialDebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (
                debug_position_system.run_if(|enabled: Res<DebugPositions>| **enabled),
                debug_collision_system.run_if(|enabled: Res<DebugCollisions>| **enabled),
            ), // .before(position_update_transforms_system),
        )
        .init_resource::<DebugCollisions>()
        .init_resource::<DebugPositions>();
    }
}

#[derive(Debug, Resource, Default, DerefMut, Deref)]
pub struct DebugCollisions(pub bool);

#[derive(Debug, Resource, DerefMut, Deref, Default)]
pub struct DebugPositions(pub bool);

fn debug_collision_system(
    mut gizmos: Gizmos,
    q_colliders: Query<(
        &Collider,
        &Position,
        &PhysicsGridMember,
        Option<&Solid>,
        Option<&Actor>,
    )>,
    q_physics_grid: Query<(&SpatialGrid, &Transform)>,
) {
    for (collider, position, grid_member, solid, actor) in q_colliders.iter() {
        let Ok((grid, transform)) = q_physics_grid.get(grid_member.grid) else {
            continue;
        };

        for shape in collider
            .shape
            .shapes
            .iter()
            .filter_map(|shape| match shape {
                grid_physics::collision::CollisionShape::Aabb(aabb) => Some(aabb),
                grid_physics::collision::CollisionShape::HalfPlane(half_plane) => None,
            })
        {
            let min = (**position + shape.start).as_vec2() * grid.size.as_vec2()
                + transform.translation.truncate();
            let size = shape.size.as_vec2() * grid.size.as_vec2();

            let center = min + 0.5 * size;

            if solid.is_some() {
                gizmos.rect_2d(center, 0.0, size, Color::GREEN);
            } else if actor.is_some() {
                gizmos.rect_2d(center, 0.0, size, Color::RED);
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
    q_physics_grid: Query<(&SpatialGrid, &Transform)>,
) {
    for (position, remainder, velocity, grid_member) in q_position.iter() {
        let Ok((grid, transform)) = q_physics_grid.get(grid_member.grid) else {
            continue;
        };
        let position = **position * grid.size.as_ivec2();
        let position = position.as_vec2() + transform.translation.truncate();

        gizmos.circle_2d(position, 5.0, Color::BLUE);

        let position = if let Some(remainder) = remainder {
            let remainder = **remainder * grid.size.as_vec2();
            gizmos.circle_2d(position + remainder, 2.0, Color::RED);
            position + remainder
        } else {
            position
        };

        if let Some(velocity) = velocity {
            gizmos.line_2d(position, position + **velocity, Color::GREEN);
        }
    }
}
