use bevy::prelude::*;

use crate::{
    layout::{
        constraint::Constraint,
        positioned::WidgetSize,
        widget_layout::{WidgetLayout, WidgetLayoutLogic},
    },
    widget_builder::WidgetBuilderFn,
};
use spatial_grid::position::Position;

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

        let children = world.get::<Children>(entity).expect("No children on grid!");
        let height = (children.len() as u32).div_ceil(columns);

        for (i, child) in children.iter().enumerate() {
            let child_grid_pos = UVec2::new(i as u32 % columns, i as u32 / columns);
            let local_child_pos = child_grid_pos * grid.child_size;
            let global_pos = IVec2::new(1, -1) * local_child_pos.as_ivec2() // Invert y coordinate
                 + height as i32 * IVec2::Y; // Y starts from top of bounding box

            let child_logic = world
                .get::<WidgetLayout>(*child)
                .expect("Failed to get widget logic for child");

            let child_size = constraint.constrain((child_logic.logic).layout(
                *child,
                &child_constraint,
                world,
                commands,
            ));

            commands.entity(*child).insert((
                Position(global_pos - child_size.as_ivec2().with_x(0)),
                WidgetSize(grid.child_size),
            ));

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
