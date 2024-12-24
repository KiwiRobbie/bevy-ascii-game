use bevy::app::Plugin;

use self::brush::BrushPlugin;

pub(crate) mod brush;

pub(crate) struct PainterPlugin;
impl Plugin for PainterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(BrushPlugin);
    }
}
