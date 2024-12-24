use ascii_ui::plugin::UiPlugin;
use bevy::app::{Plugin, Startup, Update};

use super::{
    setup::setup_ui,
    state::EditorPanelState,
    update::{toggle_menu, update_editor_ui, update_position, update_tilesets_system},
};

pub(crate) struct TilesetPanelPlugin;
impl Plugin for TilesetPanelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((UiPlugin,))
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    update_position,
                    toggle_menu,
                    update_tilesets_system,
                    update_editor_ui,
                ),
            )
            .init_resource::<EditorPanelState>();
    }
}
