use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{bundle::Bundle, component::Component};
use bevy_math::IVec2;
use bevy_reflect::Reflect;

use crate::remainder::Remainder;

#[derive(Component, Default, Debug, Clone, Reflect, Deref, DerefMut)]
pub struct Position(pub IVec2);

#[derive(Bundle, Default, Clone)]
pub struct SpatialBundle {
    pub position: Position,
    pub remainder: Remainder,
}

impl<V: Into<IVec2>> From<V> for SpatialBundle {
    fn from(value: V) -> Self {
        Self {
            position: Position(value.into()),
            ..Default::default()
        }
    }
}
impl<V: Into<IVec2>> From<V> for Position {
    fn from(value: V) -> Self {
        Self(value.into())
    }
}
