// Extract from textures

use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res},
    },
    math::IVec2,
    render::Extract,
    transform::components::GlobalTransform,
};
use spatial_grid::{grid::SpatialGrid, position::Position};

use crate::{
    atlas::FontAtlasCache,
    font::{CustomFont, CustomFontSource, FontSize},
    glyph_animation::{GlyphAnimation, GlyphAnimationSource},
    glyph_render_plugin::{
        ExtractedAtlas, ExtractedGlyphTexture, GlyphSolidColor, GlyphSprite, GlyphSpriteMirrored,
        GlyphTextureSource,
    },
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
        )>,
    >,

    glyph_textures: Extract<Res<Assets<GlyphTextureSource>>>,
    glyph_animations: Extract<Res<Assets<GlyphAnimationSource>>>,
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
            if let Ok((entity, position, target, sprite, animation, mirrored, solid_color)) =
                q_textures.get(*entity)
            {
                if let Some(solid_color) = solid_color {
                    commands.insert_or_spawn_batch([(entity, (solid_color.clone(),))]);
                }
                if let Some(glyph_animation) = animation {
                    let Some((data, offset)) = extract_animation_frame(
                        &*glyph_animations,
                        glyph_animation,
                        mirrored.is_some(),
                    ) else {
                        continue;
                    };

                    let extracted_glyph_texture = ExtractedGlyphTexture::from_text_data(
                        &data,
                        atlas,
                        font.as_ref(),
                        font_size,
                    );

                    commands.insert_or_spawn_batch([(
                        entity,
                        (
                            Position::from(**position + offset),
                            target.clone(),
                            extracted_glyph_texture,
                        ),
                    )]);
                } else if let Some(glyph_sprite) = sprite {
                    let Some(source) = glyph_textures.get(glyph_sprite.texture.clone()) else {
                        continue;
                    };

                    let extracted_glyph_texture = ExtractedGlyphTexture::from_text_data(
                        &source.data,
                        atlas,
                        font.as_ref(),
                        font_size,
                    );
                    commands.insert_or_spawn_batch([(
                        entity,
                        (
                            Position::from(**position + glyph_sprite.offset),
                            target.clone(),
                            extracted_glyph_texture,
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

fn extract_animation_frame(
    glyph_animations: &Assets<GlyphAnimationSource>,
    glyph_animation: &GlyphAnimation,
    mirrored: bool,
) -> Option<(Vec<String>, IVec2)> {
    let source = glyph_animations.get(glyph_animation.source.clone())?;
    let (data, mirrored_data) = source.frames.get(glyph_animation.frame as usize)?;

    if mirrored {
        let data = mirrored_data.as_ref().unwrap_or(data);

        Some((data.data.clone(), data.offset))
    } else {
        Some((data.data.clone(), data.offset))
    }
}
