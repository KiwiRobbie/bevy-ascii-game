use bevy::render::render_resource::ShaderSize;
use wgpu::{
    BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, BufferSize, ShaderStages, StorageTextureAccess, TextureFormat,
    TextureViewDimension,
};

use crate::atlas::AtlasGpuBuffers;

use super::{
    render_resources::{GlyphStorageTexture, GlyphUniformBuffer, GlyphVertexBuffer},
    GlyphPipelineData, GlyphUniforms,
};

pub const fn get_bind_group_layout_descriptor() -> BindGroupLayoutDescriptor<'static> {
    const LAYOUT: [BindGroupLayoutEntry; 5] = [
        // UNIFORMS
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: BufferSize::new(GlyphUniforms::SHADER_SIZE.get()),
            },
            count: None,
        },
        // Glyph Texture
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::ReadOnly,
                format: TextureFormat::R16Uint,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        },
        // Atlas Texture
        BindGroupLayoutEntry {
            binding: 2,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::ReadOnly,
                format: TextureFormat::Rgba8Unorm,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        },
        // Atlas UV's
        BindGroupLayoutEntry {
            binding: 3,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },
        // Vertex Data Output
        BindGroupLayoutEntry {
            binding: 4,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },
    ];

    BindGroupLayoutDescriptor {
        label: Some("glyph generation bind group layout"),
        entries: &LAYOUT,
    }
}

pub fn create_bind_group(
    render_device: &bevy::render::renderer::RenderDevice,
    generation_pipeline_data: &GlyphPipelineData,
    glyph_uniform_buffer: &GlyphUniformBuffer,
    glyph_storage_texture: &GlyphStorageTexture,
    atlas_buffers: &AtlasGpuBuffers,
    vertex_buffer: &GlyphVertexBuffer,
) -> bevy::render::render_resource::BindGroup {
    let bind_group = render_device.create_bind_group(
        Some("glyph generation bind group"),
        &generation_pipeline_data.glyph_generation_bind_group_layout,
        &[
            BindGroupEntry {
                binding: 0,
                resource: glyph_uniform_buffer.binding().unwrap(),
            },
            BindGroupEntry {
                binding: 1,
                resource: bevy::render::render_resource::BindingResource::TextureView(
                    &glyph_storage_texture.create_view(&wgpu::TextureViewDescriptor::default()),
                ),
            },
            BindGroupEntry {
                binding: 2,
                resource: bevy::render::render_resource::BindingResource::TextureView(
                    &atlas_buffers
                        .data
                        .create_view(&wgpu::TextureViewDescriptor::default()),
                ),
            },
            BindGroupEntry {
                binding: 3,
                resource: bevy::render::render_resource::BindingResource::Buffer(
                    atlas_buffers.uvs.as_entire_buffer_binding(),
                ),
            },
            BindGroupEntry {
                binding: 4,
                resource: bevy::render::render_resource::BindingResource::Buffer(
                    vertex_buffer.as_entire_buffer_binding(),
                ),
            },
        ],
    );
    bind_group
}
