use bevy::prelude::*;

use super::super::attachments;
use super::super::widgets;
use crate::mouse::ExternalStateMarker;
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
        Has<CheckboxEnabledMarker>,
        Has<TriggeredMarker>,
        Has<ExternalStateMarker>,
    )>,
    mut q_checkbox_text: Query<&mut widgets::text::Text>,
) {
    for (entity, checkbox, was_enabled, triggered, external) in q_checkboxes.iter() {
        if external {
            q_checkbox_text.get_mut(checkbox.checkbox).unwrap().text =
                ["[ ]", "[x]"][was_enabled as usize].into();
        } else if triggered {
            Checkbox::toggle(&mut commands, was_enabled, entity);
            q_checkbox_text.get_mut(checkbox.checkbox).unwrap().text =
                ["[x]", "[ ]"][was_enabled as usize].into();
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
                    InteractableMarker,
                    Checkbox {
                        checkbox: toggle_text,
                    },
                ))(commands)
        })
    }

    pub fn toggle(commands: &mut Commands, enabled: bool, entity: Entity) {
        match enabled {
            true => commands
                .entity(entity)
                .remove::<CheckboxEnabledMarker>()
                .insert(CheckboxDisabledMarker),
            false => commands
                .entity(entity)
                .remove::<CheckboxDisabledMarker>()
                .insert(CheckboxEnabledMarker),
        };
    }
}
