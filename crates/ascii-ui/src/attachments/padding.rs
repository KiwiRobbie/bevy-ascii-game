use bevy::{ecs::component::Component, math::UVec2};

use crate::layout::constraint::Constraint;
#[derive(Debug, Component, Default, Clone)]
pub struct Padding(pub EdgeInsets);

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
            width: if let Some(width) = &constraint.width {
                let end = width.end() - self.top - self.bottom;
                Some(*width.start()..=end)
            } else {
                None
            },
            height: if let Some(height) = &constraint.height {
                let end = height.end() - self.left - self.right;
                Some(*height.start()..=end)
            } else {
                None
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
    pub fn symmetric(horizontal: u32, vertical: u32) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }
}
