use bevy::prelude::*;

use super::UiTheme;
pub struct ThemePlugin;
impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiTheme>();
    }
}
