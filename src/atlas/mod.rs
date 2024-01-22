use bevy::{
    math::{IVec2, UVec2},
    utils::HashMap,
};

mod builder;
pub use builder::AtlasBuilder;

pub struct AtlasItem {
    pub start: UVec2,
    pub size: UVec2,
    pub offset: IVec2,
}
pub struct Atlas {
    pub data: Box<[u8]>,
    pub size: u32,
    pub items: Box<[AtlasItem]>,
    pub local_index: HashMap<u16, u16>,
    pub glyph_ids: Box<[u16]>,
}
