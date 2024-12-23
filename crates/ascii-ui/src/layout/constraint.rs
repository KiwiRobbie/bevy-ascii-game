use std::ops::RangeInclusive;

use bevy::math::UVec2;

#[derive(Debug, Clone)]
pub struct Constraint {
    pub width: Option<RangeInclusive<u32>>,
    pub height: Option<RangeInclusive<u32>>,
}

impl Constraint {
    pub fn _remove_x_bounds(&self) -> Self {
        Self {
            width: None,
            height: self.height.clone(),
        }
    }

    pub fn remove_y_bounds(&self) -> Self {
        Self {
            width: self.width.clone(),
            height: None,
        }
    }
    pub fn constrain(&self, mut size: UVec2) -> UVec2 {
        if let Some(width) = &self.width {
            size.x = *(width.start().max(&size.x).min(width.end()));
        }
        if let Some(height) = &self.height {
            size.y = *(height.start().max(&size.y).min(height.end()));
        }
        return size;
    }

    pub fn max(&self) -> UVec2 {
        let x = if let Some(x) = &self.width {
            *x.end()
        } else {
            0
        };

        let y = if let Some(y) = &self.height {
            *y.end()
        } else {
            0
        };

        UVec2 { x, y }
    }
    pub fn from_max(size: UVec2) -> Self {
        Self {
            width: Some(0..=size.x),
            height: Some(0..=size.y),
        }
    }
}
