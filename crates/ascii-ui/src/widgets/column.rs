use bevy::{
    ecs::{
        bundle::Bundle, component::Component, entity::Entity, reflect::ReflectComponent,
        system::Commands, world::World,
    },
    math::{IVec2, UVec2},
    reflect::Reflect,
};

use crate::layout::{
    constraint::Constraint,
    positioned::Positioned,
    widget_layout::{WidgetLayout, WidgetLayoutLogic},
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

#[derive(Debug, Bundle)]
pub struct ColumnBundle<T: Bundle> {
    pub column: Column,
    pub layout: WidgetLayout,
    pub attachments: T,
}

impl<T: Bundle> ColumnBundle<T> {
    pub fn new(children: Vec<Entity>, attachments: T) -> Self {
        Self {
            column: Column { children },
            layout: WidgetLayout::new::<ColumnLogic>(),
            attachments,
        }
    }
    pub fn spawn(commands: &mut Commands, children: Vec<Entity>, attachments: T) -> Entity {
        commands.spawn(Self::new(children, attachments)).id()
    }
}
