use bevy::app::{Plugin, Update};

use super::{
    button::button_interaction_system, checkbox::checkbox_interaction_system,
    scrolling_view::scrolling_view_interaction_system, tab_view::tab_view_interaction_system,
};

pub struct WidgetPlugin;
impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                checkbox_interaction_system,
                tab_view_interaction_system,
                button_interaction_system,
                scrolling_view_interaction_system,
            ),
        );
    }
}
