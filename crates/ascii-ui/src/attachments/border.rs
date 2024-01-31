use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::UVec2,
};

use super::padding::{EdgeInsets, Padding};

#[derive(Debug, Component, Clone)]
pub struct Border {
    pub top: Option<char>,
    pub bottom: Option<char>,
    pub left: Option<char>,
    pub right: Option<char>,

    pub corners: Option<[char; 4]>,
}

impl Border {
    pub fn symmetric(
        horizontal: Option<char>,
        vertical: Option<char>,
        corners: Option<[char; 4]>,
    ) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
            corners: if vertical.is_some() && horizontal.is_some() {
                corners
            } else {
                None
            },
        }
    }
    fn sides(pos: UVec2, size: UVec2) -> (bool, bool, bool, bool) {
        let l = pos.x == 0;
        let r = pos.x == size.x - 1;
        let t = pos.y == 0;
        let b = pos.y == size.y - 1;
        return (l, r, t, b);
    }

    pub fn create_data(&self, size: UVec2) -> Vec<String> {
        (0..size.y)
            .map(|y| {
                (0..size.x)
                    .map(|x| {
                        (match Self::sides(UVec2 { x, y }, size) {
                            (true, false, true, false) => self.corners.map(|c| c[0]).or(self.left),
                            (false, true, true, false) => self.corners.map(|c| c[1]).or(self.right),
                            (false, true, false, true) => self.corners.map(|c| c[2]).or(self.left),
                            (true, false, false, true) => self.corners.map(|c| c[3]).or(self.right),
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
#[derive(Debug, Bundle)]
pub struct BorderBundle {
    pub border: Border,
    pub padding: Padding,
}
impl BorderBundle {
    pub fn new(border: Border) -> Self {
        Self {
            border: border.clone(),
            padding: Padding(EdgeInsets {
                top: border.top.is_some() as u32,
                bottom: border.bottom.is_some() as u32,
                left: border.left.is_some() as u32,
                right: border.right.is_some() as u32,
            }),
        }
    }
}
