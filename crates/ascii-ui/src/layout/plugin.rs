use bevy::{
    app::{Plugin, PostUpdate},
    ecs::schedule::IntoSystemConfigs,
    prelude::*,
};

use crate::render::UiRenderSet;

use super::{
    build_layout::{build_layout, clear_layout, propagate_data_positions},
    UiLayoutSet,
};

pub(crate) struct LayoutPlugin;
impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (clear_layout, build_layout, propagate_data_positions)
                .chain()
                .in_set(UiLayoutSet)
                .before(UiRenderSet)
                .before(TransformSystem::TransformPropagate),
        );
    }
}
