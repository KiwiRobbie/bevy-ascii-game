use std::ops::Deref;

use bevy::prelude::*;
use spatial_grid::position::Position;

use crate::{
    attachments::{padding::Padding, SizedBox},
    layout::{
        constraint::Constraint,
        positioned::WidgetSize,
        widget_layout::{WidgetLayout, WidgetLayoutLogic},
    },
    mouse::{InteractableMarker, ScrollInteraction, ScrollableMarker},
    widget_builder::WidgetBuilder,
};

use super::MultiChildWidget;

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
#[require(MultiChildWidget)]
pub struct ScrollingView {
    pub(crate) position: u32,
    pub(crate) remainder: f32,
}

#[derive(Debug, Default)]
pub(crate) struct ScrollingViewLogic;
impl WidgetLayoutLogic for ScrollingViewLogic {
    fn layout(
        &self,
        entity: Entity,
        constraint: &Constraint,
        world: &World,
        commands: &mut Commands,
    ) -> UVec2 {
        let scrolling_view = world
            .get::<ScrollingView>(entity)
            .expect("Container Logic without Container!");

        let padding = world
            .get::<Padding>(entity)
            .map(|p| p.clone())
            .unwrap_or_default();

        let children: &[Entity] = world
            .get::<Children>(entity)
            .map(Deref::deref)
            .unwrap_or(&[]);

        let sized_box = world.get::<SizedBox>(entity);

        let mut constraint = padding.0.shrink_constraint(constraint);
        let child_constraint = constraint.remove_y_bounds();

        if let Some(SizedBox { width, height }) = sized_box {
            if let Some(width) = *width {
                constraint.width = Some(width..=width);
            }
            if let Some(height) = *height {
                constraint.height = Some(height..=height);
            }
        }
        let height = constraint.max().y as i32;

        let mut y_offset: i32 = -(scrolling_view.position as i32);
        for &child in children {
            let child_widget = world
                .get::<WidgetLayout>(child)
                .expect("Container child invalid!");

            let child_size = (child_widget.logic).layout(child, &child_constraint, world, commands);

            let offset = IVec2 {
                x: padding.0.left as i32,
                y: height - 1 + padding.0.top as i32 - y_offset,
            };

            let size = child_constraint.constrain(child_size);
            commands
                .entity(child)
                .insert((Position(offset), WidgetSize(size)));
            y_offset += size.y as i32;
        }

        return constraint.max();
    }
}

impl ScrollingView {
    pub fn build<'a>(children: Vec<WidgetBuilder<'a>>) -> WidgetBuilder<'a> {
        WidgetBuilder::new(move |commands| {
            let mut children_entities = vec![];
            for child in children.into_iter() {
                children_entities.push(child.build(commands));
            }
            commands
                .spawn((
                    Self {
                        position: 0,
                        remainder: 0.0,
                    },
                    WidgetLayout::new::<ScrollingViewLogic>(),
                    InteractableMarker,
                    ScrollableMarker,
                ))
                .add_children(&children_entities)
                .id()
        })
    }
}

pub(crate) fn scrolling_view_interaction_system(
    mut q_scrolling_view: Query<
        (&mut ScrollingView, &ScrollInteraction),
        (With<InteractableMarker>, With<ScrollableMarker>),
    >,
) {
    for (mut view, interaction) in q_scrolling_view.iter_mut() {
        view.remainder += interaction.distance.y;
        let delta = view.remainder as i32;
        view.remainder -= delta as f32;

        if delta > 0 {
            view.position += delta as u32;
        } else {
            view.position = view.position.saturating_sub((-delta) as u32);
        }
    }
}
