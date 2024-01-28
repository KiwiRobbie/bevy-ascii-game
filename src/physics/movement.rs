use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component, Default, Debug)]
pub struct Movement {
    pub delta: Vec2,
}

impl Movement {
    pub fn add(&mut self, movement: Vec2) {
        self.delta += movement;
    }
}

#[derive(Debug, Default, Component)]
pub struct MovementObstructed {
    pub x: bool,
    pub y: bool,
    pub neg_x: bool,
    pub neg_y: bool,
}
