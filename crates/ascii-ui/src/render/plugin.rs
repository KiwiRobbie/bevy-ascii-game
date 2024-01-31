use bevy::app::{Plugin, Update};

use super::{
    attachments::border::border_render,
    widgets::{divider::divider_render, text::text_render},
};

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (text_render, divider_render, border_render));
    }
}
