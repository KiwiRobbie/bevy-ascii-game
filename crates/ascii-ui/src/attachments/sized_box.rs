use bevy::prelude::*;

use crate::layout::constraint::Constraint;

#[derive(Debug, Component, Clone, Default)]
pub struct SizedBox {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl SizedBox {
    pub fn vertical(height: u32) -> Self {
        Self {
            width: None,
            height: Some(height),
        }
    }
    pub fn horizontal(width: u32) -> Self {
        Self {
            width: Some(width),
            height: None,
        }
    }
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: Some(width),
            height: Some(height),
        }
    }
    pub fn as_max_constraint(&self) -> Constraint {
        Constraint {
            width: self.width.map(|x| 0..=x),
            height: self.height.map(|x| 0..=x),
        }
    }
}
