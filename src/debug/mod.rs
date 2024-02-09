use bevy::app::Plugin;

use self::spatial::SpatialDebugPlugin;

mod spatial;
pub use spatial::{DebugCollisions, DebugPositions};

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(SpatialDebugPlugin);
    }
}
