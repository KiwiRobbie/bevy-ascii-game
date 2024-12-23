use bevy::ecs::{component::Component, entity::Entity};

#[derive(Debug, Component, Clone)]
pub(crate) struct Stack {
    pub(crate) children: Vec<Entity>,
    pub(crate) active: usize,
}

pub(crate) struct StackBuilder;
impl StackBuilder {
    pub(crate) fn new(children: Vec<Entity>, active: usize) -> Stack {
        Stack { children, active }
    }
}
