use bevy::ecs::{
    component::Component,
    entity::Entity,
    query::With,
    system::{Commands, Query},
};

use crate::{
    attachments::{stack::Stack, MainAxisAlignment},
    mouse::{IntractableMarker, TriggeredMarker},
};

use super::{
    super::{attachments, widgets},
    text::Text,
};

#[derive(Debug, Component)]
pub struct TabView {
    pub tabs: Vec<String>,
    pub left: Entity,
    pub title: Entity,
    pub right: Entity,
    pub stack: Entity,
}

pub struct TabViewBuilder;
impl TabViewBuilder {
    pub fn spawn(commands: &mut Commands, children: Vec<(String, Entity)>) -> Entity {
        let tabs = children.iter().map(|(name, _)| name.clone()).collect();

        let left = widgets::TextBundle::spawn(commands, "<-".into(), IntractableMarker);
        let title = widgets::TextBundle::spawn(commands, "[ Tab View ]".into(), ());
        let right = widgets::TextBundle::spawn(commands, "->".into(), IntractableMarker);

        let header = widgets::RowBundle::spawn(
            commands,
            vec![left, title, right],
            attachments::MainAxisAlignment::SpaceAround,
        );
        let stack = widgets::ContainerBundle::spawn(
            commands,
            None,
            (attachments::StackBuilder::new(
                children.iter().map(|(_, tab)| *tab).collect(),
                0,
            ),),
        );

        widgets::ColumnBundle::spawn(
            commands,
            vec![header, stack],
            (
                MainAxisAlignment::Start,
                TabView {
                    left,
                    title,
                    right,
                    stack,
                    tabs,
                },
            ),
        )
    }
}

pub fn tab_view_interaction_system(
    q_tab_view: Query<&TabView>,
    mut q_stack: Query<&mut Stack>,
    mut q_text: Query<&mut Text>,
    q_buttons: Query<Option<&TriggeredMarker>, (With<IntractableMarker>, With<Text>)>,
) {
    for tab_view in q_tab_view.iter() {
        let Some(mut stack) = q_stack.get_mut(tab_view.stack).ok() else {
            continue;
        };

        if q_buttons
            .get(tab_view.left)
            .ok()
            .map(|t| t.is_some())
            .unwrap_or(false)
        {
            stack.active =
                (stack.active + stack.children.len() - 1).rem_euclid(stack.children.len());

            if let Ok(mut text) = q_text.get_mut(tab_view.title) {
                text.text = format!("[ {} ]", tab_view.tabs[stack.active].clone());
            }
        } else if q_buttons
            .get(tab_view.right)
            .ok()
            .map(|t| t.is_some())
            .unwrap_or(false)
        {
            stack.active = (stack.active + 1).rem_euclid(stack.children.len());
            if let Ok(mut text) = q_text.get_mut(tab_view.title) {
                text.text = format!("[ {} ]", tab_view.tabs[stack.active].clone());
            }
        }
    }
}
