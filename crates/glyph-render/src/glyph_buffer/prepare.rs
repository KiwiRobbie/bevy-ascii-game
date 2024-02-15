// Extract from textures

use std::sync::Arc;

use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    math::UVec2,
    render::{
        render_resource::UniformBuffer,
        renderer::{RenderDevice, RenderQueue},
    },
};
use spatial_grid::position::Position;
use wgpu::{Extent3d, TextureUsages};

use crate::{
    glyph_render_plugin::{
        GlyphRenderUniformBuffer, GlyphRenderUniforms, GlyphSolidColor, GpuGlyphTexture,
        GpuGlyphTextureSource,
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
        &ExtractedGlyphTexture,
        Option<&GlyphSolidColor>,
    )>,
    mut prepare_glyph_texture_cache: ResMut<PreparedGlyphTextureCache>,
) {
    for (buffer_entity, buffer) in q_glyph_buffer.iter() {
        let buffer_texture = render_device.create_texture(&wgpu::TextureDescriptor {
            label: Some("glyph buffer data"),
            size: Extent3d {
                depth_or_array_layers: 1,
                width: buffer.size.x,
                height: buffer.size.y,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R16Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        for (entity, _, position, texture, _solid_color) in q_textures
            .iter()
            .filter(|(_, target, _, _, _)| target.0 == buffer_entity)
        {
            let mut uniform_buffer = UniformBuffer::from(GlyphRenderUniforms {
                position: **position,
                size: UVec2::new(texture.width, texture.height),
                target_size: buffer.size,
            });
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
            .insert(GpuGlyphTexture(Arc::new(GpuGlyphTextureSource {
                buffer_texture,
                width: buffer.size.x,
                height: buffer.size.y,
            })));
    }
}
