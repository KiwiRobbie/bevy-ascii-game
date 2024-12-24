use bevy::ecs::{component::Component, entity::Entity, query::With, system::Query};

use crate::{
    attachments::{stack::Stack, MainAxisAlignment},
    mouse::{InteractableMarker, TriggeredMarker},
    widget_builder::{WidgetBuilder, WidgetBuilderFn},
};

use super::{
    super::{attachments, widgets},
    text::Text,
};

#[derive(Debug, Component)]
pub struct TabView {
    pub(crate) tabs: Vec<String>,
    pub(crate) left: Entity,
    pub(crate) title: Entity,
    pub(crate) right: Entity,
    pub(crate) stack: Entity,
}

pub(crate) fn tab_view_interaction_system(
    q_tab_view: Query<&TabView>,
    mut q_stack: Query<&mut Stack>,
    mut q_text: Query<&mut Text>,
    q_buttons: Query<Option<&TriggeredMarker>, (With<InteractableMarker>, With<Text>)>,
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
    pub fn build<'a>(children: Vec<(impl Into<String> + 'a, Entity)>) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            // let tab_titles = children.iter().map(|(name, _)| (*name).into()).collect();
            // let tab_entities = children.iter().map(|(_, tab)| *tab).collect();

            let (tab_titles, tab_entities) =
                children.into_iter().map(|(a, b)| (a.into(), b)).unzip();

            let left = widgets::Text::build("<-").with(InteractableMarker)(commands);
            let title = widgets::Text::build("[ Tab View ]")(commands);
            let right = widgets::Text::build("->").with(InteractableMarker)(commands);

            let stack = widgets::SingleChildWidget::build(None)
                .with((attachments::StackBuilder::new(tab_entities, 0),))(
                commands
            );

            widgets::FlexWidget::column(vec![
                widgets::FlexWidget::row(vec![
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
