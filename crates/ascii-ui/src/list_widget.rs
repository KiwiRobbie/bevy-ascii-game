use bevy::prelude::*;

use crate::{
    widget_builder::WidgetBuilder,
    widgets::{self},
    FlexDirection,
};

pub trait ListWidgetExtension {
    type Args;
    fn build<'a>(children: Vec<WidgetBuilder<'a>>, args: Self::Args) -> WidgetBuilder<'a>;
}

impl ListWidgetExtension for widgets::FlexWidget {
    type Args = FlexDirection;
    fn build<'a>(children: Vec<WidgetBuilder<'a>>, direction: Self::Args) -> WidgetBuilder<'a> {
        Self::build(direction, children)
    }
}

impl ListWidgetExtension for widgets::Grid {
    type Args = UVec2;
    fn build<'a>(children: Vec<WidgetBuilder<'a>>, args: Self::Args) -> WidgetBuilder<'a> {
        Self::build(children, args)
    }
}
