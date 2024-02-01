use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, component::Component},
    transform::components::{GlobalTransform, Transform},
};
use glyph_render::{
    atlas::{CharacterSet, FontAtlasUser},
    font::{CustomFont, CustomFontSource, FontSize},
};

#[derive(Debug, Component)]
pub struct RenderWidgetMarker;

#[derive(Bundle)]
pub struct RenderBundle {
    pub render_widget_marker: RenderWidgetMarker,
    pub font_atlas_user: FontAtlasUser,
    pub character_set: CharacterSet,
    pub font_size: FontSize,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for RenderBundle {
    fn default() -> Self {
        Self {
            render_widget_marker: RenderWidgetMarker,
            font_atlas_user: FontAtlasUser,
            character_set: Default::default(),
            font_size: Default::default(),
            global_transform: Default::default(),
            transform: Default::default(),
        }
    }
}
