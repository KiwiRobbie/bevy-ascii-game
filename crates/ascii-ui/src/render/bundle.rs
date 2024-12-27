use bevy::prelude::*;

use glyph_render::{
    atlas::{CharacterSet, FontAtlasUser},
    font::FontSize,
};

#[derive(Debug, Component)]
pub(crate) struct RenderWidgetMarker;

#[derive(Bundle)]
pub struct RenderBundle {
    pub(crate) render_widget_marker: RenderWidgetMarker,
    pub(crate) font_atlas_user: FontAtlasUser,
    pub(crate) character_set: CharacterSet,
    pub(crate) font_size: FontSize,
    pub(crate) transform: Transform,
    pub(crate) global_transform: GlobalTransform,
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
