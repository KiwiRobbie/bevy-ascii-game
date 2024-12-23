use bevy::app::Plugin;

use self::{spatial::SpatialDebugPlugin, ui::UiDebugPlugin};

mod spatial;
mod ui;

pub(crate) use spatial::{DebugCollisions, DebugPositions};
pub(crate) use ui::DebugUi;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(SpatialDebugPlugin)
            .add_plugins(UiDebugPlugin);
    }
}
