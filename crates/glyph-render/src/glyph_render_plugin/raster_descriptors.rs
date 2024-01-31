use bevy::{
    ecs::world::World,
    render::{
        render_resource::ShaderSize,
        view::{ViewUniform, ViewUniforms},
    },
};
use wgpu::{
    BindGroupEntry, BindGroupLayoutEntry, BindingType, BufferBindingType, BufferSize, ShaderStages,
    StorageTextureAccess, TextureFormat, TextureViewDimension,
};

use super::{
    render_resources::GlyphUniformBuffer, AtlasGpuBuffers, GlyphModelUniform,
    GlyphModelUniformBuffer, GlyphPipelineData, GlyphUniforms,
};

pub const RASTER_BINDGROUP_LAYOUT: [BindGroupLayoutEntry; 4] = [
    // UNIFORMS
    BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::FRAGMENT,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: BufferSize::new(GlyphUniforms::SHADER_SIZE.get()),
        },
        count: None,
    },
    BindGroupLayoutEntry {
        binding: 1,
        visibility: ShaderStages::VERTEX_FRAGMENT,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: BufferSize::new(ViewUniform::SHADER_SIZE.get()),
        },
        count: None,
    },
    BindGroupLayoutEntry {
        binding: 2,
        visibility: ShaderStages::VERTEX_FRAGMENT,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: BufferSize::new(GlyphModelUniform::SHADER_SIZE.get()),
        },
        count: None,
    },
    BindGroupLayoutEntry {
        binding: 3,
        visibility: ShaderStages::VERTEX_FRAGMENT,
        ty: BindingType::StorageTexture {
            access: StorageTextureAccess::ReadOnly,
            format: TextureFormat::Rgba8Unorm,
            view_dimension: TextureViewDimension::D2,
        },
        count: None,
    },
];

pub fn create_bind_group(
    render_device: &bevy::render::renderer::RenderDevice,
    glyph_pipeline_data: &GlyphPipelineData,
    world: &World,
    glyph_uniform_buffer: &GlyphUniformBuffer,
    glyph_model_uniforms: &GlyphModelUniformBuffer,
    atlas_buffers: &AtlasGpuBuffers,
) -> bevy::render::render_resource::BindGroup {
    let bind_group = render_device.create_bind_group(
        Some("glyph raster bind group"),
        &glyph_pipeline_data.glyph_raster_bind_group_layout,
        &[
            BindGroupEntry {
                binding: 0,
                resource: glyph_uniform_buffer.binding().unwrap(),
            },
            BindGroupEntry {
                binding: 1,
                resource: world.resource::<ViewUniforms>().uniforms.binding().unwrap(),
            },
            BindGroupEntry {
                binding: 2,
                resource: glyph_model_uniforms.binding().unwrap(),
            },
            BindGroupEntry {
                binding: 3,
                resource: bevy::render::render_resource::BindingResource::TextureView(
                    &atlas_buffers
                        .data
                        .create_view(&wgpu::TextureViewDescriptor::default()),
                ),
            },
        ],
    );
    bind_group
}
