use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    transform::components::{GlobalTransform, Transform},
};
use glyph_render::{
    atlas::{CharacterSet, FontAtlasUser},
    font::{CustomFont, CustomFontSource, FontSize},
};

#[derive(Bundle)]
pub struct RenderBundle {
    pub font_atlas_user: FontAtlasUser,
    pub font: CustomFont,
    pub character_set: CharacterSet,
    pub font_size: FontSize,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl RenderBundle {
    pub fn from_font(font: &Handle<CustomFontSource>) -> Self {
        Self {
            font: CustomFont(font.clone()),
            font_atlas_user: FontAtlasUser,
            character_set: Default::default(),
            font_size: Default::default(),
            global_transform: Default::default(),
            transform: Default::default(),
        }
    }
}
