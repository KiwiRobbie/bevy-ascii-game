use ascii_ui::plugin::UiPlugin;
use bevy::app::{Plugin, Startup, Update};

use super::{
    painter::PainterPlugin,
    setup::setup_ui,
    state::TilesetPanelState,
    update::{toggle_menu, update_list_builder, update_position, update_tilesets},
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
                    update_tilesets,
                ),
            )
            .init_resource::<TilesetPanelState>();
    }
}
