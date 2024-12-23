use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{With, Without},
    system::{Commands, Query, Res},
};
use bevy_time::Time;

use spatial_grid::{position::Position, remainder::Remainder};

use crate::actor::FilterActors;

use super::{
    actor::Actor,
    collision::Collider,
    gravity::{Gravity, GravityResource},
    movement::{Movement, MovementObstructed},
    solid::{FilterSolids, RidingEntities, SolidCollisionCache},
    velocity::Velocity,
};

pub fn update_obstructions(
    mut q_actors: Query<
        (&mut MovementObstructed, &Position, &Collider),
        (FilterActors, With<FreeMarker>),
    >,
    solid_collision_cache: Res<SolidCollisionCache>,
) {
    for (mut obstructed, actor_position, actor_collider) in q_actors.iter_mut() {
        *obstructed = MovementObstructed {
            x: Actor::test_move_x(
                1.0,
                actor_position,
                &Remainder::default(),
                actor_collider,
                solid_collision_cache.as_ref(),
            ),
            y: Actor::test_move_y(
                1.0,
                actor_position,
                &Remainder::default(),
                actor_collider,
                solid_collision_cache.as_ref(),
            ),
            neg_x: Actor::test_move_x(
                -1.0,
                actor_position,
                &Remainder::default(),
                actor_collider,
                solid_collision_cache.as_ref(),
            ),
            neg_y: Actor::test_move_y(
                -1.0,
                actor_position,
                &Remainder::default(),
                actor_collider,
                solid_collision_cache.as_ref(),
            ),
        };
    }
}

#[derive(Component, Clone)]
pub struct FreeMarker;

pub(super) fn obstruct_velocity(
    mut q_free_actors: Query<(&mut Velocity, &mut MovementObstructed), With<FreeMarker>>,
) {
    for (mut velocity, obstructed) in q_free_actors.iter_mut() {
        if velocity.x > 0.0 && obstructed.x.is_some() {
            velocity.x = 0.0;
        }
        if velocity.x < 0.0 && obstructed.neg_x.is_some() {
            velocity.x = 0.0;
        }
        if velocity.y > 0.0 && obstructed.y.is_some() {
            velocity.y = 0.0;
        }
        if velocity.y < 0.0 && obstructed.neg_y.is_some() {
            velocity.y = 0.0;
        }
    }
}

pub(super) fn apply_velocity_to_free(
    mut q_free_actors: Query<(&mut Movement, &Velocity), With<FreeMarker>>,
    time: Res<Time>,
) {
    for (mut actor_movement, actor_velocity) in q_free_actors.iter_mut() {
        actor_movement.add(**actor_velocity * time.delta_secs());
    }
}

pub(super) fn apply_gravity_to_free(
    mut q_free_actors: Query<(&mut Velocity, &Gravity), With<FreeMarker>>,
    res_gravity: Res<GravityResource>,
    time: Res<Time>,
) {
    for (mut actor_velocity, actor_gravity) in q_free_actors.iter_mut() {
        actor_velocity.y += res_gravity.acceleration * actor_gravity.multiplier * time.delta_secs();
    }
}

#[derive(Debug, Component, Default)]
pub struct FreeGrounded;

#[derive(Debug, Component, Default)]
pub(crate) struct FreeAirborne;

pub(super) fn update_free_actor_state(
    mut commands: Commands,
    mut q_solids: Query<&mut RidingEntities, FilterSolids>,
    q_free_actors: Query<(Entity, &Position, &Remainder, &Velocity, &Collider), With<FreeMarker>>,
    q_grounded_extra: Query<Entity, (Without<FreeMarker>, With<FreeGrounded>)>,
    solid_collision_cache: Res<SolidCollisionCache>,
) {
    for (actor, position, remainder, velocity, collider) in q_free_actors.iter() {
        for mut riding in q_solids.iter_mut() {
            riding.clear();
        }

        if velocity.y <= 0.0 {
            if let Some(solid) =
                Actor::test_move_y(-1.0, position, remainder, collider, &solid_collision_cache)
            {
                commands
                    .entity(actor)
                    .insert(FreeGrounded)
                    .remove::<FreeAirborne>();
                q_solids.get_mut(solid).unwrap().insert(actor);
                continue;
            }
        }

        commands
            .entity(actor)
            .insert(FreeAirborne)
            .remove::<FreeGrounded>();
    }

    for actor in q_grounded_extra.iter() {
        commands.entity(actor).remove::<FreeGrounded>();
    }
}
