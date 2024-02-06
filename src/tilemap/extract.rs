use std::ops::Div;

use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res},
    },
    math::{IVec2, UVec2},
    render::Extract,
    transform::components::GlobalTransform,
};
use glyph_render::{
    atlas::FontAtlasCache,
    font::{CustomFont, CustomFontSource, FontSize},
    glyph_buffer::{GlyphBuffer, TargetGlyphBuffer},
    glyph_render_plugin::{ExtractedGlyphTexture, GlyphSolidColor},
};
use spatial_grid::{grid::SpatialGrid, position::Position};

use crate::tileset::asset::TilesetSource;

use super::{asset::TilemapSource, component::Tilemap};

pub fn extract_tilemaps(
    mut commands: Commands,
    atlas_cache: Extract<Res<FontAtlasCache>>,
    fonts: Extract<Res<Assets<CustomFontSource>>>,
    q_glyph_buffer: Extract<
        Query<(
            &Position,
            &GlyphBuffer,
            &CustomFont,
            &FontSize,
            &SpatialGrid,
        )>,
    >,
    q_tilemaps: Extract<
        Query<(
            Entity,
            &Position,
            &TargetGlyphBuffer,
            &Tilemap,
            Option<&GlyphSolidColor>,
        )>,
    >,
    tilemaps: Extract<Res<Assets<TilemapSource>>>,
    tilesets: Extract<Res<Assets<TilesetSource>>>,
) {
    for (buffer_position, buffer, font, font_size, grid) in q_glyph_buffer.iter() {
        let Some(font) = fonts.get(font.id()) else {
            continue;
        };

        let atlas = atlas_cache
            .cached
            .get(&(font_size.clone(), font.key()))
            .unwrap();

        let buffer_start = buffer_position.position;
        let buffer_end = buffer_start + buffer.size.as_ivec2();

        for entity in buffer.textures.iter() {
            if let Ok((entity, tilemap_position, target, tilemap, solid_color)) =
                q_tilemaps.get(*entity)
            {
                if let Some(solid_color) = solid_color {
                    commands.insert_or_spawn_batch([(entity, (solid_color.clone(),))]);
                }

                let Some(tilemap) = tilemaps.get(tilemap.id()) else {
                    continue;
                };

                let tilemap_offset = tilemap_position.position;

                let chunk_start = (buffer_start - tilemap_offset)
                    .as_vec2()
                    .div(tilemap.chunk_size.as_vec2())
                    .floor()
                    .as_ivec2();

                let chunk_end = (buffer_end - tilemap_offset)
                    .as_vec2()
                    .div(tilemap.chunk_size.as_vec2())
                    .ceil()
                    .as_ivec2();

                for chunk_y in chunk_start.y..chunk_end.y {
                    for chunk_x in chunk_start.x..chunk_end.x {
                        let chunk_position =
                            IVec2::new(chunk_x, chunk_y) * tilemap.chunk_size.as_ivec2();

                        let chunk_id: IVec2 = (chunk_x, chunk_y).into();
                        let Some(chunk) = tilemap.chunk_data.get(&chunk_id) else {
                            continue;
                        };

                        for (index, tile) in chunk.data.iter().enumerate() {
                            let tileset = tilesets
                                .get(tilemap.tilesets[tile.0 as usize].id())
                                .unwrap();
                            let tile_offset = UVec2::new(
                                (index as u32).rem_euclid(tilemap.chunk_size.x),
                                (index as u32).div_euclid(tilemap.chunk_size.x),
                            ) * tileset.tile_size;

                            let data = &tileset.tiles[tile.1 as usize];

                            let extracted_glyph_texture = ExtractedGlyphTexture::from_text_data(
                                data,
                                atlas,
                                font.as_ref(),
                                font_size,
                            );

                            commands.spawn((
                                Position::from(
                                    tilemap_offset + chunk_position + tile_offset.as_ivec2(),
                                ),
                                target.clone(),
                                extracted_glyph_texture,
                            ));
                        }
                    }
                }
            }
        }
    }
}
