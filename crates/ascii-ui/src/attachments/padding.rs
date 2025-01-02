use bevy::prelude::*;

use crate::layout::constraint::Constraint;
#[derive(Debug, Component, Default, Clone)]
pub struct Padding(pub EdgeInsets);
impl Padding {
    pub fn symmetric(x: u32, y: u32) -> Self {
        Self(EdgeInsets::symmetric(x, y))
    }
    pub fn all(amount: u32) -> Self {
        Self(EdgeInsets {
            top: amount,
            bottom: amount,
            left: amount,
            right: amount,
        })
    }
    pub fn total(&self) -> UVec2 {
        UVec2::new(self.0.left + self.0.right, self.0.top + self.0.bottom)
    }
}

#[derive(Debug, Default, Clone)]
pub struct EdgeInsets {
    pub top: u32,
    pub bottom: u32,
    pub left: u32,
    pub right: u32,
}

impl EdgeInsets {
    pub(crate) fn shrink_constraint(&self, constraint: &Constraint) -> Constraint {
        // TODO: Handle to small

        Constraint {
            width: match &constraint.width {
                Some(w) => Some(*w.start()..=(w.end() - self.left - self.right)),
                _ => None,
            },
            height: match &constraint.height {
                Some(h) => Some(*h.start()..=(h.end() - self.top - self.bottom)),
                _ => None,
            },
        }
    }
    pub fn inflate(&self, size: UVec2) -> UVec2 {
        UVec2 {
            x: size.x + self.left + self.right,
            y: size.y + self.top + self.bottom,
        }
    }
}

impl EdgeInsets {
    pub fn symmetric(x: u32, y: u32) -> Self {
        Self {
            top: y,
            bottom: y,
            left: x,
            right: x,
        }
    }
}
