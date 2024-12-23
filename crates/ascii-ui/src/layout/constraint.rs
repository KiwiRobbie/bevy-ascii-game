use std::ops::RangeInclusive;

use bevy::math::UVec2;

#[derive(Debug, Clone)]
pub(crate) struct Constraint {
    pub(crate) width: Option<RangeInclusive<u32>>,
    pub(crate) height: Option<RangeInclusive<u32>>,
}

impl Constraint {
    pub(crate) fn remove_x_bounds(&self) -> Self {
        Self {
            width: None,
            height: self.height.clone(),
        }
    }

    pub(crate) fn remove_y_bounds(&self) -> Self {
        Self {
            width: self.width.clone(),
            height: None,
        }
    }
    pub(crate) fn constrain(&self, mut size: UVec2) -> UVec2 {
        if let Some(width) = &self.width {
            size.x = *(width.start().clamp(&size.x, width.end()));
        }
        if let Some(height) = &self.height {
            size.y = *(height.start().clamp(&size.y, height.end()));
        }
        return size;
    }

    pub(crate) fn max(&self) -> UVec2 {
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
    pub(crate) fn from_max(size: UVec2) -> Self {
        Self {
            width: Some(0..=size.x),
            height: Some(0..=size.y),
        }
    }
}
