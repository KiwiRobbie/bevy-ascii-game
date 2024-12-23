use bevy::{
    ecs::{
        component::Component, entity::Entity, reflect::ReflectComponent, system::Commands,
        world::World,
    },
    math::{IVec2, UVec2},
    reflect::Reflect,
};

use crate::{
    layout::{
        constraint::Constraint,
        positioned::Positioned,
        widget_layout::{WidgetLayout, WidgetLayoutLogic},
    },
    widget_builder::WidgetBuilderFn,
};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Column {
    pub children: Vec<Entity>,
}
#[derive(Debug, Default)]
pub(crate) struct ColumnLogic;
impl WidgetLayoutLogic for ColumnLogic {
    fn layout(
        &self,
        entity: Entity,
        constraint: &Constraint,
        world: &World,
        commands: &mut Commands,
    ) -> UVec2 {
        let column = world
            .get::<Column>(entity)
            .expect("Column Widget Logic missing Column Component!");

        let child_constraint = constraint.remove_y_bounds();

        let mut cursor_y: u32 = 0;
        let mut width: u32 = 0;

        for child in column.children.iter() {
            let child_logic = world
                .get::<WidgetLayout>(*child)
                .expect("Failed to get widget logic for child");

            let size = constraint.constrain((child_logic.logic).layout(
                *child,
                &child_constraint,
                world,
                commands,
            ));

            commands.entity(*child).insert(Positioned {
                offset: IVec2 {
                    x: 0,
                    y: cursor_y as i32,
                },
                size,
            });

            width = width.max(size.x);
            cursor_y += size.y;
        }
        return UVec2 {
            x: width,
            y: cursor_y,
        };
    }

    fn children(&self, entity: Entity, world: &World) -> Vec<Entity> {
        world
            .get::<Column>(entity)
            .expect("Column logic without Column!")
            .children
            .clone()
    }
}

impl Column {
    pub fn build<'a>(
        children: impl IntoIterator<Item = WidgetBuilderFn<'a>> + 'a,
    ) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            let children_entities = children
                .into_iter()
                .map(|child| (child)(commands))
                .collect();

            commands
                .spawn((
                    Self {
                        children: children_entities,
                    },
                    WidgetLayout::new::<ColumnLogic>(),
                ))
                .id()
        })
    }
}
