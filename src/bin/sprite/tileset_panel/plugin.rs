use ascii_ui::plugin::UiPlugin;
use bevy::app::{Plugin, Startup, Update};

use super::{
    painter::PainterPlugin,
    setup::setup_ui,
    state::TilesetPanelState,
    update::{
        save_tilemap_system, toggle_menu, update_list_builder, update_position,
        update_tilesets_system,
    },
};

pub struct TilesetPanelPlugin;
impl Plugin for TilesetPanelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((UiPlugin, PainterPlugin))
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    update_position,
                    toggle_menu,
                    update_list_builder,
                    update_tilesets_system,
                    save_tilemap_system,
                ),
            )
            .init_resource::<TilesetPanelState>();
    }
}
