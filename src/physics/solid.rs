use bevy::{prelude::*, utils::EntityHashSet};

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
) {
    let mut solids: Vec<_> = q_solids.iter_mut().collect();
    for i in 0..solids.len() {
        let (before, after) = solids.split_at_mut(i);
        let (current, after) = after.split_at_mut(1);

        let (_solid, solid_pos, solid_collision, movement, riding) =
            current.iter_mut().next().unwrap();

        solid_pos.remainder += movement.delta;
        movement.delta = Vec2::ZERO;

        let movement = solid_pos.remainder.round().as_ivec2();
        if movement.x != 0 {
            solid_pos.remainder.x -= movement.x as f32;
            solid_pos.position.x += movement.x;
        }

        let solid_aabbs: Vec<Aabb> = solid_collision
            .shape
            .colliders_at(solid_pos.position)
            .collect();

        let other_solid_aabbs: Box<[_]> = before
            .iter_mut()
            .chain(after.iter_mut())
            .flat_map(|(_solid, solid_pos, solid_collision, _movement, _riding)| {
                solid_collision.shape.colliders_at(solid_pos.position)
            })
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
                if Actor::move_x(
                    distance as f32,
                    &mut actor_pos,
                    actor_collision,
                    other_solid_aabbs.iter(),
                ) {
                    commands.entity(actor).insert(SquishedMarker);
                }
            } else if riding.contains(&actor) {
                if Actor::move_x(
                    movement.x as f32,
                    &mut actor_pos,
                    actor_collision,
                    other_solid_aabbs.iter(),
                ) {
                    commands.entity(actor).insert(SquishedMarker);
                }
            }
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
