use bevy::prelude::*;

use crate::widget_builder::WidgetBuilder;

use super::MultiChildWidget;

#[derive(Component)]
pub struct ListBuilderWidget<T: Send + Sync> {
    items: Vec<T>,
    pub builder: Box<dyn Fn(usize, &T) -> WidgetBuilder + Send + Sync>,
}

impl<T> ListBuilderWidget<T>
where
    T: Send + Sync + 'static,
{
    pub fn build(
        base_width: WidgetBuilder,
        builder: Box<dyn Fn(usize, &T) -> WidgetBuilder + Send + Sync>,
        items: Vec<T>,
    ) -> WidgetBuilder {
        WidgetBuilder::new(move |commands| {
            let children: Vec<Entity> = items
                .iter()
                .enumerate()
                .map(|(i, t)| builder(i, t).build(commands))
                .collect();
            base_width
                .children(&children)
                .with(MultiChildWidget)
                .with(ListBuilderWidget { builder, items })
                .build(commands)
        })
    }

    pub fn push(&mut self, widget: Entity, item: T, commands: &mut Commands) {
        (self.builder)(self.items.len(), &item)
            .parent(widget)
            .build(commands);
        self.items.push(item);
    }
    pub fn pop(&mut self, children: &mut Children, commands: &mut Commands) {
        if let Some(&entity) = children.last() {
            self.items.pop().unwrap();
            commands.entity(entity).despawn_recursive();
        }
    }
    pub fn clear(&mut self, children: &mut Children, commands: &mut Commands) {
        for &entity in children.iter() {
            self.items.pop().unwrap();
            commands.entity(entity).despawn_recursive();
        }
    }
}
