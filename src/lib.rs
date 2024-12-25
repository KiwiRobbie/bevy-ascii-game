// #![feature(array_chunks)]
// #![feature(iter_array_chunks)]

pub mod debug;
pub mod debug_menu;
pub mod ecs_utils;
pub mod mount;
pub mod physics_grids;
pub mod player;
pub mod tilemap;
pub mod tileset;
pub mod widgets;

pub(crate) mod utils {
    use bevy::{ecs::entity::EntityHashSet, prelude::*};

    pub(crate) fn clear_component<T: Component>(
        q_focused: Query<Entity, With<T>>,
        mut commands: Commands,
    ) {
        for iter in q_focused.iter() {
            commands.entity(iter).remove::<T>();
        }
    }
}
