use bevy::{
    ecs::{component::Component, entity::Entity, system::Commands},
    prelude::{BuildChildren, Children, DespawnRecursiveExt},
};

use ascii_ui::{
    list_widget::ListWidgetExtension,
    widget_builder::{WidgetBuilder, WidgetBuilderFn},
};

#[derive(Component)]
pub(crate) struct ListBuilderWidget<T: Send + Sync> {
    items: Vec<T>,
    pub(crate) builder:
        Box<dyn Fn(usize, &T) -> Box<dyn FnOnce(&mut Commands) -> Entity> + Send + Sync>,
}

impl<T> ListBuilderWidget<T>
where
    T: Send + Sync + 'static,
{
    pub(crate) fn build<'b, W: ListWidgetExtension>(
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

    pub(crate) fn push(&mut self, list_widget: Entity, item: T, commands: &mut Commands) {
        let list_item = (self.builder)(self.items.len(), &item)(commands);
        commands.entity(list_widget).add_child(list_item);
        self.items.push(item);
    }
    pub(crate) fn pop(&mut self, list_widget: &Children, commands: &mut Commands) {
        if let Some(&entity) = list_widget.last() {
            self.items.pop().unwrap();
            commands.entity(entity).despawn_recursive();
        }
    }

    // pub(crate) fn _set(
    //     &mut self,
    //     self_column: &mut widgets::FlexWidget,
    //     items: Vec<T>,
    //     commands: &mut Commands,
    // ) {
    //     self.items = items;
    //     for entity in self_column.children.iter() {
    //         commands.entity(*entity).despawn();
    //     }
    //     self_column.children = self
    //         .items
    //         .iter()
    //         .enumerate()
    //         .map(|(index, item)| (self.builder)(index, item)(commands))
    //         .collect();
    // }
}
