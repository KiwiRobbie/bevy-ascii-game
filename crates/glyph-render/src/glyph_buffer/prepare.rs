// Extract from textures

use std::sync::Arc;

use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    math::UVec2,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
            UniformBuffer,
        },
        renderer::{RenderDevice, RenderQueue},
    },
};
use spatial_grid::{depth::Depth, position::Position};

use crate::{
    glyph_render_plugin::{
        GlyphRenderUniformBuffer, GlyphRenderUniforms, GlyphSolidColor, GpuGlyphTexture,
        PreparedGlyphTextureSource,
    },
    glyph_texture::{ExtractedGlyphTexture, PreparedGlyphTextureCache},
};

use super::{GlyphBuffer, TargetBufferTexture, TargetGlyphBuffer};
pub fn prepare_glyph_buffers(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    q_glyph_buffer: Query<(Entity, &GlyphBuffer)>,
    q_textures: Query<(
        Entity,
        &TargetGlyphBuffer,
        &Position,
        &Depth,
        &ExtractedGlyphTexture,
        Option<&GlyphSolidColor>,
    )>,
    mut prepare_glyph_texture_cache: ResMut<PreparedGlyphTextureCache>,
) {
    for (buffer_entity, buffer) in q_glyph_buffer.iter() {
        let buffer_texture = render_device.create_texture(&TextureDescriptor {
            label: Some("glyph buffer data"),
            size: Extent3d {
                depth_or_array_layers: 1,
                width: buffer.size.x,
                height: buffer.size.y,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba32Uint,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        for (entity, _, position, depth, texture, _solid_color) in q_textures
            .iter()
            .filter(|(_, target, _, _, _, _)| target.0 == buffer_entity)
        {
            let mut uniform_buffer = UniformBuffer::from(GlyphRenderUniforms {
                position: **position,
                size: UVec2::new(texture.width, texture.height),
                target_size: buffer.size,
                depth: **depth,
                padding: Default::default(),
            });
            uniform_buffer.set_label(Some("Glyph render uniforms"));
            uniform_buffer.write_buffer(&render_device, &render_queue);

            commands.entity(entity).insert((
                GpuGlyphTexture(prepare_glyph_texture_cache.get_or_create(
                    texture,
                    &render_device,
                    &render_queue,
                )),
                TargetBufferTexture(buffer_texture.create_view(&Default::default())),
                GlyphRenderUniformBuffer(uniform_buffer),
            ));
        }

        commands
            .entity(buffer_entity)
            .insert(GpuGlyphTexture(Arc::new(PreparedGlyphTextureSource {
                buffer_texture,
                width: buffer.size.x,
                height: buffer.size.y,
            })));
    }
}
