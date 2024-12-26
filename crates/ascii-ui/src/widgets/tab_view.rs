use bevy::prelude::*;

use crate::{
    attachments::MainAxisAlignment,
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
    mut q_stack: Query<&mut widgets::Stack>,
    mut q_text: Query<&mut Text>,
    q_buttons: Query<Option<&TriggeredMarker>, (With<InteractableMarker>, With<Text>)>,
    mut commands: Commands,
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
            stack.prev(tab_view.stack, &mut commands);

            if let Ok(mut text) = q_text.get_mut(tab_view.title) {
                text.text = format!("[ {} ]", tab_view.tabs[stack.get_active()].clone());
            }
        } else if q_buttons
            .get(tab_view.right)
            .ok()
            .map(|t| t.is_some())
            .unwrap_or(false)
        {
            stack.next(tab_view.stack, &mut commands);

            if let Ok(mut text) = q_text.get_mut(tab_view.title) {
                text.text = format!("[ {} ]", tab_view.tabs[stack.get_active()].clone());
            }
        }
    }
}
impl TabView {
    pub fn build<'a>(
        children: Vec<(
            impl Into<String> + 'a,
            Box<dyn Fn(&mut Commands) -> Entity + Send + Sync>,
        )>,
    ) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            // let tab_titles = children.iter().map(|(name, _)| (*name).into()).collect();
            // let tab_entities = children.iter().map(|(_, tab)| *tab).collect();

            let (tab_titles, tab_entities): (Vec<String>, Vec<_>) =
                children.into_iter().map(|(a, b)| (a.into(), b)).unzip();

            let left = widgets::Text::build("<-").with(InteractableMarker)(commands);
            let title = widgets::Text::build("[ Tab View ]")(commands);
            let right = widgets::Text::build("->").with(InteractableMarker)(commands);

            let stack = widgets::Stack::build(tab_entities)(commands);

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
