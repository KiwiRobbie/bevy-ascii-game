use bevy::{
    app::{Plugin, PostUpdate, Update},
    ecs::schedule::IntoSystemConfigs,
    transform::TransformSystem,
};

use super::{
    actor::actor_move_system,
    collision::debug_collision_system,
    free::{
        apply_gravity_to_free, apply_velocity_to_free, obstruct_velocity, update_free_actor_state,
    },
    gravity::GravityResource,
    position::position_update_transforms_system,
    solid::{solid_move_system, update_collision_cache, SolidCollisionCache},
};

#[derive(Default)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<GravityResource>()
            .init_resource::<SolidCollisionCache>()
            .add_systems(Update, (debug_collision_system, apply_gravity_to_free))
            .add_systems(
                PostUpdate,
                (
                    update_collision_cache,
                    update_free_actor_state,
                    solid_move_system,
                    actor_move_system,
                    obstruct_velocity,
                    apply_velocity_to_free,
                    apply_gravity_to_free,
                    position_update_transforms_system,
                )
                    .chain()
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
