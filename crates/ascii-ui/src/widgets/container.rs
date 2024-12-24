use bevy::{
    ecs::{
        component::Component, entity::Entity, reflect::ReflectComponent, system::Commands,
        world::World,
    },
    math::{IVec2, UVec2},
    reflect::Reflect,
};

use crate::{
    attachments::{self, padding::Padding, SizedBox},
    layout::{
        constraint::Constraint,
        positioned::Positioned,
        widget_layout::{WidgetLayout, WidgetLayoutLogic},
    },
    widget_builder::WidgetBuilderFn,
};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct SingleChildWidget {
    pub child: Option<Entity>,
}

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
        let container = world
            .get::<SingleChildWidget>(entity)
            .cloned()
            .unwrap_or_default();

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

        if let Some(child) = world
            .get::<attachments::stack::Stack>(entity)
            .map(|stack| stack.children[stack.active])
            .or(container.child)
        {
            let child_widget = world
                .get::<WidgetLayout>(child)
                .expect("Container child invalid!");

            let size = (child_widget.logic).layout(child, &constraint, world, commands);

            let offset = IVec2 {
                x: padding.0.left as i32,
                y: padding.0.top as i32,
            };

            commands.entity(child).insert(Positioned {
                offset,
                size: constraint.constrain(size),
            });
            return padding.0.inflate(size);
        }

        return UVec2::new(
            sized_box.width.unwrap_or(constraint.max().x),
            sized_box.height.unwrap_or(constraint.max().y),
        );
    }

    fn children(&self, entity: Entity, world: &World) -> Vec<Entity> {
        let Some(container) = world.get::<SingleChildWidget>(entity) else {
            return vec![];
        };

        let child = world
            .get::<attachments::stack::Stack>(entity)
            .map(|stack| stack.children[stack.active])
            .or(container.child);

        child.iter().map(|e| e.clone()).collect()
    }
}

impl SingleChildWidget {
    pub fn build<'a>(child: Option<WidgetBuilderFn<'a>>) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            let child = child.map(|child| child(commands));
            commands
                .spawn((Self { child }, WidgetLayout::new::<ContainerLogic>()))
                .id()
        })
    }
}
