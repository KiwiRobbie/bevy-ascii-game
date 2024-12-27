use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use std::sync::Arc;

mod builder;
mod plugin;
#[derive(Component, DerefMut, Deref, Clone)]
pub struct CharacterSet(pub HashSet<char>);
impl Default for CharacterSet {
    fn default() -> Self {
        const CHARSET: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_´`abcdefghijklmnopqrstuvwxyz{|}~";
        Self(CHARSET.chars().collect())
    }
}

use bytemuck::{Pod, Zeroable};
pub use plugin::FontAtlasPlugin;

pub(crate) use builder::AtlasBuilder;

use crate::font::{CustomFontCacheKey, FontSize};

#[derive(Resource, Default, Clone)]
pub struct FontAtlasCache {
    pub cached: HashMap<(FontSize, CustomFontCacheKey), Arc<FontAtlasSource>>,
}

#[derive(Debug, Clone, Pod, Copy, Zeroable)]
#[repr(C)]
pub(crate) struct AtlasItem {
    pub(crate) start: UVec2,
    pub(crate) size: UVec2,
    pub(crate) offset: IVec2,
}

#[derive(Component, TypePath, Debug, Clone)]
pub struct FontAtlasSource {
    pub(crate) data: Box<[u8]>,
    pub(crate) size: u32,
    pub(crate) items: Box<[AtlasItem]>,
    pub(crate) local_index: HashMap<u16, u16>,
    // pub(crate) glyph_ids: Box<[u16]>,
    pub(crate) charset: HashSet<char>,
}

#[derive(Component, Clone)]
pub struct FontAtlasUser;
