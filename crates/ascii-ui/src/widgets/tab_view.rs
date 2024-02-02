use bevy::ecs::{
    component::Component,
    entity::Entity,
    query::With,
    system::{Commands, Query},
};

use crate::{
    attachments::{stack::Stack, MainAxisAlignment},
    mouse::{IntractableMarker, TriggeredMarker},
    widget_builder::{WidgetBuilder, WidgetBuilderFn},
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
impl TabView {
    pub fn build<'a>(children: Vec<(String, Entity)>) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            let tab_titles = children.iter().map(|(name, _)| name.clone()).collect();
            let tab_entities = children.iter().map(|(_, tab)| *tab).collect();

            let left = widgets::Text::build("<-".into()).with(IntractableMarker)(commands);
            let title = widgets::Text::build("[ Tab View ]".into())(commands);
            let right = widgets::Text::build("->".into()).with(IntractableMarker)(commands);

            let stack = widgets::Container::build(None)
                .with((attachments::StackBuilder::new(tab_entities, 0),))(
                commands
            );

            widgets::Column::build(vec![
                widgets::Row::build(vec![
                    WidgetBuilderFn::entity(left),
                    WidgetBuilderFn::entity(title),
                    WidgetBuilderFn::entity(right),
                ])
                .with(attachments::MainAxisAlignment::SpaceAround),
                WidgetBuilderFn::entity(stack),
            ])
            .with((
                MainAxisAlignment::Start,
                TabView {
                    left,
                    title,
                    right,
                    stack,
                    tabs: tab_titles,
                },
            ))(commands)
        })
    }
}
