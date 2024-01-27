use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        query::{With, Without},
        system::Query,
    },
    math::IVec2,
};

use super::{
    collision::{Aabb, Collider},
    movement::Movement,
    position::{Position, PositionBundle},
    solid::{FilterSolids, Solid},
};

pub type FilterActors = (With<Actor>, Without<Solid>);

#[derive(Component, Default)]
pub struct Actor;

impl Actor {
    pub fn move_x<'a, I: Iterator<Item = &'a Aabb> + Clone>(
        amount: f32,
        actor_position: &mut Position,
        actor_collider: &Collider,
        solid_collisions: I,
    ) -> bool {
        actor_position.remainder.x += amount;
        let mut movement: i32 = actor_position.remainder.x.round() as i32;
        if movement != 0 {
            actor_position.remainder.x -= movement as f32;
            let step = movement.signum();
            while movement != 0 {
                if actor_collider.overlaps(
                    actor_position.position + IVec2::X * step,
                    solid_collisions.clone(),
                ) {
                    return true;
                } else {
                    actor_position.position.x += step;
                    movement -= step;
                }
            }
        }
        false
    }
    pub fn move_y<'a, I: Iterator<Item = &'a Aabb> + Clone>(
        amount: f32,
        actor_position: &mut Position,
        actor_collider: &Collider,
        solid_collisions: I,
    ) -> bool {
        actor_position.remainder.y += amount;
        let mut movement: i32 = actor_position.remainder.y.round() as i32;
        if movement != 0 {
            actor_position.remainder.y -= movement as f32;
            let step = movement.signum();
            while movement != 0 {
                if actor_collider.overlaps(
                    actor_position.position + IVec2::Y * step,
                    solid_collisions.clone(),
                ) {
                    return true;
                } else {
                    actor_position.position.y += step;
                    movement -= step;
                }
            }
        }
        false
    }
}

#[derive(Bundle, Default)]
pub struct ActorPhysicsBundle {
    pub actor: Actor,
    pub position: PositionBundle,
    pub collider: Collider,
    pub movement: Movement,
}

pub fn actor_move_system(
    mut q_actors: Query<(&mut Position, &mut Movement, &Collider), FilterActors>,
    mut q_solids: Query<(&mut Position, &Collider), FilterSolids>,
) {
    let solids_aabbs: Box<[_]> = q_solids
        .iter_mut()
        .flat_map(|(solid_pos, solid_collision)| {
            solid_collision.shape.colliders_at(solid_pos.position)
        })
        .collect();

    dbg!(&solids_aabbs);

    for (mut actor_position, mut actor_movement, actor_collider) in q_actors.iter_mut() {
        Actor::move_x(
            actor_movement.delta.x,
            &mut actor_position,
            actor_collider,
            solids_aabbs.into_iter(),
        );
        actor_movement.delta.x = 0.0;
    }
}
