use bevy::ecs::{
    component::Component,
    entity::Entity,
    system::{Commands, Query},
};

use crate::{
    mouse::{IntractableMarker, TriggeredMarker},
    widget_builder::{WidgetBuilder, WidgetBuilderFn, WidgetSaver},
};

use super::super::attachments;
use super::super::widgets;

#[derive(Debug, Component)]
pub struct Checkbox {
    checkbox: Entity,
}

#[derive(Debug, Component)]
pub struct CheckboxEnabledMarker;

#[derive(Debug, Component)]
pub(crate) struct CheckboxDisabledMarker;

pub(crate) fn checkbox_interaction_system(
    mut commands: Commands,
    q_checkboxes: Query<(
        Entity,
        &Checkbox,
        Option<&CheckboxEnabledMarker>,
        Option<&TriggeredMarker>,
    )>,
    mut q_checkbox_text: Query<&mut widgets::text::Text>,
) {
    for (entity, checkbox, enabled, triggered) in q_checkboxes.iter() {
        if triggered.is_some() {
            if enabled.is_some() {
                commands
                    .entity(entity)
                    .remove::<CheckboxEnabledMarker>()
                    .insert(CheckboxDisabledMarker);
                q_checkbox_text.get_mut(checkbox.checkbox).unwrap().text = "[ ]".into();
            } else {
                commands
                    .entity(entity)
                    .remove::<CheckboxDisabledMarker>()
                    .insert(CheckboxEnabledMarker);
                q_checkbox_text.get_mut(checkbox.checkbox).unwrap().text = "[x]".into();
            }
        };
    }
}
impl Checkbox {
    pub fn build<'a>(label: String) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            let mut toggle_text = Entity::PLACEHOLDER;
            widgets::Row::build(vec![
                widgets::Text::build(label),
                widgets::Text::build("[ ]".into()).save_id(&mut toggle_text),
            ])
            .apply(commands)
            .with((
                attachments::MainAxisAlignment::SpaceBetween,
                IntractableMarker,
                Checkbox {
                    checkbox: toggle_text,
                },
            ))(commands)
        })
    }
}
