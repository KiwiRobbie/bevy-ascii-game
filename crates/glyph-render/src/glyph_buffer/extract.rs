// Extract from textures

use std::sync::Arc;

use bevy::{
    asset::Assets,
    color::Color,
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    math::IVec2,
    render::Extract,
    transform::components::GlobalTransform,
};
use spatial_grid::{depth::Depth, grid::SpatialGrid, position::Position};

use crate::{
    atlas::FontAtlasCache,
    font::{CustomFont, CustomFontSource, FontSize},
    glyph_animation::{GlyphAnimation, GlyphAnimationSource},
    glyph_render_plugin::{
        ExtractedAtlas, GlyphSolidColor, GlyphSprite, GlyphSpriteMirrored, GlyphTexture,
        GlyphTextureSource,
    },
    glyph_texture::{ExtractedGlyphTexture, ExtractedGlyphTextureCache},
};

use super::{GlyphBuffer, TargetGlyphBuffer};

pub fn extract_glyph_buffers(
    mut commands: Commands,
    atlas_cache: Extract<Res<FontAtlasCache>>,
    fonts: Extract<Res<Assets<CustomFontSource>>>,
    q_glyph_buffer: Extract<
        Query<(
            Entity,
            &GlobalTransform,
            &GlyphBuffer,
            &CustomFont,
            &FontSize,
            &SpatialGrid,
        )>,
    >,
    q_textures: Extract<
        Query<(
            Entity,
            &Position,
            &TargetGlyphBuffer,
            Option<&GlyphSprite>,
            Option<&GlyphAnimation>,
            Option<&GlyphSpriteMirrored>,
            Option<&GlyphSolidColor>,
            Option<&Depth>,
        )>,
    >,

    glyph_textures: Extract<Res<Assets<GlyphTexture>>>,
    glyph_animations: Extract<Res<Assets<GlyphAnimationSource>>>,
    mut glyph_texture_cache: ResMut<ExtractedGlyphTextureCache>,
) {
    for (buffer_entity, transform, buffer, font, font_size, grid) in q_glyph_buffer.iter() {
        let Some(font) = fonts.get(font.id()) else {
            continue;
        };

        let atlas = atlas_cache
            .cached
            .get(&(font_size.clone(), font.key()))
            .unwrap();

        for entity in buffer.textures.iter() {
            if let Ok((entity, position, target, sprite, animation, mirrored, solid_color, depth)) =
                q_textures.get(*entity)
            {
                if let Some(glyph_animation) = animation {
                    let Some((data, offset)) = extract_animation_frame(
                        &*glyph_animations,
                        glyph_animation,
                        mirrored.is_some(),
                    ) else {
                        continue;
                    };

                    let extracted_glyph_texture = glyph_texture_cache.get_or_create(
                        &data,
                        solid_color.map(|c| c.color).unwrap_or(Color::WHITE),
                        atlas,
                        font.as_ref(),
                    );

                    commands.insert_or_spawn_batch([(
                        entity,
                        (
                            Position::from(**position + offset),
                            target.clone(),
                            ExtractedGlyphTexture(extracted_glyph_texture),
                            depth.cloned().unwrap_or_default(),
                        ),
                    )]);
                } else if let Some(glyph_sprite) = sprite {
                    let Some(texture) = glyph_textures.get(&glyph_sprite.texture) else {
                        continue;
                    };

                    let extracted_glyph_texture = glyph_texture_cache.get_or_create(
                        &texture.source,
                        solid_color.map(|c| c.color).unwrap_or(Color::WHITE),
                        atlas,
                        font.as_ref(),
                    );

                    commands.insert_or_spawn_batch([(
                        entity,
                        (
                            Position::from(**position + glyph_sprite.offset),
                            target.clone(),
                            ExtractedGlyphTexture(extracted_glyph_texture),
                            depth.cloned().unwrap_or_default(),
                        ),
                    )]);
                }
            }
        }

        commands.insert_or_spawn_batch([(
            buffer_entity,
            (
                transform.clone(),
                buffer.clone(),
                ExtractedAtlas(atlas.clone()),
                grid.clone(),
            ),
        )]);
    }
}

fn extract_animation_frame<'a>(
    glyph_animations: &'a Assets<GlyphAnimationSource>,
    glyph_animation: &'a GlyphAnimation,
    mirrored: bool,
) -> Option<(&'a Arc<GlyphTextureSource>, IVec2)> {
    let source = glyph_animations.get(&glyph_animation.source)?;
    let (data, mirrored_data) = source.frames.get(glyph_animation.frame as usize)?;

    if mirrored {
        let data = mirrored_data.as_ref().unwrap_or(data);

        Some((&data.source, data.offset))
    } else {
        Some((&data.source, data.offset))
    }
}
