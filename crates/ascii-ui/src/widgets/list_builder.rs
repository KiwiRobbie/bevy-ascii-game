use bevy::prelude::*;

use crate::widget_builder::{WidgetBuilder, WidgetBuilderFn};

use super::MultiChildWidget;

#[derive(Component)]
pub struct ListBuilderWidget<T: Send + Sync> {
    items: Vec<T>,
    pub builder: Box<dyn Fn(usize, &T) -> Box<dyn FnOnce(&mut Commands) -> Entity> + Send + Sync>,
}

impl<T> ListBuilderWidget<T>
where
    T: Send + Sync + 'static,
{
    pub fn build(
        base_width: WidgetBuilderFn,
        builder: Box<dyn Fn(usize, &T) -> Box<dyn FnOnce(&mut Commands) -> Entity> + Send + Sync>,
        items: Vec<T>,
    ) -> WidgetBuilderFn {
        Box::new(move |commands| {
            base_width
                .with(MultiChildWidget(
                    items
                        .iter()
                        .enumerate()
                        .map(|(i, t)| builder(i, t)(commands))
                        .collect(),
                ))
                .with(ListBuilderWidget { builder, items })(commands)
        })
    }

    pub fn push(&mut self, list_widget: &mut MultiChildWidget, item: T, commands: &mut Commands) {
        list_widget.push((self.builder)(self.items.len(), &item)(commands));
        self.items.push(item);
    }
    pub fn pop(&mut self, self_column: &mut MultiChildWidget, commands: &mut Commands) {
        if let Some(entity) = self_column.pop() {
            self.items.pop().unwrap();
            commands.entity(entity).despawn();
        }
    }
}
