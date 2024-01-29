use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    time::Time,
};

use super::{
    actor::Actor,
    collision::Collider,
    gravity::{Gravity, GravityResource},
    movement::{Movement, MovementObstructed},
    position::Position,
    solid::{FilterSolids, RidingEntities, SolidCollisionCache},
    velocity::Velocity,
};

#[derive(Component, Clone)]
pub struct FreeMarker;

pub fn obstruct_velocity(
    mut q_free_actors: Query<(&mut Velocity, &MovementObstructed), With<FreeMarker>>,
) {
    for (mut velocity, obstructed) in q_free_actors.iter_mut() {
        if velocity.velocity.x > 0.0 && obstructed.x.is_some() {
            velocity.velocity.x = 0.0;
        }
        if velocity.velocity.x < 0.0 && obstructed.neg_x.is_some() {
            velocity.velocity.x = 0.0;
        }
        if velocity.velocity.y > 0.0 && obstructed.y.is_some() {
            velocity.velocity.y = 0.0;
        }
        if velocity.velocity.y < 0.0 && obstructed.neg_y.is_some() {
            velocity.velocity.y = 0.0;
        }
    }
}

pub fn apply_velocity_to_free(
    mut q_free_actors: Query<(&mut Movement, &Velocity), With<FreeMarker>>,
    time: Res<Time>,
) {
    for (mut actor_movement, actor_velocity) in q_free_actors.iter_mut() {
        actor_movement.add(actor_velocity.velocity * time.delta_seconds());
    }
}

pub fn apply_gravity_to_free(
    mut q_free_actors: Query<(&mut Velocity, &Gravity), With<FreeMarker>>,
    res_gravity: Res<GravityResource>,
    time: Res<Time>,
) {
    for (mut actor_velocity, actor_gravity) in q_free_actors.iter_mut() {
        actor_velocity.velocity.y +=
            res_gravity.acceleration * actor_gravity.multiplier * time.delta_seconds();
    }
}

#[derive(Debug, Component, Default)]
pub struct FreeGrounded;

#[derive(Debug, Component, Default)]
pub struct FreeAirborne;

pub fn update_free_actor_state(
    mut commands: Commands,
    mut q_solids: Query<&mut RidingEntities, FilterSolids>,
    q_free_actors: Query<(Entity, &Position, &Velocity, &Collider), With<FreeMarker>>,
    solid_collision_cache: Res<SolidCollisionCache>,
) {
    for (actor, position, velocity, collider) in q_free_actors.iter() {
        for mut riding in q_solids.iter_mut() {
            riding.clear();
        }

        if velocity.velocity.y <= 0.0 {
            if let Some(solid) =
                Actor::test_move_y(-1.0, position, collider, &solid_collision_cache)
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
}
