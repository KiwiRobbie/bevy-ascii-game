use bevy::prelude::*;

use crate::{
    attachments::SizedBox, layout::widget_layout::WidgetLayout, render::RenderBundle,
    theme::TextTheme, widget_builder::WidgetBuilder,
};

use super::{container::ContainerLogic, SingleChildWidget};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Divider {
    pub(crate) character: char,
    pub(crate) style: TextTheme,
}

#[derive(Bundle)]
pub(crate) struct DividerBundle {
    pub(crate) divider: Divider,
    pub(crate) layout: WidgetLayout,
    pub(crate) render: RenderBundle,
}

impl Divider {
    pub fn build<'a>(character: char) -> WidgetBuilder<'a> {
        Self::build_styled(character, TextTheme::Subtle)
    }

    pub fn build_styled<'a>(character: char, style: TextTheme) -> WidgetBuilder<'a> {
        WidgetBuilder::new(move |commands| {
            commands
                .spawn((
                    SingleChildWidget,
                    SizedBox::vertical(1),
                    Self { character, style },
                    RenderBundle::default(),
                    WidgetLayout::new::<ContainerLogic>(),
                ))
                .id()
        })
    }
}
