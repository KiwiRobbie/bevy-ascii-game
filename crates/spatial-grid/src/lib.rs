use bevy_app::{Plugin, PostUpdate};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_transform::TransformSystem;
use global_position::propagate_positions;

pub mod depth;
pub mod direction;
pub mod global_position;
pub mod grid;
pub mod position;
pub mod remainder;

pub struct PositionPropagationPlugin;
impl Plugin for PositionPropagationPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            PostUpdate,
            propagate_positions.in_set(TransformSystem::TransformPropagate),
        );
    }
}
