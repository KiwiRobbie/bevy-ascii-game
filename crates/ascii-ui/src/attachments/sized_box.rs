use bevy::ecs::component::Component;

#[derive(Debug, Component, Clone)]
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
    pub(crate) fn horizontal(width: u32) -> Self {
        Self {
            width: Some(width),
            height: None,
        }
    }
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self {
            width: Some(width),
            height: Some(height),
        }
    }
}
