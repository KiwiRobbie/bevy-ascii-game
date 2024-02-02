use bevy::{
    ecs::{
        bundle::Bundle, component::Component, entity::Entity, reflect::ReflectComponent,
        system::Commands, world::World,
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
pub struct ColumnLogic;
impl WidgetLayoutLogic for ColumnLogic {
    fn layout(
        &self,
        entity: Entity,
        constraint: &Constraint,
        world: &World,
        commands: &mut Commands,
    ) -> UVec2 {
        let row = world
            .get::<Column>(entity)
            .expect("Row Widget Logic missing Row Component!");

        let child_constraint = constraint.remove_y_bounds();

        let mut cursor_y: u32 = 0;
        let mut width: u32 = 0;

        for child in row.children.iter() {
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
            .expect("Row logic without Row!")
            .children
            .clone()
    }
}

impl Column {
    pub fn build<'a>(children: Vec<WidgetBuilderFn<'a>>) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            let mut children_entities = vec![];
            for child in children.into_iter() {
                children_entities.push((child)(commands));
            }
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
