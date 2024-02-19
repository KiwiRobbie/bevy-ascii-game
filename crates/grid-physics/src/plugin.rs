use bevy_app::{Plugin, PostUpdate};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_transform::TransformSystem;

use crate::sets::{physics_systems_enabled, EnablePhysicsSystems};

use super::{
    actor::actor_move_system,
    free::{
        apply_gravity_to_free, apply_velocity_to_free, obstruct_velocity, update_free_actor_state,
    },
    gravity::GravityResource,
    solid::{solid_move_system, update_collision_cache, SolidCollisionCache},
};

#[derive(Default)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_resource::<GravityResource>()
            .init_resource::<SolidCollisionCache>()
            .init_resource::<EnablePhysicsSystems>()
            .add_systems(
                PostUpdate,
                ((
                    update_collision_cache,
                    update_free_actor_state,
                    solid_move_system,
                    actor_move_system,
                    obstruct_velocity,
                    apply_velocity_to_free,
                    apply_gravity_to_free,
                )
                    .chain()
                    .run_if(physics_systems_enabled),)
                    .chain()
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
