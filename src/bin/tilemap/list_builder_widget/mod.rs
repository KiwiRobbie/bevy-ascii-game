use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, component::Component, entity::Entity, system::Commands},
};

use ascii_ui::{
    mouse::IntractableMarker,
    widget_builder::{WidgetBuilder, WidgetBuilderFn},
    widgets,
};

#[derive(Component)]
pub struct ListBuilderWidget<T: Send + Sync> {
    items: Vec<T>,
    pub builder: Box<dyn Fn(&T) -> Box<dyn FnOnce(&mut Commands) -> Entity> + Send + Sync>,
}

impl<T: Send + Sync + 'static> ListBuilderWidget<T> {
    pub fn build<'b>(
        builder: Box<dyn Fn(&T) -> Box<dyn FnOnce(&mut Commands) -> Entity> + Send + Sync>,
        items: Vec<T>,
    ) -> WidgetBuilderFn<'b> {
        Box::new(move |commands| {
            widgets::Column::build(items.iter().map(&builder).collect())
                .with(ListBuilderWidget { builder, items })(commands)
        })
    }

    pub fn push(&mut self, self_column: &mut widgets::Column, item: T, commands: &mut Commands) {
        self_column.children.push((self.builder)(&item)(commands));
        self.items.push(item);
    }
    pub fn pop(&mut self, self_column: &mut widgets::Column, commands: &mut Commands) {
        if let Some(entity) = self_column.children.pop() {
            self.items.pop().unwrap();
            commands.entity(entity).despawn();
        }
    }

    pub fn set(
        &mut self,
        self_column: &mut widgets::Column,
        items: Vec<T>,
        commands: &mut Commands,
    ) {
        self.items = items;
        for entity in self_column.children.iter() {
            commands.entity(*entity).despawn();
        }
        self_column.children = self
            .items
            .iter()
            .map(|item| (&self.builder)(item)(commands))
            .collect();
    }
}
