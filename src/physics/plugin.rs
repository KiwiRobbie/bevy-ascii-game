use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

use super::{
    actor::actor_move_system, collision::debug_collision_shapes, position::update_transforms,
    solid::solid_move_system,
};

#[derive(Default)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                actor_move_system.before(solid_move_system),
                solid_move_system
                    .before(update_transforms)
                    .after(actor_move_system),
                update_transforms.after(solid_move_system),
                debug_collision_shapes,
            ),
        );
    }
}
