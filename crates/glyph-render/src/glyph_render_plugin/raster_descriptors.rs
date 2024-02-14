use bevy::render::{render_resource::ShaderSize, view::ViewUniform};
use wgpu::{
    BindGroupLayoutEntry, BindingType, BufferBindingType, BufferSize, ShaderStages,
    TextureViewDimension,
};

use super::{GlyphModelUniform, GlyphUniforms};

pub fn raster_bind_group_layout() -> [BindGroupLayoutEntry; 6] {
    [
        // UNIFORMS
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: BufferSize::new(GlyphUniforms::SHADER_SIZE.get()),
            },
            count: None,
        },
        // VIEW
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
        // MODEL
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
        // Atlas Texture
        BindGroupLayoutEntry {
            binding: 3,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: BindingType::Texture {
                multisampled: false,
                sample_type: wgpu::TextureSampleType::Float { filterable: false },
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        },
        // Atlas UV's
        BindGroupLayoutEntry {
            binding: 4,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: BindingType::Texture {
                multisampled: false,
                sample_type: wgpu::TextureSampleType::Uint,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        },
        // Glyph Buffer
        BindGroupLayoutEntry {
            binding: 5,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: BindingType::Texture {
                multisampled: false,
                sample_type: wgpu::TextureSampleType::Uint,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        },
    ]
}
