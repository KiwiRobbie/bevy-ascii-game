use bevy::prelude::*;

use super::super::attachments;
use super::super::widgets;
use crate::{
    mouse::{InteractableMarker, TriggeredMarker},
    widget_builder::WidgetBuilder,
};

#[derive(Debug, Component)]
pub struct Button {}

#[derive(Debug, Component)]
pub(crate) struct ButtonPressedMarker;

#[derive(Debug, Component)]
pub struct ButtonJustPressedMarker;

pub(crate) fn button_interaction_system(
    mut commands: Commands,
    q_buttons: Query<(Entity, Has<ButtonPressedMarker>, Has<TriggeredMarker>)>,
) {
    for (entity, pressed, triggered) in q_buttons.iter() {
        commands.entity(entity).remove::<ButtonJustPressedMarker>();
        if triggered {
            if !pressed {
                commands
                    .entity(entity)
                    .insert((ButtonPressedMarker, ButtonJustPressedMarker));
            }
        } else {
            commands.entity(entity).remove::<ButtonPressedMarker>();
        };
    }
}
impl Button {
    pub fn build<'a>(label: impl Into<&'a str>) -> WidgetBuilder<'a> {
        widgets::Text::build_styled(
            format!("[ {} ]", label.into()),
            crate::theme::TextTheme::Heavy,
        )
        .with((
            attachments::MainAxisAlignment::SpaceBetween,
            InteractableMarker,
            Button {},
        ))
    }

    pub fn build_raw<'a>(label: impl Into<String>) -> WidgetBuilder<'a> {
        widgets::Text::build_styled(label.into(), crate::theme::TextTheme::Heavy).with((
            attachments::MainAxisAlignment::SpaceBetween,
            InteractableMarker,
            Button {},
        ))
    }
}
