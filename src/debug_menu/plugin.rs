use ascii_ui::plugin::UiPlugin;
use bevy::app::{Plugin, Startup, Update};

use super::{
    setup::setup_ui,
    state::DebugMenuState,
    update::{toggle_menu, update},
};

pub struct DebugMenuPlugin;
impl Plugin for DebugMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(UiPlugin)
            .add_systems(Startup, setup_ui)
            .add_systems(Update, (update, toggle_menu))
            .init_resource::<DebugMenuState>();
    }
}
