use super::{
    collision::Collider,
    movement::{Movement, MovementObstructed},
    solid::{Solid, SolidCollisionCache},
};
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    query::{With, Without},
    system::{Commands, Query, Res},
};
use bevy_math::{IVec2, Vec2};
use spatial_grid::{
    position::{Position, SpatialBundle},
    remainder::Remainder,
};

pub type FilterActors = (With<Actor>, Without<Solid>);

#[derive(Component, Default, Clone)]
pub struct Actor;

impl Actor {
    pub fn move_x(
        amount: f32,
        actor_position: &mut Position,
        actor_remainder: &mut Remainder,
        actor_collider: &Collider,
        collision_cache: &SolidCollisionCache,
    ) -> Option<Entity> {
        actor_remainder.x += amount;
        let mut movement: i32 = actor_remainder.x.round() as i32;
        if movement != 0 {
            actor_remainder.x -= movement as f32;
            let step = movement.signum();
            while movement != 0 {
                if let Some(solid) =
                    actor_collider.overlaps(**actor_position + IVec2::X * step, collision_cache)
                {
                    return Some(solid);
                } else {
                    actor_position.x += step;
                    movement -= step;
                }
            }
        }
        None
    }
    pub fn move_y(
        amount: f32,
        actor_position: &mut Position,
        actor_remainder: &mut Remainder,
        actor_collider: &Collider,
        collision_cache: &SolidCollisionCache,
    ) -> Option<Entity> {
        actor_remainder.y += amount;
        let mut movement: i32 = actor_remainder.y.round() as i32;
        if movement != 0 {
            actor_remainder.y -= movement as f32;
            let step = movement.signum();
            while movement != 0 {
                if let Some(solid) =
                    actor_collider.overlaps(**actor_position + IVec2::Y * step, collision_cache)
                {
                    return Some(solid);
                } else {
                    actor_position.y += step;
                    movement -= step;
                }
            }
        }
        None
    }

    pub fn test_move_x(
        amount: f32,
        actor_position: &Position,
        actor_remainder: &Remainder,
        actor_collider: &Collider,
        collision_cache: &SolidCollisionCache,
    ) -> Option<Entity> {
        Self::move_x(
            amount,
            &mut actor_position.clone(),
            &mut actor_remainder.clone(),
            actor_collider,
            collision_cache,
        )
    }
    pub fn test_move_y(
        amount: f32,
        actor_position: &Position,
        actor_remainder: &Remainder,
        actor_collider: &Collider,
        collision_cache: &SolidCollisionCache,
    ) -> Option<Entity> {
        Self::move_y(
            amount,
            &mut actor_position.clone(),
            &mut actor_remainder.clone(),
            actor_collider,
            collision_cache,
        )
    }
}

#[derive(Bundle, Default, Clone)]
pub struct ActorPhysicsBundle {
    pub actor: Actor,
    pub position: SpatialBundle,
    pub collider: Collider,
    pub movement: Movement,
}

pub fn actor_move_system(
    mut q_actors: Query<
        (
            Entity,
            &mut Position,
            &mut Remainder,
            &mut Movement,
            &Collider,
        ),
        FilterActors,
    >,
    mut commands: Commands,
    solid_collision_cache: Res<SolidCollisionCache>,
) {
    for (entity, mut actor_position, mut actor_remainder, mut actor_movement, actor_collider) in
        q_actors.iter_mut()
    {
        let mut obstructed = MovementObstructed::default();

        if let Some(solid) = Actor::move_x(
            actor_movement.delta.x,
            &mut actor_position,
            &mut actor_remainder,
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
            &mut actor_remainder,
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
