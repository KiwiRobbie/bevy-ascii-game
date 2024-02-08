use bevy::{
    app::{Plugin, PreUpdate, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::mouse::mouse_interaction;

use super::{
    button::button_interaction_system, checkbox::checkbox_interaction_system,
    scrolling_view::scrolling_view_interaction_system, tab_view::tab_view_interaction_system,
};

pub struct WidgetPlugin;
impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PreUpdate,
            (
                checkbox_interaction_system,
                tab_view_interaction_system,
                button_interaction_system,
                scrolling_view_interaction_system,
            )
                .after(mouse_interaction),
        );
    }
}
