use bevy::ecs::{bundle::Bundle, component::Component, entity::Entity, system::Commands};

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

    // pub fn spawn<T: Bundle>(
    //     commands: &mut Commands,
    //     children: Vec<Entity>,
    //     active: usize,
    //     attachments: T,
    // ) -> Entity {
    //     commands
    //         .spawn((Self::new(children, active), attachments))
    //         .id()
    // }
}
