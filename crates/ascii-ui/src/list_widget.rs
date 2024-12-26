use bevy::{ecs::entity::Entity, math::UVec2};

use crate::{
    widget_builder::WidgetBuilderFn,
    widgets::{self},
    FlexDirection,
};

pub trait ListWidgetExtension {
    type Args;
    fn build<'a>(children: Vec<WidgetBuilderFn<'a>>, args: Self::Args) -> WidgetBuilderFn<'a>;
}

impl ListWidgetExtension for widgets::FlexWidget {
    type Args = FlexDirection;
    fn build<'a>(children: Vec<WidgetBuilderFn<'a>>, direction: Self::Args) -> WidgetBuilderFn<'a> {
        Self::build(direction, children)
    }
}

impl ListWidgetExtension for widgets::Grid {
    type Args = UVec2;
    fn build<'a>(children: Vec<WidgetBuilderFn<'a>>, args: Self::Args) -> WidgetBuilderFn<'a> {
        Self::build(children, args)
    }
}
