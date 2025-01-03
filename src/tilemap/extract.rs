use bevy::{
    prelude::*,
    render::{sync_world::RenderEntity, Extract},
};

use super::{
    asset::TilemapSource,
    chunk::{TilemapChunk, EMPTY_TILE},
    component::Tilemap,
};
use crate::tileset::asset::TilesetSource;
use glyph_render::{
    atlas::FontAtlasCache,
    font::{CustomFont, CustomFontSource, FontSize},
    glyph_buffer::{GlyphBuffer, TargetGlyphBuffer},
    glyph_render_plugin::SolidColor,
    glyph_texture::{ExtractedGlyphTexture, ExtractedGlyphTextureCache},
};
use spatial_grid::{depth::Depth, global_position::GlobalPosition};
use std::ops::Div;

#[derive(Component)]
pub(crate) struct ExtractedTileMapTileMarker;

pub(crate) fn extract_tilemaps(
    mut commands: Commands,
    q_existing_tiles: Query<Entity, With<ExtractedTileMapTileMarker>>,
    atlas_cache: Extract<Res<FontAtlasCache>>,
    fonts: Extract<Res<Assets<CustomFontSource>>>,
    q_extracted_glyph_buffer: Extract<
        Query<(
            RenderEntity,
            &GlobalPosition,
            &GlyphBuffer,
            &CustomFont,
            &FontSize,
        )>,
    >,
    q_tilemaps: Extract<
        Query<(
            &GlobalPosition,
            Option<&Depth>,
            &Tilemap,
            Option<&SolidColor>,
        )>,
    >,
    tilemaps: Extract<Res<Assets<TilemapSource>>>,
    tilesets: Extract<Res<Assets<TilesetSource>>>,
    chunks: Extract<Res<Assets<TilemapChunk>>>,
    mut extracted_glyph_cache: ResMut<ExtractedGlyphTextureCache>,
) {
    for entity in q_existing_tiles.iter() {
        commands.entity(entity).despawn();
    }

    for (render_entity, buffer_position, buffer, font, font_size) in q_extracted_glyph_buffer.iter()
    {
        let Some(font) = fonts.get(font.id()) else {
            continue;
        };

        let atlas = atlas_cache
            .cached
            .get(&(font_size.clone(), font.key()))
            .unwrap();

        let buffer_start = **buffer_position;
        let buffer_end = buffer_start + buffer.size.as_ivec2();

        for (tilemap_position, tilemap_depth, tilemap, solid_color) in buffer
            .textures
            .iter()
            .flat_map(|entity| q_tilemaps.get(*entity))
        {
            let Some(tilemap) = tilemaps.get(tilemap.id()) else {
                continue;
            };

            let tilemap_offset = **tilemap_position;

            let chunk_start = (buffer_start - tilemap_offset)
                .as_vec2()
                .div(tilemap.chunk_size.as_vec2() * tilemap.tile_size.as_vec2())
                .floor()
                .as_ivec2();

            let chunk_end = (buffer_end - tilemap_offset)
                .as_vec2()
                .div(tilemap.chunk_size.as_vec2() * tilemap.tile_size.as_vec2())
                .ceil()
                .as_ivec2();

            for chunk_y in chunk_start.y..chunk_end.y {
                for chunk_x in chunk_start.x..chunk_end.x {
                    let chunk_position = IVec2::new(chunk_x, chunk_y)
                        * tilemap.chunk_size.as_ivec2()
                        * tilemap.tile_size.as_ivec2();

                    let chunk_id: IVec2 = (chunk_x, chunk_y).into();
                    let Some(chunk) = tilemap
                        .chunk_handles
                        .get(&chunk_id)
                        .and_then(|chunk| chunks.get(chunk.id()))
                    else {
                        continue;
                    };

                    for (index, tile) in chunk.data.iter().enumerate() {
                        if tile == &EMPTY_TILE {
                            continue;
                        }

                        let Some(tileset) = tilesets.get(tilemap.tilesets[tile.0 as usize].id())
                        else {
                            continue;
                        };
                        let tile_offset = UVec2::new(
                            (index as u32) % tilemap.chunk_size.x,
                            (index as u32) / tilemap.chunk_size.x,
                        ) * tileset.tile_size;

                        let data = &tileset.tiles[tile.1 as usize];

                        let extracted_glyph_texture = extracted_glyph_cache.get_or_create(
                            data,
                            solid_color.map(|c| c.color).unwrap_or(Color::WHITE),
                            atlas,
                            font.as_ref(),
                        );

                        let mut entity_commands = commands.spawn((
                            GlobalPosition::from(
                                tilemap_offset + chunk_position + tile_offset.as_ivec2()
                                    - **buffer_position,
                            ),
                            tilemap_depth.cloned().unwrap_or_default(),
                            TargetGlyphBuffer(render_entity),
                            ExtractedTileMapTileMarker,
                            ExtractedGlyphTexture(extracted_glyph_texture),
                        ));
                        if let Some(color) = solid_color {
                            entity_commands.insert(color.clone());
                        }
                    }
                }
            }
        }
    }
}
