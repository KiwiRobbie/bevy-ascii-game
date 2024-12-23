// Extract from textures

use std::sync::Arc;

use bevy::{
    asset::Assets,
    color::Color,
    ecs::{
        entity::{Entity, EntityHashSet},
        system::{Commands, Query, Res, ResMut},
    },
    math::IVec2,
    prelude::{Component, Deref},
    render::{
        sync_world::{MainEntity, RenderEntity, TemporaryRenderEntity},
        Extract,
    },
    transform::components::GlobalTransform,
};
use spatial_grid::{depth::Depth, grid::SpatialGrid, position::Position};

use crate::{
    atlas::FontAtlasCache,
    font::{CustomFont, CustomFontSource, FontSize},
    glyph_animation::{GlyphAnimation, GlyphAnimationSource},
    glyph_render_plugin::{
        ExtractedAtlas, GlyphSolidColor, GlyphSpriteMirrored, GlyphTexture, GlyphTextureSource,
    },
    glyph_sprite::GlyphSprite,
    glyph_texture::{ExtractedGlyphTexture, ExtractedGlyphTextureCache},
};

use super::{GlyphBuffer, TargetGlyphBuffer};

pub fn extract_glyph_buffers(
    mut commands: Commands,
    atlas_cache: Extract<Res<FontAtlasCache>>,
    fonts: Extract<Res<Assets<CustomFontSource>>>,
    q_glyph_buffer: Extract<
        Query<(
            RenderEntity,
            &Position,
            &GlobalTransform,
            &GlyphBuffer,
            &CustomFont,
            &FontSize,
            &SpatialGrid,
        )>,
    >,
    q_textures: Extract<
        Query<(
            &Position,
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
    for (buffer_render_entity, buffer_position, transform, buffer, font, font_size, grid) in
        &q_glyph_buffer
    {
        let Some(font_source) = fonts.get(font.id()) else {
            continue;
        };

        let atlas = atlas_cache
            .cached
            .get(&(font_size.clone(), font_source.key()))
            .unwrap();

        commands.entity(buffer_render_entity).insert((
            GlyphBuffer {
                size: buffer.size,
                textures: EntityHashSet::default(),
            },
            buffer_position.clone(),
            transform.clone(),
            ExtractedAtlas(atlas.clone()),
            grid.clone(),
        ));

        for entity in buffer.textures.iter() {
            if let Ok((position, sprite, animation, mirrored, solid_color, depth)) =
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
                        font_source.as_ref(),
                    );

                    commands.spawn((
                        TemporaryRenderEntity,
                        Position::from(**position + offset - **buffer_position),
                        TargetGlyphBuffer(buffer_render_entity),
                        ExtractedGlyphTexture(extracted_glyph_texture),
                        depth.cloned().unwrap_or_default(),
                    ));
                } else if let Some(glyph_sprite) = sprite {
                    let Some(texture) = glyph_textures.get(&glyph_sprite.texture) else {
                        continue;
                    };

                    let extracted_glyph_texture = glyph_texture_cache.get_or_create(
                        &texture.source,
                        solid_color.map(|c| c.color).unwrap_or(Color::WHITE),
                        atlas,
                        font_source.as_ref(),
                    );

                    commands.spawn((
                        TemporaryRenderEntity,
                        Position::from(**position + glyph_sprite.offset - **buffer_position),
                        TargetGlyphBuffer(buffer_render_entity),
                        ExtractedGlyphTexture(extracted_glyph_texture),
                        depth.cloned().unwrap_or_default(),
                    ));
                }
            }
        }
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
