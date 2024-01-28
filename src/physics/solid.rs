use bevy::{
    prelude::*,
    utils::{EntityHashMap, EntityHashSet},
};

use super::{
    actor::{Actor, FilterActors},
    collision::{Aabb, Collider},
    direction::Direction,
    movement::Movement,
    position::{Position, PositionBundle},
};

#[derive(Component, Default)]
pub struct Solid;

#[derive(Component, Deref, DerefMut, Default)]
pub struct RidingEntities(pub EntityHashSet<Entity>);

pub type FilterSolids = (With<Solid>, Without<Actor>);

pub fn solid_move_system(
    mut commands: Commands,
    mut q_solids: Query<
        (
            Entity,
            &mut Position,
            &Collider,
            &mut Movement,
            &RidingEntities,
        ),
        FilterSolids,
    >,
    mut q_actors: Query<(Entity, &mut Position, &Collider), FilterActors>,
    mut solid_collision_cache: ResMut<SolidCollisionCache>,
) {
    for (solid, mut solid_pos, solid_collision, mut movement, riding) in q_solids.iter_mut() {
        solid_pos.remainder += movement.delta;
        movement.delta = Vec2::ZERO;
        let movement = solid_pos.remainder.round().as_ivec2();
        solid_collision_cache.collisions.remove(&solid);

        if movement.x != 0 {
            solid_pos.remainder.x -= movement.x as f32;
            solid_pos.position.x += movement.x;

            let solid_aabbs: Vec<Aabb> = solid_collision
                .shape
                .colliders_at(solid_pos.position)
                .collect();

            for (actor, mut actor_pos, actor_collision) in q_actors.iter_mut() {
                if let Some(distance) = actor_collision.overlap_distance(
                    actor_pos.position,
                    &solid_aabbs,
                    if movement.x > 0 {
                        Direction::X
                    } else {
                        Direction::NegX
                    },
                ) {
                    let distance = distance * movement.x.signum();
                    if let Some(_) = Actor::move_x(
                        distance as f32,
                        &mut actor_pos,
                        actor_collision,
                        &solid_collision_cache,
                    ) {
                        commands.entity(actor).insert(SquishedMarker);
                    }
                } else if riding.contains(&actor) {
                    if let Some(_) = Actor::move_x(
                        movement.x as f32,
                        &mut actor_pos,
                        actor_collision,
                        &solid_collision_cache,
                    ) {
                        commands.entity(actor).insert(SquishedMarker);
                    }
                }
            }
        }

        if movement.y != 0 {
            solid_pos.remainder.y -= movement.y as f32;
            solid_pos.position.y += movement.y;
        }

        // Update collision cache entry
        {
            let solid_aabbs: Vec<Aabb> = solid_collision
                .shape
                .colliders_at(solid_pos.position)
                .collect();
            solid_collision_cache.collisions.insert(solid, solid_aabbs);
        }
    }
}

#[derive(Component)]
pub struct SquishedMarker;

#[derive(Bundle, Default)]
pub struct SolidPhysicsBundle {
    pub solid: Solid,
    pub position: PositionBundle,
    pub collider: Collider,
    pub riding: RidingEntities,
}

#[derive(Resource, Debug, Default)]
pub struct SolidCollisionCache {
    pub collisions: EntityHashMap<Entity, Vec<Aabb>>,
}

pub fn update_collision_cache(
    mut solid_collision_cache: ResMut<SolidCollisionCache>,
    q_solids: Query<(Entity, &Position, &Collider), FilterSolids>,
) {
    solid_collision_cache.collisions.clear();
    for (solid, position, collider) in q_solids.iter() {
        solid_collision_cache.collisions.insert(
            solid,
            collider.shape.colliders_at(position.position).collect(),
        );
    }
}
