use bevy::{
    asset::Asset,
    core::{cast_slice, Pod},
    ecs::{
        component::Component,
        system::{lifetimeless::SRes, SystemParamItem},
    },
    math::{IVec2, UVec2},
    reflect::TypePath,
    render::{
        render_asset::RenderAsset,
        render_resource::{
            Buffer, BufferInitDescriptor, BufferUsages, Extent3d, ShaderType, Texture,
            TextureDescriptor, TextureFormat, TextureUsages,
        },
        renderer::{RenderDevice, RenderQueue},
    },
    utils::HashMap,
};

mod builder;
pub use builder::AtlasBuilder;
use bytemuck::Zeroable;

#[derive(Debug, Clone)]
pub struct AtlasItem {
    pub start: UVec2,
    pub size: UVec2,
    pub offset: IVec2,
}

#[derive(Asset, TypePath, Debug, Clone)]
pub struct Atlas {
    pub data: Box<[u8]>,
    pub size: u32,
    pub items: Box<[AtlasItem]>,
    pub local_index: HashMap<u16, u16>,
    pub glyph_ids: Box<[u16]>,
}

#[derive(ShaderType, Pod, Clone, Copy, Zeroable)]
#[repr(C)]
struct GpuAtlasItem {
    start: UVec2,
    size: UVec2,
    offset: IVec2,
}

#[derive(Component, Clone)]
pub struct AtlasGpuBuffers {
    pub data: Texture,
    pub uvs: Buffer,
}

impl RenderAsset for Atlas {
    type ExtractedAsset = Atlas;
    type PreparedAsset = AtlasGpuBuffers;
    type Param = (SRes<RenderDevice>, SRes<RenderQueue>);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, render_queue): &mut SystemParamItem<Self::Param>,
    ) -> Result<
        Self::PreparedAsset,
        bevy::render::render_asset::PrepareAssetError<Self::ExtractedAsset>,
    > {
        let data = render_device.create_texture_with_data(
            render_queue,
            &TextureDescriptor {
                label: Some("gpu font atlas storage texture"),
                size: Extent3d {
                    width: extracted_asset.size,
                    height: extracted_asset.size,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: bevy::render::render_resource::TextureDimension::D2,
                format: TextureFormat::Rgba8Unorm,
                usage: TextureUsages::STORAGE_BINDING | TextureUsages::COPY_SRC,
                view_formats: &[TextureFormat::Rgba8Unorm],
            },
            &extracted_asset.data,
        );

        let uvs = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("gpu font atlas uv buffer"),
            usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
            contents: cast_slice(
                &extracted_asset
                    .items
                    .iter()
                    .map(|x| GpuAtlasItem {
                        offset: x.offset,
                        size: x.size,
                        start: x.start,
                    })
                    .collect::<Box<[_]>>(),
            ),
        });

        Ok(AtlasGpuBuffers { data, uvs })
    }
}
