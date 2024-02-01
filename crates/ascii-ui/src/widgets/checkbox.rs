use bevy::ecs::{
    component::Component,
    entity::Entity,
    system::{Commands, Query},
};

use crate::mouse::{IntractableMarker, TriggeredMarker};

use super::super::attachments;
use super::super::widgets;

#[derive(Debug, Component)]
pub struct Checkbox {
    checkbox: Entity,
}

#[derive(Debug, Component)]
pub struct CheckboxEnabledMarker;

#[derive(Debug, Component)]
pub struct CheckboxDisabledMarker;

pub struct CheckboxBuilder;
impl CheckboxBuilder {
    pub fn spawn(commands: &mut Commands, label: String) -> Entity {
        let label: Entity = widgets::TextBundle::spawn(commands, label, ());
        let checkbox: Entity = widgets::TextBundle::spawn(commands, "[ ]".into(), ());
        widgets::RowBundle::spawn(
            commands,
            vec![label, checkbox],
            (
                attachments::MainAxisAlignment::SpaceBetween,
                IntractableMarker,
                Checkbox { checkbox },
            ),
        )
    }
}

pub fn checkbox_interaction_system(
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
