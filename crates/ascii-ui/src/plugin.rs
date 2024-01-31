use bevy::app::Plugin;

use crate::layout::plugin::LayoutPlugin;
use crate::render::plugin::RenderPlugin;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((LayoutPlugin, RenderPlugin));
    }
}
