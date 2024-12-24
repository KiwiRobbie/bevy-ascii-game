use bevy::prelude::*;

#[derive(Debug, Component, Clone, Default)]
pub struct Flex {
    pub factor: u32,
}
impl Flex {
    pub fn new(factor: u32) -> Self {
        Self { factor }
    }
}
