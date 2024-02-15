// Extract from textures

use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res},
    },
    math::UVec2,
    render::{
        render_resource::UniformBuffer,
        renderer::{RenderDevice, RenderQueue},
    },
};
use bytemuck::cast_slice;
use spatial_grid::position::Position;
use wgpu::{Extent3d, TextureDescriptor, TextureUsages};

use crate::glyph_render_plugin::{
    ExtractedGlyphTexture, GlyphRenderUniformBuffer, GlyphRenderUniforms, GlyphSolidColor,
    GpuGlyphTexture,
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
                GpuGlyphTexture {
                    width: texture.width,
                    height: texture.height,
                    buffer_texture: render_device.create_texture_with_data(
                        &render_queue,
                        &TextureDescriptor {
                            label: "glyph texture".into(),
                            size: Extent3d {
                                width: texture.width,
                                height: texture.height,
                                depth_or_array_layers: 1,
                            },
                            mip_level_count: 1,
                            sample_count: 1,
                            dimension: wgpu::TextureDimension::D2,
                            format: wgpu::TextureFormat::R16Uint,
                            usage: TextureUsages::TEXTURE_BINDING,
                            view_formats: &[],
                        },
                        cast_slice(&texture.data),
                    ),
                },
                TargetBufferTexture(buffer_texture.create_view(&Default::default())),
                GlyphRenderUniformBuffer(uniform_buffer),
            ));
        }

        commands.entity(buffer_entity).insert(GpuGlyphTexture {
            buffer_texture,
            width: buffer.size.x,
            height: buffer.size.y,
        });
    }
}
