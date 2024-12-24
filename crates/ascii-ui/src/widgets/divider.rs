use bevy::prelude::*;

use crate::{
    attachments::SizedBox, layout::widget_layout::WidgetLayout, render::bundle::RenderBundle,
    widget_builder::WidgetBuilderFn,
};

use super::{container::ContainerLogic, SingleChildWidget};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Divider {
    pub(crate) character: char,
}

#[derive(Bundle)]
pub(crate) struct DividerBundle {
    pub(crate) divider: Divider,
    pub(crate) layout: WidgetLayout,
    pub(crate) render: RenderBundle,
}

impl Divider {
    pub fn build<'a>(character: char) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            commands
                .spawn((
                    SingleChildWidget { child: None },
                    SizedBox::vertical(1),
                    Self { character },
                    RenderBundle::default(),
                    WidgetLayout::new::<ContainerLogic>(),
                ))
                .id()
        })
    }
}
