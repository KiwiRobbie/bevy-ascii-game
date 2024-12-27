use bevy::prelude::*;

#[derive(Component, Clone, Debug, Deref, DerefMut)]
pub struct WidgetSize(pub UVec2);
