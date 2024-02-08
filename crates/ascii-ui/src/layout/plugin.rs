use bevy::{
    app::{Plugin, PostUpdate},
    ecs::schedule::{apply_deferred, IntoSystemConfigs},
};

use super::build_layout::{build_layout, clear_layout, propagate_data_positions};

pub struct LayoutPlugin;
impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (
                clear_layout,
                apply_deferred,
                build_layout,
                apply_deferred,
                propagate_data_positions,
            )
                .chain(),
        );
    }
}
