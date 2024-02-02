use bevy::{
    ecs::{
        bundle::Bundle,
        entity::Entity,
        query::With,
        system::{Commands, Query},
        world::World,
    },
    math::IVec2,
};
use glyph_render::glyph_buffer::TargetGlyphBuffer;
use grid_physics::grid::PhysicsGridMember;

use crate::{
    attachments::Root,
    layout::{constraint::Constraint, positioned::Positioned, widget_layout::WidgetLayout},
    widgets::container::Container,
};

pub fn clear_layout(mut commands: Commands, q_positioned: Query<Entity, With<Positioned>>) {
    for entity in q_positioned.iter() {
        commands.entity(entity).remove::<Positioned>();
    }
}

pub fn build_layout(
    mut commands: Commands,
    q_root: Query<(Entity, &WidgetLayout, &Root), With<Container>>,
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
            commands.entity(entity).insert(Positioned {
                offset: root.position,
                size: root.size,
            });
        }
    }
}

pub fn propagate_data_positions(
    mut commands: Commands,
    q_root: Query<(Entity, &TargetGlyphBuffer, &PhysicsGridMember, &Root)>,
    world: &World,
) {
    for (root_entity, target_buffer, grid_member, root) in q_root.iter() {
        if root.enabled {
            recurse_apply_data(
                &mut commands,
                IVec2::ZERO,
                world,
                root_entity,
                &(target_buffer.clone(), grid_member.clone()),
            );
        }
    }
}

pub fn recurse_apply_data<B: Bundle + Clone>(
    commands: &mut Commands,
    parent_offset: IVec2,
    world: &World,
    entity: Entity,
    bundle: &B,
) {
    let Some(widget) = world.get::<WidgetLayout>(entity) else {
        println!("no widget");
        return;
    };
    let Some(position) = world.get::<Positioned>(entity) else {
        println!("no position");
        return;
    };
    let children = (widget.logic).children(entity, world);

    let new_offset = position.offset + parent_offset;
    commands.entity(entity).insert((
        Positioned {
            offset: new_offset,
            size: position.size,
        },
        bundle.clone(),
    ));

    for child in children.iter() {
        recurse_apply_data(commands, new_offset, world, *child, bundle);
    }
}
