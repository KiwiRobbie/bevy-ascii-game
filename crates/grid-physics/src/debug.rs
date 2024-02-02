use bevy::{
    app::{Plugin, PostUpdate},
    ecs::system::{Query, Res, Resource},
    gizmos::gizmos::Gizmos,
    prelude::{Deref, DerefMut},
    render::color::Color,
    transform::components::Transform,
};

use crate::{
    actor::Actor,
    collision::Collider,
    grid::{PhysicsGrid, PhysicsGridMember},
    position::Position,
    solid::Solid,
};

#[derive(Debug, Resource, Default, DerefMut, Deref)]
pub struct DebugCollisions(pub bool);

pub fn debug_collision_system(
    mut gizmos: Gizmos,
    q_colliders: Query<(
        &Collider,
        &Position,
        &PhysicsGridMember,
        Option<&Solid>,
        Option<&Actor>,
    )>,
    q_physics_grid: Query<(&PhysicsGrid, &Transform)>,
    enabled: Res<DebugCollisions>,
) {
    if !**enabled {
        return;
    }
    for (collider, position, grid_member, solid, actor) in q_colliders.iter() {
        let Ok((grid, transform)) = q_physics_grid.get(grid_member.grid) else {
            continue;
        };

        for shape in collider.shape.colliders() {
            let min = (position.position + shape.min).as_vec2() * grid.size.as_vec2()
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

#[derive(Debug, Resource, DerefMut, Deref, Default)]
pub struct DebugPositions(pub bool);

pub fn debug_position_system(
    mut gizmos: Gizmos,
    q_position: Query<(&Position, &PhysicsGridMember)>,
    q_physics_grid: Query<(&PhysicsGrid, &Transform)>,
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
        app.add_systems(
            PostUpdate,
            (debug_position_system, debug_collision_system), // .before(position_update_transforms_system),
        )
        .init_resource::<DebugCollisions>()
        .init_resource::<DebugPositions>();
    }
}
