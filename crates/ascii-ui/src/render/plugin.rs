use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{apply_deferred, IntoSystemConfigs},
};

use super::{
    attachments::border::border_render,
    clear::clear_sprites,
    widgets::{divider::divider_render, text::text_render},
};

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                clear_sprites,
                apply_deferred,
                (text_render, divider_render, border_render),
            )
                .chain(),
        );
    }
}
