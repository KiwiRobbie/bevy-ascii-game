use bevy_ecs::{component::Component, entity::Entity};
use bevy_math::Vec2;

#[derive(Component, Default, Debug, Clone)]
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
    pub x: Option<Entity>,
    pub y: Option<Entity>,
    pub neg_x: Option<Entity>,
    pub neg_y: Option<Entity>,
}
