use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::IVec2,
    prelude::{Deref, DerefMut},
    reflect::Reflect,
};

use crate::remainder::Remainder;

#[derive(Component, Default, Debug, Clone, Reflect, Deref, DerefMut)]
pub struct Position(pub IVec2);

#[derive(Bundle, Default, Clone)]
pub struct PositionBundle {
    pub position: Position,
    pub remainder: Remainder,
}

impl<V: Into<IVec2>> From<V> for PositionBundle {
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
