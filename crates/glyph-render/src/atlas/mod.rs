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
#[derive(Component, DerefMut, Deref, Clone)]
pub struct CharacterSet(pub HashSet<char>);
impl Default for CharacterSet {
    fn default() -> Self {
        const CHARSET: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
        Self(CHARSET.chars().collect())
    }
}

use bytemuck::{Pod, Zeroable};
pub use plugin::FontAtlasPlugin;

pub use builder::AtlasBuilder;

use crate::font::{CustomFontCacheKey, FontSize};

#[derive(Resource, Default, Clone)]
pub struct FontAtlasCache {
    pub cached: HashMap<(FontSize, CustomFontCacheKey), Arc<FontAtlasSource>>,
}

#[derive(Debug, Clone, Pod, Copy, Zeroable)]
#[repr(C)]
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

#[derive(Component, Clone)]
pub struct FontAtlasUser;
