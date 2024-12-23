use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{bundle::Bundle, component::Component, world::Mut};
use bevy_math::{IVec2, Vec2};
use bevy_reflect::Reflect;

use crate::remainder::Remainder;

#[derive(Component, Default, Debug, Clone, Copy, Reflect, Deref, DerefMut)]
pub struct Position(pub IVec2);

impl Position {
    pub(crate) const ZERO: Self = Self(IVec2::ZERO);
    pub fn offset(self, offset: IVec2) -> Self {
        Self(*self + offset)
    }
}

#[derive(Bundle, Default, Clone)]
pub struct SpatialBundle {
    pub position: Position,
    pub remainder: Remainder,
}

impl SpatialBundle {
    pub(crate) fn offset(&mut self, delta: Vec2) {
        *self.remainder += delta;
        let delta = self.remainder.round();
        *self.remainder -= delta;
        *self.position += delta.as_ivec2();
    }
}
pub trait SpatialTraits {
    fn offset(&mut self, delta: Vec2);
}
impl SpatialTraits for (&mut Position, &mut Remainder) {
    fn offset(&mut self, delta: Vec2) {
        **self.1 += delta;
        let delta = self.1.round();
        **self.1 -= delta;
        **self.0 += delta.as_ivec2();
    }
}
impl<'a> SpatialTraits for (Mut<'a, Position>, Mut<'a, Remainder>) {
    fn offset(&mut self, delta: Vec2) {
        **self.1 += delta;
        let delta = self.1.round();
        **self.1 -= delta;
        **self.0 += delta.as_ivec2();
    }
}
impl From<IVec2> for SpatialBundle {
    fn from(value: IVec2) -> Self {
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
