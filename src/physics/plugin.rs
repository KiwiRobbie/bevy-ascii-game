use bevy::{
    app::{Plugin, PostUpdate, Update},
    ecs::schedule::{IntoSystemConfigs, SystemSet},
    transform::TransformSystem,
};

use super::{
    actor::actor_move_system,
    collision::debug_collision_system,
    free::{apply_gravity_to_free, apply_velocity_to_free},
    gravity::GravityResource,
    position::position_update_transforms_system,
    solid::solid_move_system,
};

#[derive(Default)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<GravityResource>()
            .add_systems(Update, (debug_collision_system, apply_gravity_to_free))
            .add_systems(
                PostUpdate,
                (
                    actor_move_system,
                    solid_move_system,
                    apply_velocity_to_free,
                    apply_gravity_to_free,
                    position_update_transforms_system,
                )
                    .chain()
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
