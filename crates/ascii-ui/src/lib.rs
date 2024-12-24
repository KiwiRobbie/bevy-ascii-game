pub mod attachments;
pub mod layout;
pub mod list_widget;
pub mod mouse;
pub mod plugin;
pub(crate) mod render;
pub mod widget_builder;
pub mod widgets;

#[derive(Debug, Clone, bevy::reflect::Reflect, Default)]
pub enum FlexDirection {
    #[default]
    Horizontal,
    Vertical,
}
