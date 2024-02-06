use bevy::{
    asset::Handle,
    ecs::component::Component,
    prelude::{Deref, DerefMut},
};

use super::asset::TilemapSource;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct Tilemap(pub Handle<TilemapSource>);
