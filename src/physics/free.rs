use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res},
    },
    time::Time,
};

use super::{
    gravity::{Gravity, GravityResource},
    movement::Movement,
    velocity::Velocity,
};

#[derive(Component)]
pub struct FreeMarker;

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
