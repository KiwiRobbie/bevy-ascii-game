use std::sync::Arc;

use bevy::{
    color::palettes::css,
    prelude::*,
    render::{
        sync_world::{RenderEntity, SyncToRenderWorld},
        Extract, RenderApp,
    },
};
use glyph_render::{
    atlas::FontAtlasCache,
    font::{CustomFont, CustomFontSource, FontSize},
    glyph_buffer::TargetGlyphBuffer,
    glyph_render_plugin::{GlyphSolidColor, GlyphTextureSource},
    glyph_texture::{ExtractedGlyphTexture, ExtractedGlyphTextureCache},
};
use spatial_grid::{depth::Depth, global_position::GlobalPosition};

#[derive(Debug)]
pub struct EditorLayerItem {
    name: String,
    layer: Entity,
    hidden: bool,
}

#[derive(Debug, Component)]
pub struct EditorLayers {
    layers: Vec<EditorLayerItem>,
}

#[derive(Debug, Component)]
pub struct SelectedEditorLayer;

#[derive(Debug, Component)]
#[require(GlobalPosition, Depth, SyncToRenderWorld)]
pub struct EditorLayer {
    data: Vec<char>,
    offset: IVec2,
    size: UVec2,
}

impl EditorLayer {
    pub fn new(offset: IVec2, size: UVec2) -> Self {
        assert!(size.x > 0);
        assert!(size.y > 0);
        Self {
            data: vec!['.'; size.x as usize * size.y as usize],
            offset,
            size,
        }
    }
    pub fn get_texture_source(&self) -> GlyphTextureSource {
        GlyphTextureSource {
            data: self.data.clone().into(),
            width: self.size.x as usize,
            height: self.size.y as usize,
        }
    }

    pub fn get_offset(&self) -> IVec2 {
        self.offset
    }
    pub fn clear_characters(&mut self, clear_character: char) {
        for character in &mut self.data {
            *character = clear_character;
        }
    }
    pub fn write_character(&mut self, position: IVec2, character: char) -> Result<(), ()> {
        let pos =
            position * IVec2::new(1, -1) + self.size.as_ivec2().with_x(0) - IVec2::Y - self.offset;
        if !(0 <= pos.x && pos.x < self.size.x as i32 && 0 <= pos.y && pos.y < self.size.y as i32) {
            return Err(());
        }

        let index = pos.x as usize + pos.y as usize * self.size.x as usize;
        self.data[index] = character;

        Ok(())
    }
}

fn extract_editor_layer(
    mut commands: Commands,
    atlas_cache: Extract<Res<FontAtlasCache>>,
    fonts: Extract<Res<Assets<CustomFontSource>>>,
    q_buffer: Extract<Query<(RenderEntity, &CustomFont, &FontSize, &GlobalPosition)>>,
    q_layer: Extract<
        Query<(
            RenderEntity,
            &EditorLayer,
            &GlobalPosition,
            &Depth,
            &TargetGlyphBuffer,
        )>,
    >,
    mut extracted_glyph_cache: ResMut<ExtractedGlyphTextureCache>,
) {
    for (layer_render_entity, layer, position, depth, target) in &q_layer {
        let Ok((target_render_entity, font, font_size, buffer_position)) = q_buffer.get(**target)
        else {
            continue;
        };

        let Some(font) = fonts.get(font.id()) else {
            continue;
        };

        let atlas = atlas_cache
            .cached
            .get(&(font_size.clone(), font.key()))
            .unwrap();

        let extracted_glyph_texture = extracted_glyph_cache.get_or_create(
            &Arc::new(layer.get_texture_source()),
            Color::WHITE,
            atlas,
            font.as_ref(),
        );

        commands.entity(layer_render_entity).insert((
            GlobalPosition::from(**position + layer.get_offset() - **buffer_position),
            ExtractedGlyphTexture(extracted_glyph_texture),
            TargetGlyphBuffer(target_render_entity),
            depth.clone(),
            GlyphSolidColor {
                color: css::WHITE.into(),
            },
        ));
    }
}

pub struct EditorLayerPlugin;

impl Plugin for EditorLayerPlugin {
    fn build(&self, app: &mut App) {
        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .add_systems(ExtractSchedule, extract_editor_layer);
    }
}
