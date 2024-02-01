use bevy::app::{Plugin, Update};

use super::checkbox::checkbox_interaction_system;

pub struct WidgetPlugin;
impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, checkbox_interaction_system);
    }
}
