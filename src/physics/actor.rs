use bevy::ecs::{
    bundle::Bundle,
    component::Component,
    query::{With, Without},
};

use super::{
    collision::{Aabb, Collider},
    position::Position,
    solid::Solid,
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
                if actor_collider.overlaps(solid_collisions.clone()) {
                    actor_position.position += step;
                    movement -= step;
                } else {
                    return true;
                }
            }
        }
        false
    }
    pub fn move_y(amount: f32, actor_position: &mut Position, on_collide: &dyn Fn()) {}
}

#[derive(Bundle, Default)]
pub struct ActorPhysicsBundle {
    pub actor: Actor,
    pub position: Position,
    pub collider: Collider,
}
