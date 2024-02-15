use std::sync::Arc;

use bevy::{
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        render_graph::RenderGraphApp,
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
        Render, RenderApp, RenderSet,
    },
};
use bytemuck::{cast_slice, Pod, Zeroable};
pub use node::GlyphGenerationNode;
use spatial_grid::grid::SpatialGrid;
use swash::FontRef;

use crate::{
    atlas::FontAtlasSource,
    font::FontSize,
    glyph_buffer::{
        extract::extract_glyph_buffers, prepare::prepare_glyph_buffers,
        update_glyph_buffer_entities,
    },
    glyph_render_plugin::render_resources::{GlyphBufferData, GlyphUniformBuffer},
};

use self::raster_descriptors::{raster_bind_group_layout, render_bind_group_layout};

mod node;
mod raster_descriptors;
mod render_resources;

pub struct GlyphRenderPlugin;
const MAIN_GRAPH_2D: &str = bevy::core_pipeline::core_2d::graph::NAME;

impl Plugin for GlyphRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GlyphTextureSource>()
            .add_systems(Last, update_glyph_buffer_entities);
        app.get_sub_app_mut(RenderApp)
            .unwrap()
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
            .add_render_graph_node::<GlyphGenerationNode>(MAIN_GRAPH_2D, "glyph_generation")
            .add_render_graph_edges(
                MAIN_GRAPH_2D,
                &[
                    bevy::core_pipeline::core_2d::graph::node::MAIN_PASS,
                    "glyph_generation",
                    bevy::core_pipeline::core_2d::graph::node::BLOOM,
                ],
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

#[derive(Asset, TypePath, Clone)]
pub struct GlyphTextureSource {
    pub data: Vec<String>,
}

impl GlyphTextureSource {
    pub fn size(&self) -> UVec2 {
        UVec2 {
            x: self.data[0].len() as u32,
            y: self.data.len() as u32,
        }
    }
}

#[derive(Component, Clone)]
pub struct ExtractedGlyphTexture {
    pub data: Box<[u16]>,

    pub width: u32,
    pub height: u32,

    pub advance: u32,
    pub line_spacing: u32,
}

impl ExtractedGlyphTexture {
    pub fn from_text_data(
        text: &Vec<String>,
        atlas: &FontAtlasSource,
        font: FontRef,
        font_size: &FontSize,
    ) -> Self {
        let height = text.len();
        let width = text[0].len();

        let mut data: Box<[u16]> = vec![0; width * height].into();
        let charmap = font.charmap();

        for (y, chars) in text.iter().enumerate() {
            assert_eq!(text[y].len(), width);
            for (x, c) in chars.chars().enumerate() {
                let index = x + (height - y - 1) * width;
                let glyph_id = atlas.local_index.get(&charmap.map(c)).unwrap_or(&u16::MAX);
                data[index] = *glyph_id;
            }
        }

        Self {
            data,
            width: width as u32,
            height: height as u32,
            advance: font_size.advance(),
            line_spacing: font_size.line_spacing(),
        }
    }
}

#[derive(Component, Clone)]
pub struct GlyphSprite {
    pub texture: Handle<GlyphTextureSource>,
    pub offset: IVec2,
}

#[derive(Component, Clone, ExtractComponent)]
pub struct GlyphSolidColor {
    pub color: Color,
}

#[derive(Component)]
pub struct GpuGlyphTexture {
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
) {
    for (entity, atlas) in q_atlases.iter() {
        let uv_texture_size =
            (((atlas.items.len() * 3) as f64).sqrt().ceil() as u32).next_power_of_two();

        let mut data = vec![0u8; 4 * 2 * (uv_texture_size as usize * uv_texture_size as usize)]
            .into_boxed_slice();
        let item_data = cast_slice(&atlas.items);
        data[0..item_data.len()].copy_from_slice(item_data);

        let uvs = render_device.create_texture_with_data(
            &render_queue,
            &TextureDescriptor {
                label: Some("gpu font atlas uv texture"),
                size: Extent3d {
                    width: uv_texture_size,
                    height: uv_texture_size,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: bevy::render::render_resource::TextureDimension::D2,
                format: TextureFormat::Rg32Uint,
                usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC,
                view_formats: &[TextureFormat::Rg32Uint],
            },
            &data,
        );

        let data = render_device.create_texture_with_data(
            &render_queue,
            &TextureDescriptor {
                label: Some("gpu font atlas storage texture"),
                size: Extent3d {
                    width: atlas.size,
                    height: atlas.size,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: bevy::render::render_resource::TextureDimension::D2,
                format: TextureFormat::Rgba8Unorm,
                usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC,
                view_formats: &[TextureFormat::Rgba8Unorm],
            },
            &atlas.data,
        );

        commands.entity(entity).insert(AtlasGpuData { data, uvs });
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

#[derive(Component)]
pub struct AtlasGpuData {
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
                .map(|color| color.color.into())
                .unwrap_or(Color::WHITE.into()),
            width: gpu_glyph_texture.width,
            height: gpu_glyph_texture.height,
            advance: grid.size.x,
            line_spacing: grid.size.y,
        });
        uniform_buffer.write_buffer(&render_device, &render_queue);

        let mut model_uniform_buffer =
            UniformBuffer::from(GlyphModelUniform::new(*global_transform));
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

            let glyph_render_bind_group_layout =
                render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("glyph render bind group layout"),
                    entries: &render_bind_group_layout(),
                });
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
                        format: TextureFormat::R16Uint,
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

            let glyph_raster_bind_group_layout =
                render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("glyph raster bind group layout"),
                    entries: &raster_bind_group_layout(),
                });
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
}
