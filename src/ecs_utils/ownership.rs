// use bevy::{
//     ecs::{component::ComponentId, entity::EntityHashSet, world::DeferredWorld},
//     prelude::*,
// };
// pub struct OwnershipPlugin;
// impl Plugin for OwnershipPlugin {
//     fn build(&self, app: &mut App) {}
// }

// fn add_owned_by_hook(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
//     let owner = world.get::<OwnedBy>(entity).unwrap().0;
//     let mut owner_entities = world.get_mut::<OwnedEntities>(owner).unwrap();
//     owner_entities.insert(entity);
// }
// fn remove_owned_by_hook(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
//     let owner = world.get::<OwnedBy>(entity).unwrap().0;
//     let mut owner_entities = world.get_mut::<OwnedEntities>(owner).unwrap();
//     owner_entities.remove(&entity);
// }

// #[derive(Debug, Component)]
// #[component(on_add = add_owned_by_hook)]
// #[component(on_remove = remove_owned_by_hook)]
// pub(crate) struct OwnedBy(putb Entity);

// fn remove_owner_hook(mut world: DeferredWorld, entity: Entity, id: ComponentId) {
//     let Some(owned_entities) = world.get::<OwnedEntities>(entity) else {
//         return;
//     };
//     let owned_entities = owned_entities.clone();
//     for entity in &owned_entities.0 {
//         world.commands().entity(*entity).despawn();
//     }
// }

// #[derive(Debug, Component, Deref, DerefMut, Clone)]
// pub(crate) struct OwnedEntities(pub EntityHashSet);
