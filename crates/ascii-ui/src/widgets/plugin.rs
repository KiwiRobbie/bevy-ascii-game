use bevy::prelude::*;

use super::{Button, Checkbox, ScrollingView, TabView, TextEdit};
use crate::mouse::mouse_interaction;

pub(crate) struct WidgetPlugin;
impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PreUpdate,
            (
                Checkbox::update,
                TabView::update,
                Button::update,
                ScrollingView::update,
                TextEdit::update,
            )
                .after(mouse_interaction),
        );
    }
}
