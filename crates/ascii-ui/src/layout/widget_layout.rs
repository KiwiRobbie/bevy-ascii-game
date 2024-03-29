use bevy::{
    ecs::{component::Component, entity::Entity, system::Commands, world::World},
    math::UVec2,
};

use super::constraint::Constraint;

#[derive(Debug, Component)]
pub struct WidgetLayout {
    pub logic: Box<dyn WidgetLayoutLogic>,
}

impl WidgetLayout {
    pub fn new<T: WidgetLayoutLogic + Default + 'static>() -> Self {
        Self {
            logic: Box::new(T::default()),
        }
    }
}

pub trait WidgetLayoutLogic: std::fmt::Debug + Send + Sync {
    fn layout(
        &self,
        entity: Entity,
        constraint: &Constraint,
        world: &World,
        commands: &mut Commands,
    ) -> UVec2;

    fn children(&self, entity: Entity, world: &World) -> Vec<Entity>;
}
