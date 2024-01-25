use bevy::{
    app::{Plugin, PostUpdate, Update},
    ecs::schedule::IntoSystemConfigs,
};

use super::{
    actor::FilterActors, position::update_transforms, solid::solid_move_system, solid::FilterSolids,
};

#[derive(Default)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                solid_move_system.before(update_transforms),
                update_transforms.after(solid_move_system),
            ),
        );
    }
}
