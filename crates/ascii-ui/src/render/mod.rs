use bevy::prelude::SystemSet;

mod attachments;
mod bundle;
mod clear;
pub(super) mod plugin;
mod theme;
mod widgets;

#[derive(Debug, SystemSet, Hash, PartialEq, Eq, Clone)]
pub struct UiRenderSet;

pub use bundle::RenderBundle;
