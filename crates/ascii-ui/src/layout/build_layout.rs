use bevy::{
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query},
        world::World,
    },
    math::IVec2,
};

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

pub fn propagate_positions(
    mut commands: Commands,
    q_root: Query<Entity, With<Root>>,
    world: &World,
) {
    for root_entity in q_root.iter() {
        recurse_apply_position(&mut commands, IVec2::ZERO, world, root_entity);
    }
}

pub fn recurse_apply_position(
    commands: &mut Commands,
    parent_offset: IVec2,
    world: &World,
    entity: Entity,
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
    commands.entity(entity).insert(Positioned {
        offset: new_offset,
        size: position.size,
    });

    for child in children.iter() {
        recurse_apply_position(commands, new_offset, world, *child);
    }
}
