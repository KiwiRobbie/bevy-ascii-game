use bevy::{
    ecs::world::World,
    render::{
        render_resource::{GpuArrayBuffer, ShaderSize},
        renderer::RenderDevice,
        view::ViewUniform,
    },
};
use wgpu::{
    BindGroupLayoutEntry, BindingType, BufferBindingType, BufferSize, ShaderStages,
    TextureViewDimension,
};

use super::{GlyphModelUniform, GlyphUniforms, GpuAtlasItem};

pub fn raster_bind_group_layout(render_device: &RenderDevice) -> [BindGroupLayoutEntry; 5] {
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
        GpuArrayBuffer::<GpuAtlasItem>::binding_layout(
            4,
            ShaderStages::VERTEX_FRAGMENT,
            render_device,
        ), // BindGroupLayoutEntry {
           //     binding: 4,
           //     visibility: ShaderStages::VERTEX_FRAGMENT,
           //     ty: BindingType::Buffer {
           //         ty: BufferBindingType::Storage { read_only: true },
           //         has_dynamic_offset: false,
           //         min_binding_size: None,
           //     },
           //     count: None,
           // },
    ]
}
