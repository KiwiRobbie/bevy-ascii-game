use bevy::prelude::*;

use glyph_render::glyph_buffer::TargetGlyphBuffer;
use spatial_grid::{grid::PhysicsGridMember, position::Position};

use crate::{
    attachments::Root,
    layout::{constraint::Constraint, positioned::WidgetSize, widget_layout::WidgetLayout},
    widgets::{container::SingleChildWidget, ScrollingView},
};

use super::render_clip::ClipRegion;

pub(crate) fn clear_layout(
    mut commands: Commands,
    q_positioned: Query<Entity, With<WidgetSize>>,
    q_depth: Query<Entity, With<LayoutDepth>>,
) {
    for entity in q_positioned.iter() {
        commands.entity(entity).remove::<WidgetSize>();
    }
    for entity in q_depth.iter() {
        commands.entity(entity).remove::<LayoutDepth>();
    }
}

pub(crate) fn build_layout(
    mut commands: Commands,
    q_root: Query<(Entity, &WidgetLayout, &Root), With<SingleChildWidget>>,
    world: &World,
) {
    for (entity, widget, root) in q_root.iter() {
        if root.enabled {
            (widget.logic).layout(
                entity,
                &Constraint {
                    width: Some(0..=root.size.x),
                    height: Some(0..=root.size.y),
                },
                world,
                &mut commands,
            );
            commands
                .entity(entity)
                .insert((Position(root.position), WidgetSize(root.size)));
        }
    }
}

pub(crate) fn propagate_data_positions(
    mut commands: Commands,
    q_root: Query<(Entity, &TargetGlyphBuffer, &PhysicsGridMember, &Root)>,
    world: &World,
) {
    for (root_entity, target_buffer, grid_member, root) in q_root.iter() {
        if root.enabled {
            recurse_apply_data(
                &mut commands,
                0,
                world,
                root_entity,
                None,
                &(target_buffer.clone(), grid_member.clone()),
            );
        }
    }
}
pub(crate) fn recurse_apply_data<B: Bundle + Clone>(
    commands: &mut Commands,
    depth: usize,
    world: &World,
    entity: Entity,
    clip_region: Option<&ClipRegion>,
    bundle: &B,
) {
    let Some(position) = world.get::<Position>(entity) else {
        println!("no position");
        return;
    };
    let Some(size) = world.get::<WidgetSize>(entity) else {
        println!("no size");
        return;
    };
    let children = world.get::<Children>(entity).into_iter().flatten().copied();

    let clip_region = if let Some(existing_clip_region) =
        world.get::<ScrollingView>(entity).map(|_| ClipRegion {
            start: **position,
            size: **size,
        }) {
        if let Some(clip_region) = clip_region {
            Some(clip_region.intersection(&existing_clip_region))
        } else {
            Some(existing_clip_region.clone())
        }
    } else {
        clip_region.map(|r| r.clone())
    };

    if let Some(clip_region) = &clip_region {
        commands.entity(entity).insert(clip_region.clone());
    }

    commands
        .entity(entity)
        .insert((bundle.clone(), LayoutDepth(depth)));

    for child in children {
        recurse_apply_data(
            commands,
            depth + 1,
            world,
            child,
            clip_region.as_ref(),
            bundle,
        );
    }
}

#[derive(Component, DerefMut, Deref)]
pub(crate) struct LayoutDepth(pub(crate) usize);
