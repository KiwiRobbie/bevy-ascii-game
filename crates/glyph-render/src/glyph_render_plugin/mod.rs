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
use bytemuck::{cast_slice_mut, Pod, Zeroable};
pub(crate) use node::GlyphGenerationNode;
use spatial_grid::{depth::Depth, grid::SpatialGrid};
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
pub(crate) struct GlyphGeneration;

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
                    .chain()
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
                    bevy::core_pipeline::core_2d::graph::Node2d::EndMainPass,
                    GlyphGeneration,
                    bevy::core_pipeline::core_2d::graph::Node2d::Bloom,
                ),
            );
        // .add_systems(Render, debug);
    }
    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .init_resource::<GlyphPipelineData>();
    }
}

#[derive(Clone, ShaderType)]
pub(crate) struct GlyphRasterUniforms {
    pub(crate) color: Vec4,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) advance: u32,
    pub(crate) line_spacing: u32,
}

#[derive(Debug, Clone)]
pub struct GlyphTextureSource {
    pub width: usize,
    pub height: usize,
    pub data: Box<[char]>,
}

impl GlyphTextureSource {
    pub fn new(width: usize, height: usize, data: Box<[char]>) -> Self {
        assert_eq!(data.len(), width * height);
        Self {
            data,
            width,
            height,
        }
    }
    pub fn new_iter<I: IntoIterator<Item = char>>(width: usize, height: usize, iter: I) -> Self {
        let data = iter
            .into_iter()
            .take(width * height)
            .collect::<Box<[char]>>();
        assert_eq!(data.len(), width * height);
        Self {
            data,
            width,
            height,
        }
    }
}

#[derive(Asset, TypePath, Clone)]
pub struct GlyphTexture {
    pub source: Arc<GlyphTextureSource>,
}

impl GlyphTexture {
    pub fn new(source: Arc<GlyphTextureSource>) -> Self {
        Self { source: source }
    }
    pub fn size(&self) -> UVec2 {
        UVec2 {
            x: self.source.width as u32,
            y: self.source.height as u32,
        }
    }
}

impl From<GlyphTextureSource> for GlyphTexture {
    fn from(value: GlyphTextureSource) -> Self {
        Self {
            source: Arc::new(value),
        }
    }
}

impl From<&Vec<String>> for GlyphTextureSource {
    fn from(from: &Vec<String>) -> Self {
        let height = from.len();
        let width = from[0].chars().count();

        let mut data: Box<[char]> = vec!['\0'; width * height].into_boxed_slice();
        let mut index = 0;
        for row in from.into_iter() {
            // assert_eq!(row.chars().count(), width);
            for ch in row.chars() {
                data[index] = ch;
                index += 1;
            }
        }

        Self {
            width,
            height,
            data,
        }
    }
}

impl From<&Vec<String>> for GlyphTexture {
    fn from(from: &Vec<String>) -> Self {
        Self {
            source: Arc::new(from.into()),
        }
    }
}
impl From<Vec<String>> for GlyphTexture {
    fn from(from: Vec<String>) -> Self {
        Self {
            source: Arc::new((&from).into()),
        }
    }
}

#[derive(Clone)]
pub struct ExtractedGlyphTextureSource {
    pub(crate) data: Box<[u8]>,

    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl ExtractedGlyphTextureSource {
    pub(crate) fn from_texture_data(
        texture: &GlyphTextureSource,
        atlas: &FontAtlasSource,
        font: FontRef,
        color: Color,
    ) -> Self {
        let mut data: Box<[u8]> = vec![0; 4 * 4 * texture.width * texture.height].into();
        let charmap = font.charmap();

        for (source_index, c) in texture.data.iter().copied().enumerate() {
            let x = source_index % texture.width;
            let y = texture.height - source_index / texture.width - 1;
            let position = x + texture.width * y;
            let index: usize = 16 * position;

            let glyph_id = atlas
                .local_index
                .get(&charmap.map(c))
                .unwrap_or(&if c == '·' { u16::MAX - 1 } else { u16::MAX })
                .to_le_bytes();

            data[index + 4..index + 16].copy_from_slice(cast_slice_mut(
                &mut color.to_srgba().to_f32_array_no_alpha(),
            ));

            data[index + 0] = glyph_id[0];
            data[index + 1] = glyph_id[1];
        }

        Self {
            data,
            width: texture.width as u32,
            height: texture.height as u32,
        }
    }
}

#[derive(Component, Clone, ExtractComponent, Debug)]
pub struct GlyphSolidColor {
    pub color: Color,
}

#[derive(Component, DerefMut, Deref)]
pub(crate) struct GpuGlyphTexture(pub(crate) Arc<PreparedGlyphTextureSource>);

pub(crate) struct PreparedGlyphTextureSource {
    pub(crate) buffer_texture: Texture,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[derive(Component)]
pub(crate) struct GlyphTextureInfo {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[derive(ShaderType)]
pub(crate) struct GlyphModelUniform {
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
pub(crate) struct ExtractedAtlas(pub(crate) Arc<FontAtlasSource>);

#[derive(Component, Deref)]
pub(crate) struct GlyphModelUniformBuffer(pub(crate) UniformBuffer<GlyphModelUniform>);

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

pub(crate) struct GpuGlyphItem {
    pub(crate) start: UVec2,
    pub(crate) size: UVec2,
    pub(crate) offset: IVec2,
    pub(crate) padding: Vec2,
    pub(crate) color: Vec4,
}

#[derive(Component, Deref)]
pub(crate) struct AtlasGpuData(pub(crate) Arc<AtlasGpuDataSource>);

pub(crate) struct AtlasGpuDataSource {
    pub(crate) data: Texture,
    pub(crate) uvs: Texture,
}

fn prepare_buffers(
    mut commands: Commands,
    q_textures: Query<(
        Entity,
        Option<&GlyphSolidColor>,
        &GlobalTransform,
        &GpuGlyphTexture,
        &SpatialGrid,
    )>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    for (entity, color, global_transform, gpu_glyph_texture, grid) in q_textures.iter() {
        let mut uniform_buffer = UniformBuffer::from(GlyphRasterUniforms {
            color: color
                .map(|color| color.color.to_srgba().to_vec4())
                .unwrap_or(Color::WHITE.to_srgba().to_vec4()),
            width: gpu_glyph_texture.width,
            height: gpu_glyph_texture.height,
            advance: grid.step.x,
            line_spacing: grid.step.y,
        });
        uniform_buffer.set_label(Some("Glyph raster uniforms"));
        uniform_buffer.write_buffer(&render_device, &render_queue);

        let mut model_uniform_buffer =
            UniformBuffer::from(GlyphModelUniform::new(*global_transform));
        model_uniform_buffer.set_label(Some("Glyph raster model uniforms"));
        model_uniform_buffer.write_buffer(&render_device, &render_queue);

        let glyph_buffer_texture = gpu_glyph_texture.buffer_texture.clone();

        commands.entity(entity).insert((
            GlyphUniformBuffer(uniform_buffer),
            GlyphModelUniformBuffer(model_uniform_buffer),
            GlyphTextureInfo {
                width: gpu_glyph_texture.width,
                height: gpu_glyph_texture.height,
            },
            GlyphBufferData {
                buffer: glyph_buffer_texture,
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
                zero_initialize_workgroup_memory: false,
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
                zero_initialize_workgroup_memory: false,
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
pub(crate) struct GlyphRenderUniformBuffer(pub(crate) UniformBuffer<GlyphRenderUniforms>);
#[derive(Clone, ShaderType)]
pub(crate) struct GlyphRenderUniforms {
    pub(crate) position: IVec2,
    pub(crate) size: UVec2,
    pub(crate) target_size: UVec2,
    pub(crate) depth: f32,
    pub(crate) padding: f32,
}
