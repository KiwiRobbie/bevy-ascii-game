use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    math::{IVec2, Vec2},
};

use super::{
    collision::Collider,
    movement::{Movement, MovementObstructed},
    position::{Position, PositionBundle},
    solid::{Solid, SolidCollisionCache},
};

pub type FilterActors = (With<Actor>, Without<Solid>);

#[derive(Component, Default)]
pub struct Actor;

impl Actor {
    pub fn move_x(
        amount: f32,
        actor_position: &mut Position,
        actor_collider: &Collider,
        collision_cache: &SolidCollisionCache,
    ) -> Option<Entity> {
        actor_position.remainder.x += amount;
        let mut movement: i32 = actor_position.remainder.x.round() as i32;
        if movement != 0 {
            actor_position.remainder.x -= movement as f32;
            let step = movement.signum();
            while movement != 0 {
                if let Some(solid) = actor_collider
                    .overlaps(actor_position.position + IVec2::X * step, collision_cache)
                {
                    return Some(solid);
                } else {
                    actor_position.position.x += step;
                    movement -= step;
                }
            }
        }
        None
    }
    pub fn move_y(
        amount: f32,
        actor_position: &mut Position,
        actor_collider: &Collider,
        collision_cache: &SolidCollisionCache,
    ) -> Option<Entity> {
        actor_position.remainder.y += amount;
        let mut movement: i32 = actor_position.remainder.y.round() as i32;
        if movement != 0 {
            actor_position.remainder.y -= movement as f32;
            let step = movement.signum();
            while movement != 0 {
                if let Some(solid) = actor_collider
                    .overlaps(actor_position.position + IVec2::Y * step, collision_cache)
                {
                    return Some(solid);
                } else {
                    actor_position.position.y += step;
                    movement -= step;
                }
            }
        }
        None
    }

    pub fn test_move_x(
        amount: f32,
        actor_position: &Position,
        actor_collider: &Collider,
        collision_cache: &SolidCollisionCache,
    ) -> Option<Entity> {
        Self::move_x(
            amount,
            &mut actor_position.clone(),
            actor_collider,
            collision_cache,
        )
    }
    pub fn test_move_y(
        amount: f32,
        actor_position: &Position,
        actor_collider: &Collider,
        collision_cache: &SolidCollisionCache,
    ) -> Option<Entity> {
        Self::move_y(
            amount,
            &mut actor_position.clone(),
            actor_collider,
            collision_cache,
        )
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
    mut q_actors: Query<(Entity, &mut Position, &mut Movement, &Collider), FilterActors>,
    mut commands: Commands,
    solid_collision_cache: Res<SolidCollisionCache>,
) {
    dbg!(&solid_collision_cache);

    for (entity, mut actor_position, mut actor_movement, actor_collider) in q_actors.iter_mut() {
        let mut obstructed = MovementObstructed::default();

        if let Some(solid) = Actor::move_x(
            actor_movement.delta.x,
            &mut actor_position,
            actor_collider,
            &solid_collision_cache,
        ) {
            if actor_movement.delta.x > 0.0 {
                obstructed.x = Some(solid);
            } else {
                obstructed.neg_x = Some(solid);
            }
        }
        if let Some(solid) = Actor::move_y(
            actor_movement.delta.y,
            &mut actor_position,
            actor_collider,
            &solid_collision_cache,
        ) {
            if actor_movement.delta.y > 0.0 {
                obstructed.y = Some(solid);
            } else {
                obstructed.neg_y = Some(solid);
            }
        }
        commands.entity(entity).insert(obstructed);
        actor_movement.delta = Vec2::ZERO;
    }
}
