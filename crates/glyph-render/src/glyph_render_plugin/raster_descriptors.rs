use bevy::render::render_resource::{
    BindGroupLayoutEntry, BindingType, BufferBindingType, BufferSize, ShaderStages,
    TextureSampleType, TextureViewDimension,
};
use bevy::render::{render_resource::ShaderSize, view::ViewUniform};

use super::{GlyphModelUniform, GlyphRasterUniforms, GlyphRenderUniforms};

pub(crate) fn raster_bind_group_layout() -> [BindGroupLayoutEntry; 6] {
    [
        // UNIFORMS
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: BufferSize::new(GlyphRasterUniforms::SHADER_SIZE.get()),
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
                sample_type: TextureSampleType::Float { filterable: false },
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
                sample_type: TextureSampleType::Uint,
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
                sample_type: TextureSampleType::Uint,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        },
    ]
}
pub(crate) fn render_bind_group_layout() -> [BindGroupLayoutEntry; 2] {
    [
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: BufferSize::new(GlyphRenderUniforms::SHADER_SIZE.get()),
            },
            count: None,
        },
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: BindingType::Texture {
                multisampled: false,
                sample_type: TextureSampleType::Uint,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        },
    ]
}
