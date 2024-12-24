use bevy::prelude::*;

use crate::{
    list_widget::ListWidgetExtension,
    widget_builder::{WidgetBuilder, WidgetBuilderFn},
    widgets,
};

#[derive(Component)]
pub struct ListBuilderWidget<T: Send + Sync> {
    items: Vec<T>,
    pub builder: Box<dyn Fn(usize, &T) -> Box<dyn FnOnce(&mut Commands) -> Entity> + Send + Sync>,
}

impl<T> ListBuilderWidget<T>
where
    T: Send + Sync + 'static,
{
    pub fn build<'b, W: ListWidgetExtension>(
        builder: Box<dyn Fn(usize, &T) -> Box<dyn FnOnce(&mut Commands) -> Entity> + Send + Sync>,
        items: Vec<T>,
        args: W::Args,
    ) -> WidgetBuilderFn<'b>
    where
        <W as ListWidgetExtension>::Args: 'b,
    {
        Box::new(move |commands| {
            W::build(
                items
                    .iter()
                    .enumerate()
                    .map(|(i, t)| builder(i, t))
                    .collect(),
                args,
            )
            .with(ListBuilderWidget { builder, items })(commands)
        })
    }

    pub fn push<W: ListWidgetExtension>(
        &mut self,
        list_widget: &mut W,
        item: T,
        commands: &mut Commands,
    ) {
        list_widget.push((self.builder)(self.items.len(), &item)(commands));
        self.items.push(item);
    }
    pub fn pop<W: ListWidgetExtension>(&mut self, self_column: &mut W, commands: &mut Commands) {
        if let Some(entity) = self_column.pop() {
            self.items.pop().unwrap();
            commands.entity(entity).despawn();
        }
    }

    pub fn _set(
        &mut self,
        self_column: &mut widgets::FlexWidget,
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
            .enumerate()
            .map(|(index, item)| (self.builder)(index, item)(commands))
            .collect();
    }
}
