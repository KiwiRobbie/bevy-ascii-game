use crate::{
    attachments::{padding::Padding, SizedBox},
    layout::{
        constraint::Constraint,
        positioned::WidgetSize,
        widget_layout::{WidgetLayout, WidgetLayoutLogic},
    },
    widget_builder::WidgetBuilderFn,
};
use bevy::prelude::*;
use itertools::Itertools;
use spatial_grid::position::Position;

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
#[require(Position)]
pub struct SingleChildWidget;

#[derive(Debug, Default)]
pub(crate) struct ContainerLogic;
impl WidgetLayoutLogic for ContainerLogic {
    fn layout(
        &self,
        entity: Entity,
        constraint: &Constraint,
        world: &World,
        commands: &mut Commands,
    ) -> UVec2 {
        let padding = world
            .get::<Padding>(entity)
            .map(|p| p.clone())
            .unwrap_or_default();

        let sized_box = world
            .get::<SizedBox>(entity)
            .map(|p| p.clone())
            .unwrap_or_default();

        let constraint = constraint.intersect(&sized_box.as_max_constraint());
        let constraint = padding.0.shrink_constraint(&constraint);

        if let Some(&child) = world.get::<Children>(entity).and_then(|children| {
            children
                .iter()
                .at_most_one()
                .expect("Too many children on single child widget!")
        }) {
            let child_widget = world
                .get::<WidgetLayout>(child)
                .expect("Container child invalid!");

            let size = (child_widget.logic).layout(child, &constraint, world, commands);

            let offset = IVec2 {
                x: padding.0.left as i32,
                y: padding.0.top as i32,
            };

            let child_size = constraint.constrain(size);
            commands.entity(child).insert((
                Position(
                    offset * IVec2::new(1, -1) - child_size.as_ivec2().with_x(0)
                        + (size + padding.total()).as_ivec2().with_x(0),
                ),
                WidgetSize(child_size),
            ));
            return padding.0.inflate(size);
        }

        return UVec2::new(
            sized_box.width.unwrap_or(constraint.max().x),
            sized_box.height.unwrap_or(constraint.max().y),
        );
    }
}

impl SingleChildWidget {
    pub fn build<'a>(child: Option<WidgetBuilderFn<'a>>) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            let child = child.map(|child| child(commands));
            let mut entity_commands =
                commands.spawn((SingleChildWidget, WidgetLayout::new::<ContainerLogic>()));

            if let Some(child) = child {
                entity_commands.add_child(child);
            }

            entity_commands.id()
        })
    }

    pub fn build_existing<'a>(child: Option<Entity>) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            let mut entity_commands =
                commands.spawn((SingleChildWidget, WidgetLayout::new::<ContainerLogic>()));

            if let Some(child) = child {
                entity_commands.add_child(child);
            }

            entity_commands.id()
        })
    }
}
