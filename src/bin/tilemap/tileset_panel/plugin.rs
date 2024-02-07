use ascii_ui::plugin::UiPlugin;
use bevy::app::{Plugin, Startup, Update};

use super::{
    setup::setup_ui,
    state::TilesetPanelState,
    update::{
        tilemap_painter, toggle_menu, update_list_builder, update_position, update_tilesets,
        update_values,
    },
};

pub struct TilesetPanelPlugin;
impl Plugin for TilesetPanelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(UiPlugin)
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    update_values,
                    update_position,
                    toggle_menu,
                    update_list_builder,
                    update_tilesets,
                    tilemap_painter,
                ),
            )
            .init_resource::<TilesetPanelState>();
    }
}
