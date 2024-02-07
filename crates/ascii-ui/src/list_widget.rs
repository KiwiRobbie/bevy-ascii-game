use bevy::{ecs::entity::Entity, math::UVec2};

use crate::{widget_builder::WidgetBuilderFn, widgets};

pub trait ListWidget {
    type Args;
    fn build<'a>(children: Vec<WidgetBuilderFn<'a>>, args: Self::Args) -> WidgetBuilderFn<'a>;
    fn push(&mut self, widget: Entity);
    fn pop(&mut self) -> Option<Entity>;
}

impl ListWidget for widgets::Column {
    type Args = ();
    fn build<'a>(children: Vec<WidgetBuilderFn<'a>>, _: Self::Args) -> WidgetBuilderFn<'a> {
        Self::build(children)
    }
    fn pop(&mut self) -> Option<Entity> {
        self.children.pop()
    }
    fn push(&mut self, widget: Entity) {
        self.children.push(widget)
    }
}

impl ListWidget for widgets::Row {
    type Args = ();
    fn build<'a>(children: Vec<WidgetBuilderFn<'a>>, _: Self::Args) -> WidgetBuilderFn<'a> {
        Self::build(children)
    }
    fn pop(&mut self) -> Option<Entity> {
        self.children.pop()
    }
    fn push(&mut self, widget: Entity) {
        self.children.push(widget)
    }
}
impl ListWidget for widgets::Grid {
    type Args = UVec2;
    fn build<'a>(children: Vec<WidgetBuilderFn<'a>>, args: Self::Args) -> WidgetBuilderFn<'a> {
        Self::build(children, args)
    }
    fn pop(&mut self) -> Option<Entity> {
        self.children.pop()
    }
    fn push(&mut self, widget: Entity) {
        self.children.push(widget)
    }
}
