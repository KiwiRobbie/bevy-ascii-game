use bevy::prelude::SystemSet;

pub(crate) mod build_layout;
pub(crate) mod constraint;
pub(crate) mod despawn_layout;
pub(crate) mod plugin;
pub mod positioned;
pub(crate) mod render_clip;
pub(crate) mod widget_layout;

#[derive(Debug, SystemSet, Hash, PartialEq, Eq, Clone)]
pub struct UiLayoutSet;

pub use despawn_layout::delete_layout_recursive;
