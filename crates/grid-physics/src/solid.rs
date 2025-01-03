use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::{Entity, EntityHashMap, EntityHashSet},
    query::{With, Without},
    system::{Commands, Query, ResMut, Resource},
};
use bevy_math::Vec2;

use spatial_grid::{
    direction::Direction,
    position::{Position, SpatialBundle},
    remainder::Remainder,
};

use crate::collision::Aabb;

use super::{
    actor::{Actor, FilterActors},
    collision::Collider,
    movement::Movement,
};

#[derive(Component, Default)]
pub struct Solid;

#[derive(Component, Deref, DerefMut, Default)]
pub struct RidingEntities(pub(crate) EntityHashSet);

pub(crate) type FilterSolids = (With<Solid>, Without<Actor>);

pub(crate) fn solid_move_system(
    mut commands: Commands,
    mut q_solids: Query<
        (
            Entity,
            &mut Position,
            &mut Remainder,
            &Collider,
            &mut Movement,
            &RidingEntities,
        ),
        FilterSolids,
    >,
    mut q_actors: Query<(Entity, &mut Position, &mut Remainder, &Collider), FilterActors>,
    mut solid_collision_cache: ResMut<SolidCollisionCache>,
) {
    for (solid, mut solid_position, mut solid_remainder, solid_collision, mut movement, riding) in
        q_solids.iter_mut()
    {
        **solid_remainder += movement.delta;
        movement.delta = Vec2::ZERO;
        let movement = solid_remainder.round().as_ivec2();
        solid_collision_cache.collisions.remove(&solid);

        if movement.x != 0 {
            solid_remainder.x -= movement.x as f32;
            solid_position.x += movement.x;

            let solid_aabbs: Vec<Aabb> = solid_collision.shape.iter_at(**solid_position).collect();

            for (actor, mut actor_position, mut actor_remainder, actor_collision) in
                q_actors.iter_mut()
            {
                if let Some(distance) = actor_collision.overlap_distance(
                    **actor_position,
                    &solid_aabbs,
                    if movement.x > 0 {
                        Direction::PosX
                    } else {
                        Direction::NegX
                    },
                ) {
                    let distance = distance * movement.x.signum();
                    if Actor::move_x(
                        distance as f32,
                        &mut actor_position,
                        &mut actor_remainder,
                        actor_collision,
                        &solid_collision_cache,
                    )
                    .is_some()
                    {
                        commands.entity(actor).insert(SquishedMarker);
                    }
                } else if riding.contains(&actor) {
                    if Actor::move_x(
                        movement.x as f32,
                        &mut actor_position,
                        &mut actor_remainder,
                        actor_collision,
                        &solid_collision_cache,
                    )
                    .is_some()
                    {
                        commands.entity(actor).insert(SquishedMarker);
                    }
                }
            }
        }

        if movement.y != 0 {
            solid_remainder.y -= movement.y as f32;
            solid_position.y += movement.y;
        }

        // Update collision cache entry
        {
            let solid_aabbs: Vec<Aabb> = solid_collision.shape.iter_at(**solid_position).collect();
            solid_collision_cache.collisions.insert(solid, solid_aabbs);
        }
    }
}

#[derive(Component)]
pub(crate) struct SquishedMarker;

#[derive(Bundle, Default)]
pub struct SolidPhysicsBundle {
    pub solid: Solid,
    pub position: SpatialBundle,
    pub collider: Collider,
    pub riding: RidingEntities,
}

#[derive(Resource, Debug, Default)]
pub struct SolidCollisionCache {
    pub(crate) collisions: EntityHashMap<Vec<Aabb>>,
}

pub(crate) fn update_collision_cache(
    mut solid_collision_cache: ResMut<SolidCollisionCache>,
    q_solids: Query<(Entity, &Position, &Collider), FilterSolids>,
) {
    solid_collision_cache.collisions.clear();
    for (solid, position, collider) in q_solids.iter() {
        solid_collision_cache
            .collisions
            .insert(solid, collider.shape.iter_at(**position).collect());
    }
}
