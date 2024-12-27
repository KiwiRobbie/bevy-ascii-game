use bevy::prelude::*;

use crate::{
    layout::{
        constraint::Constraint,
        widget_layout::{WidgetLayout, WidgetLayoutLogic},
    },
    render::bundle::RenderBundle,
    widget_builder::WidgetBuilderFn,
};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Text {
    pub text: String,
}
#[derive(Debug, Default)]

pub(crate) struct TextLogic;
impl WidgetLayoutLogic for TextLogic {
    fn layout(
        &self,
        entity: Entity,
        _constraint: &Constraint,
        world: &World,
        _commands: &mut Commands,
    ) -> UVec2 {
        let text = world
            .get::<Text>(entity)
            .expect("Text Widget Logic missing Text Component!");

        return UVec2 {
            x: text.text.len() as u32,
            y: 1,
        };
    }
}

#[derive(Bundle)]
pub struct TextBundle<T: Bundle> {
    pub(crate) text: Text,
    pub(crate) layout: WidgetLayout,
    pub(crate) render: RenderBundle,
    pub(crate) attachments: T,
}
impl<T: Bundle> TextBundle<T> {
    pub(crate) fn new(text: String, attachments: T) -> Self {
        Self {
            layout: WidgetLayout::new::<TextLogic>(),
            render: RenderBundle::default(),
            text: Text { text },
            attachments,
        }
    }
    pub fn spawn(commands: &mut Commands, text: String, attachments: T) -> Entity {
        commands.spawn(Self::new(text, attachments)).id()
    }
}

impl Text {
    pub fn build<'a>(text: impl Into<String> + 'a) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            commands
                .spawn((
                    Self { text: text.into() },
                    RenderBundle::default(),
                    WidgetLayout::new::<TextLogic>(),
                ))
                .id()
        })
    }
}
