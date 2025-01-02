use bevy::prelude::*;

use crate::{
    attachments::Padding,
    layout::{
        constraint::Constraint,
        widget_layout::{WidgetLayout, WidgetLayoutLogic},
    },
    render::RenderBundle,
    theme::TextTheme,
    widget_builder::WidgetBuilder,
};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Text {
    pub text: String,
    pub style: TextTheme,
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

        let padding = world.get::<Padding>(entity).cloned().unwrap_or_default();
        let padding_total = padding.total();
        // let padding_offset = IVec2::new(padding.0.left, padding.0.top);
        return UVec2 {
            x: text.text.len() as u32,
            y: 1,
        } + padding_total;
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
            text: Text {
                text,
                style: Default::default(),
            },
            attachments,
        }
    }
    pub fn spawn(commands: &mut Commands, text: String, attachments: T) -> Entity {
        commands.spawn(Self::new(text, attachments)).id()
    }
}

impl Text {
    pub fn build<'a>(text: impl Into<String> + 'a) -> WidgetBuilder<'a> {
        Self::build_styled(text, Default::default())
    }

    pub fn build_styled<'a>(text: impl Into<String> + 'a, style: TextTheme) -> WidgetBuilder<'a> {
        let text = text.into();
        WidgetBuilder::new(move |commands| {
            commands
                .spawn((
                    Self { text, style },
                    RenderBundle::default(),
                    WidgetLayout::new::<TextLogic>(),
                ))
                .id()
        })
    }
}
