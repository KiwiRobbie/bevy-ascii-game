use bevy::{ecs::entity::Entity, math::UVec2};

use crate::{
    widget_builder::WidgetBuilderFn,
    widgets::{self},
    FlexDirection,
};

pub trait ListWidgetExtension {
    type Args;
    fn build<'a>(children: Vec<WidgetBuilderFn<'a>>, args: Self::Args) -> WidgetBuilderFn<'a>;
    fn push(&mut self, widget: Entity);
    fn pop(&mut self) -> Option<Entity>;
}

impl ListWidgetExtension for widgets::FlexWidget {
    type Args = FlexDirection;
    fn build<'a>(children: Vec<WidgetBuilderFn<'a>>, direction: Self::Args) -> WidgetBuilderFn<'a> {
        Self::build(direction, children)
    }
    fn pop(&mut self) -> Option<Entity> {
        self.children.pop()
    }
    fn push(&mut self, widget: Entity) {
        self.children.push(widget)
    }
}

impl ListWidgetExtension for widgets::Grid {
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
