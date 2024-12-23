use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query},
    },
    prelude::Has,
};

use crate::{
    mouse::{InteractableMarker, TriggeredMarker},
    widget_builder::{WidgetBuilder, WidgetBuilderFn},
};

use super::super::attachments;
use super::super::widgets;

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
    pub fn build<'a>(label: impl Into<&'a str>) -> WidgetBuilderFn<'a> {
        widgets::Text::build(format!("[ {} ]", label.into())).with((
            attachments::MainAxisAlignment::SpaceBetween,
            InteractableMarker,
            Button {},
        ))
    }

    pub fn build_raw<'a>(label: impl Into<String>) -> WidgetBuilderFn<'a> {
        widgets::Text::build(label.into()).with((
            attachments::MainAxisAlignment::SpaceBetween,
            InteractableMarker,
            Button {},
        ))
    }
}
