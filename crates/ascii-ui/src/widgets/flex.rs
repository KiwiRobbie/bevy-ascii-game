use bevy::prelude::*;
use spatial_grid::position::Position;

use crate::{
    attachments::{Flex, MainAxisAlignment},
    layout::{
        constraint::Constraint,
        positioned::Positioned,
        widget_layout::{WidgetLayout, WidgetLayoutLogic},
    },
    widget_builder::{WidgetBuilder, WidgetBuilderFn},
    FlexDirection,
};

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
#[require(MultiChildWidget)]
pub struct FlexWidget {
    pub direction: FlexDirection,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
#[require(Position)]
pub struct MultiChildWidget;

impl FlexDirection {
    pub fn main_axis(&self) -> UVec2 {
        match self {
            FlexDirection::Horizontal => UVec2::X,
            FlexDirection::Vertical => UVec2::Y,
        }
    }
    pub fn cross_axis(&self) -> UVec2 {
        match self {
            FlexDirection::Horizontal => UVec2::Y,
            FlexDirection::Vertical => UVec2::X,
        }
    }
}

fn get_layout(entity: Entity, world: &World) -> &WidgetLayout {
    world
        .get::<WidgetLayout>(entity)
        .expect("Failed to get widget logic for child")
}

fn get_flex(entity: Entity, world: &World) -> Option<&Flex> {
    world.get(entity)
}

#[derive(Debug, Default)]
pub(crate) struct FlexLayoutLogic;
impl WidgetLayoutLogic for FlexLayoutLogic {
    fn layout(
        &self,
        entity: Entity,
        constraint: &Constraint,
        world: &World,
        commands: &mut Commands,
    ) -> UVec2 {
        let flex_widget = world
            .get::<FlexWidget>(entity)
            .expect("Flex Widget Logic missing Flex Component!");
        let children = &**world.get::<Children>(entity).unwrap();
        let flex_dir = flex_widget.direction.clone();

        let child_constraint = match flex_dir {
            FlexDirection::Horizontal => constraint.remove_x_bounds(),
            FlexDirection::Vertical => constraint.remove_y_bounds(),
        };

        let mut child_total_space: u32 = 0;
        let mut child_total_flex: u32 = 0;
        let mut child_layout_data: Vec<(Entity, UVec2, Flex)> = Vec::with_capacity(children.len());

        for &child in children {
            let layout = get_layout(child, world);
            let child_size = layout
                .logic
                .layout(child, &child_constraint, world, commands);
            let size = constraint.constrain(child_size);
            let flex = get_flex(child, world).cloned().unwrap_or_default();

            let main_axis_size = match flex_dir {
                FlexDirection::Horizontal => size.x,
                FlexDirection::Vertical => size.y,
            };

            match flex.factor {
                0 => child_total_space += main_axis_size,
                _ => child_total_flex += flex.factor,
            };
            child_layout_data.push((child, size, flex));
        }

        let main_axis_space = match flex_dir {
            FlexDirection::Horizontal => constraint.width.as_ref().map(|w| *w.end()),
            FlexDirection::Vertical => constraint.height.as_ref().map(|h| *h.end()),
        };
        let main_axis_alignment = world
            .get::<MainAxisAlignment>(entity)
            .map(|main_axis_alignment| main_axis_alignment.clone())
            .unwrap_or_default();

        let (total_flex_space, total_padding) = match (main_axis_space, child_total_flex) {
            (Some(main_axis_space), 0) => (0, main_axis_space - child_total_space),
            (Some(main_axis_space), _) => (main_axis_space - child_total_space, 0),
            (None, _) => (0, 0),
        };

        let num_children = child_layout_data.len() as u32;
        let (spacing, extra_spaces) = {
            match main_axis_alignment {
                MainAxisAlignment::SpaceBetween => (
                    total_padding.div_euclid(num_children - 1),
                    total_padding.rem_euclid(num_children - 1) as usize,
                ),
                MainAxisAlignment::SpaceAround => (
                    total_padding.div_euclid(num_children + 1),
                    total_padding.rem_euclid(num_children + 1) as usize,
                ),
                MainAxisAlignment::Start => (0, 0),
                MainAxisAlignment::End => (0, 0),
            }
        };

        let mut cursor_pos: IVec2 = IVec2::ZERO;
        let mut cross_axis_size: u32 = 0;
        let main_axis = flex_dir.main_axis().as_ivec2();
        let cross_axis = flex_dir.cross_axis().as_ivec2();

        if main_axis_alignment == MainAxisAlignment::End {
            cursor_pos += main_axis * total_padding as i32;
        }

        let mut remaining_flex = child_total_flex;
        let mut remaining_flex_space = total_flex_space;

        for (index, (child, size, flex)) in child_layout_data.iter().enumerate() {
            if flex.factor > 0 {
                let flex_space = (remaining_flex_space * flex.factor) / remaining_flex;
                remaining_flex -= flex.factor;
                remaining_flex_space -= flex_space;

                commands.entity(*child).insert(Positioned {
                    offset: cursor_pos,
                    size: main_axis.as_uvec2() * flex_space + cross_axis.as_uvec2() * size,
                });
                cross_axis_size = cross_axis_size.max(size.y);
                cursor_pos += main_axis * flex_space as i32;
            } else {
                if main_axis_alignment == MainAxisAlignment::SpaceAround {
                    let extra = index < extra_spaces;
                    cursor_pos += main_axis * spacing as i32;
                    cursor_pos += main_axis * extra as i32;
                }

                commands.entity(*child).insert(Positioned {
                    offset: cursor_pos,
                    size: *size,
                });
                cross_axis_size = cross_axis_size.max((size * cross_axis.as_uvec2()).element_sum());
                cursor_pos += main_axis * size.as_ivec2();

                if main_axis_alignment == MainAxisAlignment::SpaceBetween
                    && index + 1 != child_layout_data.len()
                {
                    let extra = index < extra_spaces;
                    cursor_pos += main_axis * spacing as i32;
                    cursor_pos += main_axis * extra as i32;
                }
            }
        }
        if main_axis_alignment == MainAxisAlignment::SpaceAround {
            cursor_pos += main_axis * spacing as i32;
        }
        return cursor_pos.as_uvec2() + cross_axis.as_uvec2() * cross_axis_size;
    }
}

impl MultiChildWidget {
    pub fn build<'a>(children: Vec<WidgetBuilderFn<'a>>) -> WidgetBuilderFn<'a> {
        Box::new(move |commands| {
            let children_entities: Vec<Entity> = children
                .into_iter()
                .map(|child| (child)(commands))
                .collect();

            commands
                .spawn((MultiChildWidget, WidgetLayout::new::<FlexLayoutLogic>()))
                .add_children(&children_entities)
                .id()
        })
    }
}

impl FlexWidget {
    pub fn build<'a>(
        direction: FlexDirection,
        children: Vec<WidgetBuilderFn<'a>>,
    ) -> WidgetBuilderFn<'a> {
        MultiChildWidget::build(children)
            .with((Self { direction }, WidgetLayout::new::<FlexLayoutLogic>()))
    }

    pub fn row<'a>(children: Vec<WidgetBuilderFn<'a>>) -> WidgetBuilderFn<'a> {
        Self::build(FlexDirection::Horizontal, children)
    }

    pub fn column<'a>(children: Vec<WidgetBuilderFn<'a>>) -> WidgetBuilderFn<'a> {
        Self::build(FlexDirection::Vertical, children)
    }
}
