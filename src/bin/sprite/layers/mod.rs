use bevy::{
    color::palettes::css,
    prelude::*,
    render::{
        sync_world::{RenderEntity, SyncToRenderWorld, TemporaryRenderEntity},
        Extract, RenderApp,
    },
    utils::hashbrown::HashMap,
};

use glyph_render::{
    atlas::FontAtlasCache,
    font::{CustomFont, CustomFontSource, FontSize},
    glyph_buffer::{GlyphBuffer, TargetGlyphBuffer},
    glyph_render_plugin::{GlyphTextureSource, SolidColor},
    glyph_texture::{ExtractedGlyphTexture, ExtractedGlyphTextureCache},
};
use spatial_grid::{depth::Depth, global_position::GlobalPosition, position::Position};
use std::sync::Arc;
use widget::{
    init_layer_list_ui, update_indirect_list_builder, update_layer_entry_widget,
    SelectedLayerWidget,
};

pub mod widget;

#[derive(Debug, Component)]
pub struct EditorLayers;

#[derive(Debug, Component)]
pub struct SelectedEditorLayer;

#[derive(Debug, Component)]
#[require(Position, GlobalPosition, Depth, SyncToRenderWorld)]
pub struct EditorLayer {
    visible: bool,
    name: String,
    tiles: HashMap<IVec2, EditorLayerTile>,
}

const TILE_USIZE: usize = 32;
const TILE_DIMENSIONS: IVec2 = IVec2::new(32, 32);
#[derive(Debug)]
struct EditorLayerTile {
    data: Box<[char; TILE_USIZE * TILE_USIZE]>,
    count: usize,
}
impl EditorLayerTile {
    fn new() -> Self {
        Self {
            data: Box::new([' '; TILE_USIZE * TILE_USIZE]),
            count: 0usize,
        }
    }
}
impl EditorLayerTile {
    fn write(&mut self, index: usize, character: char) -> bool {
        if character != ' ' {
            self.count += 1;
        }
        if self.data[index] != ' ' {
            self.count -= 1;
        }
        self.data[index] = character;
        self.count > 0
    }
}
impl EditorLayer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            visible: true,
            name: name.into(),
            tiles: HashMap::new(),
        }
    }
    pub fn get_texture_sources(
        &self,
        start: IVec2,
        end: IVec2,
    ) -> impl Iterator<Item = (GlyphTextureSource, Position)> + '_ {
        self.tiles
            .iter()
            .filter(move |(&pos, _)| {
                let tile_start = pos * TILE_DIMENSIONS;
                let tile_end = tile_start + TILE_DIMENSIONS;
                IVec2::cmplt(start.max(tile_start), end.min(tile_end)).all()
            })
            .map(|(pos, tile)| {
                (
                    GlyphTextureSource {
                        data: tile.data.clone(),
                        width: TILE_USIZE,
                        height: TILE_USIZE,
                    },
                    Position(*pos * TILE_DIMENSIONS),
                )
            })
    }

    // pub fn clear_tiles(&mut self) {
    //     self.tiles.clear();
    // }

    pub fn write_character(&mut self, position: IVec2, character: char) -> Result<(), ()> {
        let tile_index = position.div_euclid(TILE_DIMENSIONS);
        let tile_position = TILE_DIMENSIONS.with_x(0) - IVec2::new(0, 1)
            + IVec2::new(1, -1) * position.rem_euclid(TILE_DIMENSIONS);
        let character_index = tile_position.x as usize + tile_position.y as usize * TILE_USIZE;

        if character != ' ' {
            let tile_entry = self
                .tiles
                .entry(tile_index)
                .or_insert_with(|| EditorLayerTile::new());
            assert!(tile_entry.write(character_index, character));
            return Ok(());
        }

        if let Some(tile_entry) = self.tiles.get_mut(&tile_index) {
            if !tile_entry.write(character_index, character) {
                self.tiles.remove(&tile_index);
            }
        }

        Ok(())
    }
}

fn extract_editor_layer(
    mut commands: Commands,
    atlas_cache: Extract<Res<FontAtlasCache>>,
    fonts: Extract<Res<Assets<CustomFontSource>>>,
    q_buffer: Extract<
        Query<(
            RenderEntity,
            &CustomFont,
            &FontSize,
            &GlobalPosition,
            &GlyphBuffer,
        )>,
    >,
    q_layer: Extract<Query<(&EditorLayer, &GlobalPosition, &Depth, &TargetGlyphBuffer)>>,
    mut extracted_glyph_cache: ResMut<ExtractedGlyphTextureCache>,
) {
    for (layer, layer_position, depth, target) in &q_layer {
        if !layer.visible {
            continue;
        }

        let Ok((target_render_entity, font, font_size, buffer_position, buffer)) =
            q_buffer.get(**target)
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

        for (texture, position) in layer.get_texture_sources(
            **buffer_position,
            **buffer_position + buffer.size.as_ivec2(),
        ) {
            let extracted_glyph_texture = extracted_glyph_cache.get_or_create(
                &Arc::new(texture),
                Color::WHITE,
                atlas,
                font.as_ref(),
            );

            commands.spawn((
                TemporaryRenderEntity,
                Position::from(*position + **layer_position - **buffer_position),
                GlobalPosition::from(*position + **layer_position - **buffer_position),
                ExtractedGlyphTexture(extracted_glyph_texture),
                TargetGlyphBuffer(target_render_entity),
                depth.clone(),
                SolidColor {
                    color: css::WHITE.into(),
                },
            ));
        }
    }
}

pub struct EditorLayerPlugin;

impl Plugin for EditorLayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    init_layer_list_ui,
                    update_indirect_list_builder,
                    update_layer_entry_widget,
                )
                    .chain(),
                SelectedLayerWidget::update,
            ),
        );

        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .add_systems(ExtractSchedule, extract_editor_layer);
    }
}
