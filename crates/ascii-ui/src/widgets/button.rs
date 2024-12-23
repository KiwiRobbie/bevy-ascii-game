use bevy::ecs::{
    component::Component,
    entity::Entity,
    system::{Commands, Query},
};

use crate::{
    mouse::{IntractableMarker, TriggeredMarker},
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
    q_buttons: Query<(
        Entity,
        Option<&ButtonPressedMarker>,
        Option<&TriggeredMarker>,
    )>,
) {
    for (entity, pressed, triggered) in q_buttons.iter() {
        commands.entity(entity).remove::<ButtonJustPressedMarker>();
        if triggered.is_some() {
            if !pressed.is_some() {
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
    pub fn build<'a>(label: String) -> WidgetBuilderFn<'a> {
        widgets::Text::build(format!("[ {label} ]")).with((
            attachments::MainAxisAlignment::SpaceBetween,
            IntractableMarker,
            Button {},
        ))
    }
}
