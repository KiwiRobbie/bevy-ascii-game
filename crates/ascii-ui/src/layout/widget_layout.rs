use bevy::{
    ecs::{component::Component, entity::Entity, system::Commands, world::World},
    math::UVec2,
};

use super::constraint::Constraint;

#[derive(Debug, Component)]
pub(crate) struct WidgetLayout {
    pub(crate) logic: Box<dyn WidgetLayoutLogic>,
}

impl WidgetLayout {
    pub(crate) fn new<T: WidgetLayoutLogic + Default + 'static>() -> Self {
        Self {
            logic: Box::new(T::default()),
        }
    }
}

pub(crate) trait WidgetLayoutLogic: std::fmt::Debug + Send + Sync {
    fn layout(
        &self,
        entity: Entity,
        constraint: &Constraint,
        world: &World,
        commands: &mut Commands,
    ) -> UVec2;
}
