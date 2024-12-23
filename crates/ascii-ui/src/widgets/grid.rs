use bevy::{
    ecs::{
        component::Component, entity::Entity, reflect::ReflectComponent, system::Commands,
        world::World,
    },
    math::UVec2,
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
pub struct Grid {
    pub(crate) children: Vec<Entity>,
    pub(crate) child_size: UVec2,
}
#[derive(Debug, Default)]
pub(crate) struct GridLogic;
impl WidgetLayoutLogic for GridLogic {
    fn layout(
        &self,
        entity: Entity,
        constraint: &Constraint,
        world: &World,
        commands: &mut Commands,
    ) -> UVec2 {
        let grid = world
            .get::<Grid>(entity)
            .expect("Grid Widget Logic missing Grid Component!");

        let child_constraint = Constraint::from_max(grid.child_size);

        let mut cursor: UVec2 = UVec2::ZERO;

        let columns = constraint
            .width
            .as_ref()
            .expect("Grid unbounded!")
            .end()
            .div_euclid(grid.child_size.x);
        let width = columns * grid.child_size.x;

        for child in grid.children.iter() {
            let child_logic = world
                .get::<WidgetLayout>(*child)
                .expect("Failed to get widget logic for child");

            constraint.constrain((child_logic.logic).layout(
                *child,
                &child_constraint,
                world,
                commands,
            ));

            commands.entity(*child).insert(Positioned {
                offset: cursor.as_ivec2(),
                size: grid.child_size,
            });

            cursor.x += grid.child_size.x;
            if cursor.x >= width {
                cursor.x = 0;
                cursor.y += grid.child_size.y;
            }
        }
        return UVec2 {
            x: width,
            y: cursor.y + grid.child_size.y,
        };
    }

    fn children(&self, entity: Entity, world: &World) -> Vec<Entity> {
        world
            .get::<Grid>(entity)
            .expect("Grid logic without Grid!")
            .children
            .clone()
    }
}

impl Grid {
    pub(crate) fn build<'a>(
        children: Vec<WidgetBuilderFn<'a>>,
        child_size: UVec2,
    ) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            let mut children_entities = vec![];
            for child in children.into_iter() {
                children_entities.push((child)(commands));
            }
            commands
                .spawn((
                    Self {
                        children: children_entities,
                        child_size,
                    },
                    WidgetLayout::new::<GridLogic>(),
                ))
                .id()
        })
    }
}
