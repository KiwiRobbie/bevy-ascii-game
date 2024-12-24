use std::ops::RangeInclusive;

use bevy::math::UVec2;

#[derive(Debug, Clone)]
pub struct Constraint {
    pub width: Option<RangeInclusive<u32>>,
    pub height: Option<RangeInclusive<u32>>,
}

impl Constraint {
    pub fn remove_x_bounds(&self) -> Self {
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

    fn intersect_axis(
        a: &Option<RangeInclusive<u32>>,
        b: &Option<RangeInclusive<u32>>,
    ) -> Option<RangeInclusive<u32>> {
        match (a, b) {
            (None, None) => None,
            (None, Some(b)) => Some(b.clone()),
            (Some(a), None) => Some(a.clone()),
            (Some(a), Some(b)) => Some(*a.start().max(b.start())..=*a.end().min(b.end())),
        }
    }
    pub fn intersect(&self, other: &Self) -> Self {
        Self {
            width: Self::intersect_axis(&self.width, &other.width),
            height: Self::intersect_axis(&self.height, &other.height),
        }
    }
}
