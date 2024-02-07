use bevy::app::Plugin;

use self::brush::BrushPlugin;

pub mod brush;

pub struct PainterPlugin;
impl Plugin for PainterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(BrushPlugin);
    }
}
