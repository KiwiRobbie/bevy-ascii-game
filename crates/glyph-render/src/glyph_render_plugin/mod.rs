use std::sync::Arc;

use bevy::{
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        render_graph::{RenderGraphApp, RenderLabel},
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
        Render, RenderApp, RenderSet,
    },
};
use bytemuck::{cast_slice, cast_slice_mut, Pod, Zeroable};
pub use node::GlyphGenerationNode;
use spatial_grid::grid::SpatialGrid;
use swash::FontRef;

use crate::{
    atlas::FontAtlasSource,
    glyph_buffer::{
        extract::extract_glyph_buffers, prepare::prepare_glyph_buffers,
        update_glyph_buffer_entities,
    },
    glyph_render_plugin::render_resources::{GlyphBufferData, GlyphUniformBuffer},
    glyph_texture::{PreparedAtlasCache, RenderGlyphTextureCachePlugin},
};

use self::raster_descriptors::{raster_bind_group_layout, render_bind_group_layout};

mod node;
mod raster_descriptors;
mod render_resources;

#[derive(RenderLabel, Hash, Debug, PartialEq, Eq, Clone)]
pub struct GlyphGeneration;

pub struct GlyphRenderPlugin;
const MAIN_GRAPH_2D: bevy::core_pipeline::core_2d::graph::Core2d =
    bevy::core_pipeline::core_2d::graph::Core2d;

impl Plugin for GlyphRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GlyphTexture>()
            .add_systems(Last, update_glyph_buffer_entities);
        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .add_plugins(RenderGlyphTextureCachePlugin)
            .add_systems(ExtractSchedule, (extract_glyph_buffers,))
            .add_systems(
                Render,
                (update_glyph_buffer_entities, prepare_glyph_buffers)
                    .in_set(RenderSet::PrepareAssets),
            )
            .add_systems(
                Render,
                (prepare_atlas_buffers, prepare_buffers).in_set(RenderSet::Prepare),
            )
            .add_render_graph_node::<GlyphGenerationNode>(MAIN_GRAPH_2D, GlyphGeneration)
            .add_render_graph_edges(
                MAIN_GRAPH_2D,
                (
                    bevy::core_pipeline::core_2d::graph::Node2d::MainPass,
                    GlyphGeneration,
                    bevy::core_pipeline::core_2d::graph::Node2d::Bloom,
                ),
            );
    }
    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .init_resource::<GlyphPipelineData>();
    }
}

#[derive(Clone, ShaderType)]
pub struct GlyphUniforms {
    pub color: Vec4,
    pub width: u32,
    pub height: u32,
    pub advance: u32,
    pub line_spacing: u32,
}

#[derive(Debug, Clone)]
pub struct GlyphTextureSource {
    pub data: Vec<String>,
}

#[derive(Asset, TypePath, Clone)]
pub struct GlyphTexture {
    pub source: Arc<GlyphTextureSource>,
}

impl GlyphTexture {
    pub fn new(data: Vec<String>) -> Self {
        Self {
            source: Arc::new(GlyphTextureSource { data }),
        }
    }

    pub fn size(&self) -> UVec2 {
        UVec2 {
            x: self.source.as_ref().data[0].len() as u32,
            y: self.source.as_ref().data.len() as u32,
        }
    }
}

#[derive(Clone)]
pub struct ExtractedGlyphTextureSource {
    pub data: Box<[u8]>,

    pub width: u32,
    pub height: u32,
}

impl ExtractedGlyphTextureSource {
    pub fn from_text_data(
        text: &Vec<String>,
        atlas: &FontAtlasSource,
        font: FontRef,
        color: Color,
    ) -> Self {
        let height = text.len();
        let width = text[0].chars().count();

        let mut data: Box<[u8]> = vec![0; 4 * 4 * width * height].into();
        let charmap = font.charmap();

        for (y, chars) in text.iter().enumerate() {
            assert_eq!(chars.chars().count(), width);
            for (x, c) in chars.chars().enumerate() {
                let index = 16 * (x + (height - y - 1) * width);
                let glyph_id = atlas
                    .local_index
                    .get(&charmap.map(c))
                    .unwrap_or(&if c == '·' { u16::MAX - 1 } else { u16::MAX })
                    .to_le_bytes();

                data[index + 4..index + 16]
                    .copy_from_slice(cast_slice_mut(&mut color.as_rgba_f32()[0..3]));

                data[index + 0] = glyph_id[0];
                data[index + 1] = glyph_id[1];
            }
        }

        Self {
            data,
            width: width as u32,
            height: height as u32,
        }
    }
}

#[derive(Component, Clone, ExtractComponent, Debug)]
pub struct GlyphSolidColor {
    pub color: Color,
}

#[derive(Component, DerefMut, Deref)]
pub struct GpuGlyphTexture(pub Arc<PreparedGlyphTextureSource>);

pub struct PreparedGlyphTextureSource {
    pub buffer_texture: Texture,
    pub width: u32,
    pub height: u32,
}

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
pub struct ExtractedAtlas(pub Arc<FontAtlasSource>);

#[derive(Component, Deref)]
pub struct GlyphModelUniformBuffer(pub UniformBuffer<GlyphModelUniform>);

#[derive(Debug, Component, Clone)]
pub struct GlyphSpriteMirrored;

fn prepare_atlas_buffers(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    q_atlases: Query<(Entity, &ExtractedAtlas)>,
    mut cache: ResMut<PreparedAtlasCache>,
) {
    for (entity, atlas) in q_atlases.iter() {
        commands
            .entity(entity)
            .insert(AtlasGpuData(cache.get_or_create(
                atlas,
                &render_device,
                &render_queue,
            )));
    }
}

#[derive(ShaderType, Pod, Clone, Copy, Zeroable)]
#[repr(C)]

pub struct GpuGlyphItem {
    pub start: UVec2,
    pub size: UVec2,
    pub offset: IVec2,
    pub padding: Vec2,
    pub color: Vec4,
}

#[derive(Component, Deref)]
pub struct AtlasGpuData(pub Arc<AtlasGpuDataSource>);

pub struct AtlasGpuDataSource {
    pub data: Texture,
    pub uvs: Texture,
}

fn prepare_buffers(
    mut commands: Commands,
    query: Query<(
        Entity,
        Option<&GlyphSolidColor>,
        &GlobalTransform,
        &GpuGlyphTexture,
        &SpatialGrid,
    )>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    for (entity, color, global_transform, gpu_glyph_texture, grid) in query.iter() {
        let mut uniform_buffer = UniformBuffer::from(GlyphUniforms {
            color: color
                .map(|color| color.color.rgba_to_vec4())
                .unwrap_or(Color::WHITE.rgba_to_vec4()),
            width: gpu_glyph_texture.width,
            height: gpu_glyph_texture.height,
            advance: grid.size.x,
            line_spacing: grid.size.y,
        });
        uniform_buffer.set_label(Some("Glyph raster uniforms"));
        uniform_buffer.write_buffer(&render_device, &render_queue);

        let mut model_uniform_buffer =
            UniformBuffer::from(GlyphModelUniform::new(*global_transform));
        model_uniform_buffer.set_label(Some("Glyph raster model uniforms"));
        model_uniform_buffer.write_buffer(&render_device, &render_queue);

        let glyph_buffer_texture = gpu_glyph_texture.buffer_texture.clone();

        let v_w = gpu_glyph_texture.width as f32 * grid.size.x as f32;
        let v_h = gpu_glyph_texture.height as f32 * grid.size.y as f32;

        let vertex = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: cast_slice(&[v_w, v_h]),
            usage: BufferUsages::VERTEX,
        });

        commands.entity(entity).insert((
            GlyphUniformBuffer(uniform_buffer),
            GlyphModelUniformBuffer(model_uniform_buffer),
            GlyphTextureInfo {
                width: gpu_glyph_texture.width,
                height: gpu_glyph_texture.height,
            },
            GlyphBufferData {
                buffer: glyph_buffer_texture,
                vertex,
            },
        ));
    }
}

#[derive(Resource)]
struct GlyphPipelineData {
    glyph_render_pipeline_id: CachedRenderPipelineId,
    glyph_render_bind_group_layout: BindGroupLayout,
    glyph_raster_pipeline_id: CachedRenderPipelineId,
    glyph_raster_bind_group_layout: BindGroupLayout,
}

impl FromWorld for GlyphPipelineData {
    fn from_world(render_world: &mut World) -> Self {
        let (glyph_render_pipeline_id, glyph_render_bind_group_layout) = {
            let render_device: Mut<RenderDevice> = render_world.get_resource_mut().unwrap();

            let glyph_render_bind_group_layout = render_device.create_bind_group_layout(
                Some("glyph render bind group layout"),
                &render_bind_group_layout(),
            );
            let glyph_render_shader = render_world
                .get_resource::<AssetServer>()
                .unwrap()
                .load("shaders/glyph_render.wgsl");

            let raster_pipeline_descriptor = RenderPipelineDescriptor {
                label: Some("glyph render pipeline".into()),
                layout: vec![glyph_render_bind_group_layout.clone()],
                push_constant_ranges: Vec::new(),
                vertex: VertexState {
                    shader: glyph_render_shader.clone(),
                    shader_defs: Vec::new(),
                    entry_point: "vertex".into(),
                    buffers: vec![],
                },
                fragment: Some(FragmentState {
                    shader: glyph_render_shader,
                    shader_defs: Vec::new(),
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::Rgba32Uint,
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: Default::default(),
            };

            let cache = render_world.resource::<PipelineCache>();
            let pipeline_id = cache.queue_render_pipeline(raster_pipeline_descriptor);

            (pipeline_id, glyph_render_bind_group_layout)
        };
        let (glyph_raster_pipeline_id, glyph_raster_bind_group_layout) = {
            let render_device: Mut<RenderDevice> = render_world.get_resource_mut().unwrap();

            let glyph_raster_bind_group_layout = render_device.create_bind_group_layout(
                Some("glyph raster bind group layout"),
                &raster_bind_group_layout(),
            );
            let glyph_raster_shader = render_world
                .get_resource::<AssetServer>()
                .unwrap()
                .load("shaders/glyph_raster.wgsl");

            let raster_pipeline_descriptor = RenderPipelineDescriptor {
                label: Some("glyph raster pipeline".into()),
                layout: vec![glyph_raster_bind_group_layout.clone()],
                push_constant_ranges: Vec::new(),
                vertex: VertexState {
                    shader: glyph_raster_shader.clone(),
                    shader_defs: Vec::new(),
                    entry_point: "vertex".into(),
                    buffers: vec![],
                },
                fragment: Some(FragmentState {
                    shader: glyph_raster_shader,
                    shader_defs: Vec::new(),
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::Rgba16Float,
                        blend: Some(BlendState::ALPHA_BLENDING),
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: Default::default(),
            };

            let cache = render_world.resource::<PipelineCache>();
            let pipeline_id = cache.queue_render_pipeline(raster_pipeline_descriptor);

            (pipeline_id, glyph_raster_bind_group_layout)
        };
        GlyphPipelineData {
            glyph_render_pipeline_id,
            glyph_render_bind_group_layout,
            glyph_raster_pipeline_id,
            glyph_raster_bind_group_layout,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct GlyphRenderUniformBuffer(pub UniformBuffer<GlyphRenderUniforms>);
#[derive(Clone, ShaderType)]
pub struct GlyphRenderUniforms {
    pub position: IVec2,
    pub size: UVec2,
    pub target_size: UVec2,
    pub depth: f32,
    pub padding: f32,
}
