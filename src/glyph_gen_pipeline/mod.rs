use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_asset::{RenderAsset, RenderAssetPlugin, RenderAssets},
        render_graph::{RenderGraph, RenderGraphApp},
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
        Extract, Render, RenderApp,
    },
};
pub use node::GlyphGenerationNode;
use swash::FontRef;

use crate::{
    atlas::Atlas,
    font::CustomFont,
    glyph_raster_pipeline::{GlyphRasterNode, GlyphRasterPipelineData},
};

mod node;

pub struct FontRenderPlugin;
const MAIN_GRAPH_2D: &str = bevy::core_pipeline::core_2d::graph::NAME;

impl Plugin for FontRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<GlyphSprite>::default())
            .init_asset::<GlyphTexture>()
            .add_plugins(RenderAssetPlugin::<GlyphTexture>::default())
            .add_plugins(RenderAssetPlugin::<Atlas>::default());

        let render_app = &mut app.get_sub_app_mut(RenderApp).unwrap();

        render_app
            .add_systems(Render, prepare_buffers)
            .add_systems(ExtractSchedule, extract_glyph_sprite_transform)
            .add_render_graph_node::<GlyphGenerationNode>(MAIN_GRAPH_2D, "glyph_generation")
            .add_render_graph_node::<GlyphRasterNode>(MAIN_GRAPH_2D, "glyph_raster")
            .add_render_graph_edges(
                MAIN_GRAPH_2D,
                &[
                    bevy::core_pipeline::core_2d::graph::node::TONEMAPPING,
                    "glyph_generation",
                    "glyph_raster",
                    bevy::core_pipeline::core_2d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            );

        render_app
            .world
            .resource_mut::<RenderGraph>()
            .get_sub_graph_mut(MAIN_GRAPH_2D)
            .unwrap()
            .add_slot_edge(
                "glyph_generation",
                "vertex_buffer",
                "glyph_raster",
                "vertex_buffer",
            );
    }
    fn finish(&self, app: &mut App) {
        // setup custom render pipeline
        app.sub_app_mut(RenderApp)
            .init_resource::<GlyphGenerationPipelineData>()
            .init_resource::<GlyphRasterPipelineData>();
    }
}

#[derive(Clone, ShaderType)]
pub struct GlyphGenerationUniforms {
    pub color: Vec4,
    pub width: u32,
    pub height: u32,
}

#[derive(Asset, TypePath, Clone)]
pub struct GlyphTexture {
    pub data: Box<[u8]>,

    pub width: u32,
    pub height: u32,

    pub advance: u32,
    pub leading: u32,
}

impl GlyphTexture {
    pub fn from_text(text: &[String], atlas: &Atlas, font: FontRef) -> Self {
        let height = text.len();
        let width = text[0].len();

        let mut data: Box<[u8]> = vec![0; 2 * width * height].into();
        let charmap = font.charmap();

        for (y, chars) in text.iter().enumerate() {
            assert_eq!(text[y].len(), width);
            for (x, c) in chars.chars().enumerate() {
                let index = 2 * (x + y * width);
                let glyph_id = atlas.local_index.get(&charmap.map(c)).unwrap_or(&u16::MAX);
                data[index..index + 2].copy_from_slice(&glyph_id.to_le_bytes());
            }
        }

        Self {
            data,
            width: width as u32,
            height: height as u32,
            advance: 19u32,
            leading: 32u32,
        }
    }
}

#[derive(Component, Clone, ExtractComponent)]
pub struct GlyphSprite {
    pub color: Color,
    pub atlas: Handle<Atlas>,
    pub font: Handle<CustomFont>,
    pub texture: Handle<GlyphTexture>,
}

pub struct GpuGlyphTexture {
    pub storage_texture: Texture,
    pub width: u32,
    pub height: u32,
}

impl RenderAsset for GlyphTexture {
    type ExtractedAsset = Self;
    type Param = (SRes<RenderDevice>, SRes<RenderQueue>);
    type PreparedAsset = GpuGlyphTexture;
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
        let storage_texture = render_device.create_texture_with_data(
            render_queue,
            &TextureDescriptor {
                label: Some("glyph texture"),
                size: Extent3d {
                    width: extracted_asset.width,
                    height: extracted_asset.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::R16Uint,
                usage: TextureUsages::COPY_SRC | TextureUsages::STORAGE_BINDING,
                view_formats: &[TextureFormat::R16Uint],
            },
            &extracted_asset.data,
        );
        Ok(GpuGlyphTexture {
            storage_texture,
            width: extracted_asset.width,
            height: extracted_asset.height,
        })
    }
}

#[derive(Component, Deref, DerefMut)]
struct GylphGenerationUniformBuffer(UniformBuffer<GlyphGenerationUniforms>);

#[derive(Component, Deref, DerefMut)]
struct GlyphStorageTexture(Texture);

#[derive(Component, Deref, DerefMut)]
struct GlyphVertexBuffer(Buffer);

#[derive(Component)]
pub struct GlyphTextureInfo {
    pub width: u32,
    pub height: u32,
}

#[derive(ShaderType)]
pub struct GlyphModelUniform {
    model: Mat4,
}

impl GlyphModelUniform {
    fn new(transform: GlobalTransform) -> Self {
        Self {
            model: transform.compute_matrix(),
        }
    }
}
#[derive(Component, Deref)]
pub struct GlyphModelUniforms(pub UniformBuffer<GlyphModelUniform>);

fn extract_glyph_sprite_transform(
    mut commands: Commands,
    q_glyph_sprite: Extract<Query<(Entity, &GlobalTransform), &GlyphSprite>>,
) {
    for (entity, global_transform) in q_glyph_sprite.iter() {
        commands.insert_or_spawn_batch([(entity, global_transform.clone())]);
    }
}

fn prepare_buffers(
    mut commands: Commands,
    query: Query<(Entity, &GlyphSprite, &GlobalTransform)>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    gpu_glyph_textures: Res<RenderAssets<GlyphTexture>>,
    gpu_atlas_data: Res<RenderAssets<Atlas>>,
) {
    for (entity, sprite, global_transform) in query.iter() {
        let gpu_glyph_texture = gpu_glyph_textures
            .get(sprite.texture.clone())
            .expect("No gpu glyph texture attached to sprite");

        let atlas_buffers = gpu_atlas_data
            .get(sprite.atlas.clone())
            .expect("No atlas attached to sprite")
            .clone();

        let mut uniform_buffer = UniformBuffer::from(GlyphGenerationUniforms {
            color: sprite.color.into(),
            width: gpu_glyph_texture.width,
            height: gpu_glyph_texture.height,
        });
        uniform_buffer.write_buffer(&render_device, &render_queue);

        let mut model_uniform_buffer =
            UniformBuffer::from(GlyphModelUniform::new(global_transform.clone()));
        model_uniform_buffer.write_buffer(&render_device, &render_queue);

        let glyph_storage_texture = gpu_glyph_texture.storage_texture.clone();

        const VERTEX_PER_GLYPH: u64 = 6;
        const NUM_F32: u64 = 4;
        const F32_SIZE: u64 = 4;

        let vertex_buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("glyph vertex buffer"),
            size: (gpu_glyph_texture.width * gpu_glyph_texture.height) as u64
                * VERTEX_PER_GLYPH
                * NUM_F32
                * F32_SIZE,
            usage: BufferUsages::STORAGE
                | BufferUsages::COPY_SRC
                | BufferUsages::COPY_DST
                | BufferUsages::VERTEX,
            mapped_at_creation: true,
        });

        commands.entity(entity).insert((
            GylphGenerationUniformBuffer(uniform_buffer),
            GlyphModelUniforms(model_uniform_buffer),
            GlyphTextureInfo {
                width: gpu_glyph_texture.width,
                height: gpu_glyph_texture.height,
            },
            GlyphStorageTexture(glyph_storage_texture),
            atlas_buffers,
            GlyphVertexBuffer(vertex_buffer),
        ));
    }
}

#[derive(Resource)]
struct GlyphGenerationPipelineData {
    pipeline_id: CachedComputePipelineId,
    bind_group_layout: BindGroupLayout,
}

impl FromWorld for GlyphGenerationPipelineData {
    fn from_world(render_world: &mut World) -> Self {
        let asset_server = render_world.get_resource::<AssetServer>().unwrap();

        let bind_group_layout = render_world
            .resource::<RenderDevice>()
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("glyph generation bind group layout"),
                entries: &[
                    // UNIFORMS
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: BufferSize::new(
                                GlyphGenerationUniforms::SHADER_SIZE.into(),
                            ),
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
                ],
            });

        let generate_glyphs_shader = asset_server.load("shaders/glyph_generation.wgsl");

        let trace_pipeline_descriptor = ComputePipelineDescriptor {
            label: Some("glyph generation pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            entry_point: "compute".into(),
            shader: generate_glyphs_shader,
            shader_defs: Vec::new(),
            push_constant_ranges: Vec::new(),
        };

        let cache = render_world.resource::<PipelineCache>();
        let pipeline_id = cache.queue_compute_pipeline(trace_pipeline_descriptor);

        GlyphGenerationPipelineData {
            pipeline_id,
            bind_group_layout,
        }
    }
}
