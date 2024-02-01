use bevy::{
    app::{Plugin, PostUpdate},
    ecs::{
        schedule::IntoSystemConfigs,
        system::{Query, Res, Resource},
    },
    gizmos::gizmos::Gizmos,
    prelude::{Deref, DerefMut},
    render::color::Color,
};

use crate::{
    actor::Actor,
    collision::Collider,
    position::{position_update_transforms_system, GridSize, Position},
    solid::Solid,
};

#[derive(Debug, Resource, Default, DerefMut, Deref)]
pub struct DebugCollisions(pub bool);

pub fn debug_collision_system(
    mut gizmos: Gizmos,
    q_colliders: Query<(&Collider, &Position, Option<&Solid>, Option<&Actor>)>,
    font_size: Res<GridSize>,
    enabled: Res<DebugCollisions>,
) {
    if !**enabled {
        return;
    }
    for (collider, position, solid, actor) in q_colliders.iter() {
        for shape in collider.shape.colliders() {
            let min = (position.position + shape.min).as_vec2() * font_size.as_vec2();
            let size = shape.size.as_vec2() * font_size.as_vec2();

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
    q_position: Query<&Position>,
    grid_size: Res<GridSize>,
    enabled: Res<DebugPositions>,
) {
    if !**enabled {
        return;
    }

    for position in q_position.iter() {
        let remainder = position.remainder * grid_size.as_vec2();
        let position = position.position * grid_size.as_ivec2();
        let position = position.as_vec2();

        gizmos.circle_2d(position, 5.0, Color::BLUE);
        gizmos.circle_2d(position + remainder, 2.0, Color::RED);
    }
}

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (debug_position_system, debug_collision_system)
                .before(position_update_transforms_system),
        )
        .init_resource::<DebugCollisions>()
        .init_resource::<DebugPositions>();
    }
}
