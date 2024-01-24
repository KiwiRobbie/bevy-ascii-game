use std::sync::Arc;

use bevy::{
    ecs::{component::Component, system::Resource},
    math::{IVec2, UVec2},
    prelude::{Deref, DerefMut},
    reflect::TypePath,
    utils::{HashMap, HashSet},
};

mod builder;
mod plugin;
#[derive(Component, DerefMut, Deref)]
pub struct CharacterSet(pub HashSet<char>);

pub use plugin::FontAtlasPlugin;

pub use builder::AtlasBuilder;

use crate::font::{CustomFontCacheKey, FontSize};

#[derive(Resource, Default, Clone)]
pub struct FontAtlasCache {
    pub cached: HashMap<(FontSize, CustomFontCacheKey), Arc<FontAtlasSource>>,
}

#[derive(Debug, Clone)]
pub struct AtlasItem {
    pub start: UVec2,
    pub size: UVec2,
    pub offset: IVec2,
}

#[derive(Component, TypePath, Debug, Clone)]
pub struct FontAtlasSource {
    pub data: Box<[u8]>,
    pub size: u32,
    pub items: Box<[AtlasItem]>,
    pub local_index: HashMap<u16, u16>,
    pub glyph_ids: Box<[u16]>,
    pub charset: HashSet<char>,
}

#[derive(Component)]
pub struct FontAtlasUser;
