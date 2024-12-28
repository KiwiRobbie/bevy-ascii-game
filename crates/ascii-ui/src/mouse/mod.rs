use bevy::{
    input::{mouse::mouse_button_input_system, InputSystem},
    prelude::*,
};

use self::input::{update_mouse_position, MouseInput};
use crate::layout::{build_layout::LayoutDepth, positioned::WidgetSize, render_clip::ClipRegion};
use glyph_render::glyph_render_plugin::SolidColor;
use spatial_grid::{
    global_position::GlobalPosition,
    grid::{PhysicsGridMember, SpatialGrid},
};

pub mod input;

#[derive(Debug, Component)]
pub struct InteractableMarker;

#[derive(Debug, Component)]
pub(crate) struct ScrollableMarker;

#[derive(Debug, Component)]
pub struct ActiveMarker;

#[derive(Debug, Component)]
pub struct TriggeredMarker;

#[derive(Debug, Component)]
pub struct ExternalStateMarker;

#[derive(Debug, Component)]
pub(crate) struct ScrollInteraction {
    pub(crate) distance: Vec2,
}

pub(crate) struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PreUpdate,
            (
                mouse_interaction.after(InputSystem),
                update_mouse_position
                    .in_set(InputSystem)
                    .after(mouse_button_input_system),
            ),
        )
        .init_resource::<MouseInput>();
    }
}

pub(crate) fn mouse_interaction(
    mut commands: Commands,
    q_intractable: Query<
        (
            Entity,
            &GlobalPosition,
            &WidgetSize,
            &PhysicsGridMember,
            Option<&ScrollableMarker>,
            Option<&ClipRegion>,
            &LayoutDepth,
        ),
        With<InteractableMarker>,
    >,
    q_active: Query<Entity, With<ActiveMarker>>,
    q_triggered: Query<Entity, With<TriggeredMarker>>,
    q_scroll_interaction: Query<Entity, With<ScrollInteraction>>,
    q_physics_grid: Query<(&SpatialGrid, &GlobalTransform, &GlobalPosition)>,
    mut mouse_input: ResMut<MouseInput>,
) {
    let mut mouse_capture = false;

    for entity in q_active.iter() {
        commands
            .entity(entity)
            .remove::<ActiveMarker>()
            .remove::<SolidColor>();
    }
    for entity in q_triggered.iter() {
        commands.entity(entity).remove::<TriggeredMarker>();
    }
    for entity in q_scroll_interaction.iter() {
        commands.entity(entity).remove::<ScrollInteraction>();
    }

    if let Some(position) = mouse_input.world_position() {
        let mut positions: Vec<_> = q_intractable
            .iter()
            .filter_map(
                |(entity, global_pos, positioned, grid_member, scrollable, clip, depth)| {
                    cursor_in_widget(
                        &q_physics_grid,
                        grid_member,
                        position,
                        global_pos,
                        positioned,
                        clip,
                    )
                    .map(|_| (entity, scrollable, depth))
                },
            )
            .collect();

        positions.sort_by(|a, b| (**a.2).cmp(&**b.2));

        if let Some((entity, _, _)) = positions.last() {
            mouse_capture = true;
            commands
                .entity(*entity)
                .insert(ActiveMarker)
                .insert(SolidColor {
                    color: bevy::color::palettes::css::ORANGE.into(),
                });
            if mouse_input
                .buttons()
                .map(|buttons| buttons.just_pressed(MouseButton::Left))
                .unwrap_or(false)
            {
                commands.entity(*entity).insert(TriggeredMarker);
            }
        }
        for (entity, scrollable, _) in positions.iter() {
            if scrollable.is_some() {
                commands.entity(*entity).insert(ScrollInteraction {
                    distance: mouse_input.scroll().unwrap_or_default(),
                });
                break;
            }
        }
    }
    if mouse_capture {
        mouse_input.consume();
    }
}

fn cursor_in_widget(
    q_physics_grid: &Query<'_, '_, (&SpatialGrid, &GlobalTransform, &GlobalPosition)>,
    grid_member: &PhysicsGridMember,
    world_cursor_position: bevy::prelude::Vec3,
    global_pos: &GlobalPosition,
    size: &WidgetSize,
    clip: Option<&ClipRegion>,
) -> Option<IVec2> {
    let (grid, transform, buffer_position) = q_physics_grid.get(grid_member.grid).ok()?;

    let grid_cursor_position =
        ((transform.compute_matrix().inverse() * world_cursor_position.extend(1.0)).xy()
            / grid.step.as_vec2())
        .as_ivec2()
            + **buffer_position;

    let local_cursor_position = grid_cursor_position - **global_pos;

    let in_clip = match clip {
        Some(clip) => clip.contains(local_cursor_position),
        None => true,
    };

    if in_clip
        && IVec2::ZERO.cmple(local_cursor_position).all()
        && local_cursor_position.cmplt(size.as_ivec2()).all()
    {
        Some(grid_cursor_position)
    } else {
        None
    }
}
