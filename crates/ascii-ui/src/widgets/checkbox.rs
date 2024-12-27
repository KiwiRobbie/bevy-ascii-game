use bevy::prelude::*;

use super::super::attachments;
use super::super::widgets;
use crate::{
    mouse::{InteractableMarker, TriggeredMarker},
    widget_builder::{WidgetBuilder, WidgetBuilderFn, WidgetSaver},
};

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
    pub fn build_labeled<'a>(label: impl Into<String> + 'a) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            let mut toggle_text = Entity::PLACEHOLDER;
            widgets::FlexWidget::row(vec![
                widgets::Text::build(label),
                widgets::Text::build("[ ]").save_id(&mut toggle_text),
            ])
            .apply(commands)
            .with((
                attachments::MainAxisAlignment::SpaceBetween,
                InteractableMarker,
                Checkbox {
                    checkbox: toggle_text,
                },
            ))(commands)
        })
    }
    pub fn build<'a>() -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            let mut toggle_text = Entity::PLACEHOLDER;
            widgets::Text::build("[ ]")
                .save_id(&mut toggle_text)
                .apply(commands)
                .with((
                    attachments::MainAxisAlignment::SpaceBetween,
                    InteractableMarker,
                    Checkbox {
                        checkbox: toggle_text,
                    },
                ))(commands)
        })
    }
}
