use bevy::app::{Plugin, Update};

use crate::{
    attachments::Root,
    mouse::InteractionPlugin,
    render::plugin::RenderPlugin,
    widgets::{column::Column, divider::Divider, plugin::WidgetPlugin, text::Text},
};
use crate::{layout::plugin::LayoutPlugin, widgets::container::Container};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((LayoutPlugin, RenderPlugin, InteractionPlugin, WidgetPlugin));
    }
}

pub struct UiTypesPlugin;
impl Plugin for UiTypesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Container>()
            .register_type::<Divider>()
            .register_type::<Column>()
            .register_type::<Root>()
            .register_type::<Text>();
    }
}
