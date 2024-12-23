use bevy::app::Plugin;

use crate::{layout::plugin::LayoutPlugin, widgets::plugin::WidgetPlugin};
use crate::{mouse::InteractionPlugin, render::plugin::RenderPlugin};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((LayoutPlugin, RenderPlugin, InteractionPlugin, WidgetPlugin));
    }
}
