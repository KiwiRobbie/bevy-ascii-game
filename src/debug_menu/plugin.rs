use ascii_ui::plugin::UiPlugin;
use bevy::app::{Plugin, Update};

use super::{
    inspector::InspectorPlugin,
    state::DebugMenuState,
    update::{toggle_menu, update_position},
};

pub struct DebugMenuPlugin;
impl Plugin for DebugMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((UiPlugin, InspectorPlugin))
            .add_systems(Update, (update_position, toggle_menu))
            .init_resource::<DebugMenuState>();
    }
}
