use bevy::prelude::SystemSet;

pub(crate) mod attachments;
pub(crate) mod bundle;
pub(crate) mod clear;
pub(crate) mod plugin;
pub(crate) mod widgets;

#[derive(Debug, SystemSet, Hash, PartialEq, Eq, Clone)]
pub struct UiRenderSet;
