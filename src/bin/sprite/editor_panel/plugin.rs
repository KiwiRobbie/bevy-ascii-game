use bevy::prelude::*;

use super::{
    setup::setup_ui,
    state::EditorPanelState,
    update::{toggle_menu, update_editor_shortcuts, update_editor_ui, update_position},
};
use ascii_ui::plugin::UiPlugin;

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
                    update_editor_shortcuts.before(update_editor_ui),
                    update_editor_ui,
                ),
            )
            .init_resource::<EditorPanelState>();
    }
}
