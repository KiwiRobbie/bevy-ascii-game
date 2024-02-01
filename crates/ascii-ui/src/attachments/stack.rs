use bevy::ecs::{component::Component, entity::Entity};

#[derive(Debug, Component, Clone)]
pub struct Stack {
    pub children: Vec<Entity>,
    pub active: usize,
}

pub struct StackBuilder;
impl StackBuilder {
    pub fn new(children: Vec<Entity>, active: usize) -> Stack {
        Stack { children, active }
    }
}
