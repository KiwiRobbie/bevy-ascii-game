use bevy::{
    ecs::{
        bundle::Bundle, component::Component, entity::Entity, reflect::ReflectComponent,
        system::Commands, world::World,
    },
    math::{IVec2, UVec2},
    reflect::Reflect,
};

use crate::{
    attachments::MainAxisAlignment,
    layout::{
        constraint::Constraint,
        positioned::Positioned,
        widget_layout::{WidgetLayout, WidgetLayoutLogic},
    },
    widget_builder::WidgetBuilderFn,
};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Row {
    pub children: Vec<Entity>,
}
#[derive(Debug, Default)]
pub struct RowLogic;
impl WidgetLayoutLogic for RowLogic {
    fn layout(
        &self,
        entity: Entity,
        constraint: &Constraint,
        world: &World,
        commands: &mut Commands,
    ) -> UVec2 {
        let row = world
            .get::<Row>(entity)
            .expect("Row Widget Logic missing Row Component!");

        let child_constraint = constraint.remove_y_bounds();

        let mut cursor_x: u32 = 0;
        let mut height: u32 = 0;

        let children: Vec<(&Entity, UVec2)> = row
            .children
            .iter()
            .map(|child| {
                let child_logic = world
                    .get::<WidgetLayout>(*child)
                    .expect("Failed to get widget logic for child");
                let size = constraint.constrain((child_logic.logic).layout(
                    *child,
                    &child_constraint,
                    world,
                    commands,
                ));
                (child, size)
            })
            .collect();

        let main_axis_alignment = world
            .get::<MainAxisAlignment>(entity)
            .map(|main_axis_alignment| main_axis_alignment.clone())
            .unwrap_or_default();

        let child_total_size: u32 = children.iter().map(|(_, size)| size.x).sum();
        let available_width = *constraint.width.as_ref().unwrap().end();

        let total_spacing = available_width - child_total_size;

        let (spacing, extra_spaces) = {
            match main_axis_alignment {
                MainAxisAlignment::SpaceBetween => {
                    let num_children = children.len() as u32 - 1;
                    (
                        total_spacing.div_euclid(num_children),
                        total_spacing.rem_euclid(num_children) as usize,
                    )
                }
                MainAxisAlignment::SpaceAround => {
                    let num_children = children.len() as u32 + 1;
                    (
                        total_spacing.div_euclid(num_children),
                        total_spacing.rem_euclid(num_children) as usize,
                    )
                }
                MainAxisAlignment::Start => (0, 0),
                MainAxisAlignment::End => (0, 0),
            }
        };

        if main_axis_alignment == MainAxisAlignment::End {
            cursor_x += total_spacing;
        }

        for (index, (child, size)) in children.iter().enumerate() {
            if main_axis_alignment == MainAxisAlignment::SpaceAround {
                let extra = index < extra_spaces;
                cursor_x += spacing + extra as u32
            }

            commands.entity(**child).insert(Positioned {
                offset: IVec2 {
                    x: cursor_x as i32,
                    y: 0,
                },
                size: *size,
            });
            height = height.max(size.y);
            cursor_x += size.x;

            if main_axis_alignment == MainAxisAlignment::SpaceBetween && index + 1 != children.len()
            {
                let extra = index < extra_spaces;
                cursor_x += spacing + extra as u32
            }
        }
        if main_axis_alignment == MainAxisAlignment::SpaceAround {
            cursor_x += spacing
        }
        return UVec2 {
            x: cursor_x,
            y: height,
        };
    }

    fn children(&self, entity: Entity, world: &World) -> Vec<Entity> {
        world
            .get::<Row>(entity)
            .expect("Row logic without Row!")
            .children
            .clone()
    }
}

#[derive(Debug, Bundle)]
pub struct RowBundle<T: Bundle> {
    pub column: Row,
    pub layout: WidgetLayout,
    pub attachments: T,
}

impl<T: Bundle> RowBundle<T> {
    pub fn new(children: Vec<Entity>, attachments: T) -> Self {
        Self {
            column: Row { children },
            layout: WidgetLayout::new::<RowLogic>(),
            attachments,
        }
    }
    pub fn spawn(commands: &mut Commands, children: Vec<Entity>, attachments: T) -> Entity {
        commands.spawn(Self::new(children, attachments)).id()
    }
}

impl Row {
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
                    WidgetLayout::new::<RowLogic>(),
                ))
                .id()
        })
    }
}
