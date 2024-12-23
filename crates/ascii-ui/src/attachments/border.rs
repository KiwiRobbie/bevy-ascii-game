use bevy::{ecs::component::Component, math::UVec2};

use super::padding::{EdgeInsets, Padding};

#[derive(Debug, Component, Clone)]
pub struct Border {
    pub(crate) top: Option<char>,
    pub(crate) bottom: Option<char>,
    pub(crate) left: Option<char>,
    pub(crate) right: Option<char>,

    pub(crate) corners: [Option<char>; 4],
}

impl Border {
    pub fn symmetric(
        horizontal: Option<char>,
        vertical: Option<char>,
        corners: [Option<char>; 4],
    ) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
            corners,
        }
    }
    fn sides(pos: UVec2, size: UVec2) -> (bool, bool, bool, bool) {
        let l = pos.x == 0;
        let r = pos.x == size.x - 1;
        let t = pos.y == 0;
        let b = pos.y == size.y - 1;
        return (l, r, t, b);
    }
    pub fn top(character: char) -> Self {
        Self {
            top: Some(character),
            bottom: None,
            left: None,
            right: None,
            corners: [Some(character), Some(character), None, None],
        }
    }
    pub fn bottom(character: char) -> Self {
        Self {
            top: None,
            bottom: Some(character),
            left: None,
            right: None,
            corners: [None, None, Some(character), Some(character)],
        }
    }
    pub fn left(character: char) -> Self {
        Self {
            top: None,
            bottom: None,
            left: Some(character),
            right: None,
            corners: [Some(character), None, None, Some(character)],
        }
    }
    pub fn right(character: char) -> Self {
        Self {
            top: None,
            bottom: None,
            left: None,
            right: Some(character),
            corners: [None, Some(character), Some(character), None],
        }
    }

    pub(crate) fn create_data(&self, size: UVec2) -> Vec<String> {
        (0..size.y)
            .map(|y| {
                (0..size.x)
                    .map(|x| {
                        (match Self::sides(UVec2 { x, y }, size) {
                            (true, false, true, false) => self.corners[0].or(self.left),
                            (false, true, true, false) => self.corners[1].or(self.right),
                            (false, true, false, true) => self.corners[2].or(self.left),
                            (true, false, false, true) => self.corners[3].or(self.right),
                            (true, false, false, false) => self.left,
                            (false, true, false, false) => self.right,
                            (false, false, true, false) => self.top,
                            (false, false, false, true) => self.bottom,
                            _ => None,
                        })
                        .unwrap_or(' ')
                    })
                    .collect()
            })
            .collect()
    }
}

impl Border {
    pub fn padded(self) -> (Padding, Self) {
        (
            Padding(EdgeInsets {
                top: self.top.is_some() as u32,
                bottom: self.bottom.is_some() as u32,
                left: self.left.is_some() as u32,
                right: self.right.is_some() as u32,
            }),
            self,
        )
    }
}
