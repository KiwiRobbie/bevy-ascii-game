use std::sync::Arc;

use bevy::{
    prelude::*,
    render::{
        render_graph::RenderGraphApp,
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
        Extract, Render, RenderApp, RenderSet,
    },
};
use bytemuck::{cast_slice, Pod, Zeroable};
pub use node::GlyphGenerationNode;
use swash::FontRef;

use crate::{
    atlas::{FontAtlasCache, FontAtlasSource},
    font::{CustomFont, CustomFontSource, FontLoadedMarker, FontSize},
    glyph_animation::{GlyphAnimation, GlyphAnimationSource},
    glyph_render_plugin::render_resources::{
        GlyphStorageTexture, GlyphUniformBuffer, GlyphVertexBuffer,
    },
};

use self::{
    generation_descriptors::get_bind_group_layout_descriptor,
    raster_descriptors::RASTER_BINDGROUP_LAYOUT,
};

mod generation_descriptors;
mod node;
mod raster_descriptors;
mod render_resources;

pub struct GlyphRenderPlugin;
const MAIN_GRAPH_2D: &str = bevy::core_pipeline::core_2d::graph::NAME;

impl Plugin for GlyphRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GlyphTexture>();

        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .add_systems(
                ExtractSchedule,
                (extract_glyph_sprites, extract_glyph_animations),
            )
            .add_systems(
                Render,
                (prepare_atlas_buffers, prepare_glyph_textures).in_set(RenderSet::PrepareAssets),
            )
            .add_systems(Render, (prepare_buffers,).in_set(RenderSet::Prepare))
            .add_render_graph_node::<GlyphGenerationNode>(MAIN_GRAPH_2D, "glyph_generation")
            .add_render_graph_edges(
                MAIN_GRAPH_2D,
                &[
                    bevy::core_pipeline::core_2d::graph::node::TONEMAPPING,
                    "glyph_generation",
                    bevy::core_pipeline::core_2d::graph::node::END_MAIN_PASS_POST_PROCESSING,
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
pub struct GlyphTexture {
    pub data: Vec<String>,
}

#[derive(Component, Clone)]
pub struct ExtractedGlyphTexture {
    pub data: Box<[u8]>,
    pub width: u32,
    pub height: u32,

    pub advance: u32,
    pub line_spacing: u32,
}

impl ExtractedGlyphTexture {
    pub fn extract(source: &GlyphTexture, atlas: &FontAtlasSource, font: FontRef) -> Self {
        let text = &source.data;
        let height = text.len();
        let width = text[0].len();

        let mut data: Box<[u8]> = vec![0; 2 * width * height].into();
        let charmap = font.charmap();

        for (y, chars) in text.iter().enumerate() {
            assert_eq!(text[y].len(), width);
            for (x, c) in chars.chars().enumerate() {
                let index = 2 * (x + (height - y - 1) * width);
                let glyph_id = atlas.local_index.get(&charmap.map(c)).unwrap_or(&u16::MAX);
                data[index..index + 2].copy_from_slice(&glyph_id.to_le_bytes());
            }
        }

        Self {
            data,
            width: width as u32,
            height: height as u32,
            advance: 19u32,
            line_spacing: 40u32,
        }
    }

    pub fn extract_animation(
        source: &GlyphAnimationSource,
        atlas: &FontAtlasSource,
        font: FontRef,
        frame: usize,
    ) -> Self {
        let text = &source.frames[frame].data;
        let height = text.len();
        let width = text[0].len();

        let mut data: Box<[u8]> = vec![0; 2 * width * height].into();
        let charmap = font.charmap();

        for (y, chars) in text.iter().enumerate() {
            assert_eq!(text[y].len(), width);
            for (x, c) in chars.chars().enumerate() {
                let index = 2 * (x + (height - y - 1) * width);
                let glyph_id = atlas.local_index.get(&charmap.map(c)).unwrap_or(&u16::MAX);
                data[index..index + 2].copy_from_slice(&glyph_id.to_le_bytes());
            }
        }

        Self {
            data,
            width: width as u32,
            height: height as u32,
            advance: 19u32,
            line_spacing: 40u32,
        }
    }
}

#[derive(Component, Clone)]
pub struct GlyphSprite {
    pub color: Color,
    pub texture: Handle<GlyphTexture>,
    pub offset: IVec2,
}

#[derive(Component)]
pub struct GpuGlyphTexture {
    pub storage_texture: Texture,
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

fn extract_glyph_sprites(
    mut commands: Commands,
    atlas_cache: Extract<Res<FontAtlasCache>>,
    fonts: Extract<Res<Assets<CustomFontSource>>>,
    glyph_textures: Extract<Res<Assets<GlyphTexture>>>,
    q_glyph_sprite: Extract<
        Query<
            (
                Entity,
                &GlobalTransform,
                &GlyphSprite,
                &CustomFont,
                &FontSize,
            ),
            (With<FontLoadedMarker>, Without<GlyphAnimation>),
        >,
    >,
) {
    for (entity, global_transform, sprite, font, font_size) in q_glyph_sprite.iter() {
        let transform: Transform = (*global_transform).into();
        // let offset_transform: GlobalTransform = (transform
        //     * Transform::from_translation(Vec3::new(
        //         (19 * sprite.offset.x) as f32,
        //         (40 * sprite.offset.y) as f32,
        //         0.0,
        //     )))
        // .into();

        let offset_transform: GlobalTransform = transform.into();

        let font = fonts.get(font.id()).unwrap();

        let atlas = atlas_cache
            .cached
            .get(&(font_size.clone(), font.key()))
            .unwrap();

        let extracted_glyph_texture = ExtractedGlyphTexture::extract(
            glyph_textures.get(sprite.texture.id()).unwrap(),
            atlas,
            font.as_ref(),
        );

        commands.insert_or_spawn_batch([(
            entity,
            (
                offset_transform,
                sprite.clone(),
                ExtractedAtlas(atlas.clone()),
                font_size.clone(),
                extracted_glyph_texture,
            ),
        )]);
    }
}

fn extract_glyph_animations(
    mut commands: Commands,
    atlas_cache: Extract<Res<FontAtlasCache>>,
    fonts: Extract<Res<Assets<CustomFontSource>>>,
    glyph_animations: Extract<Res<Assets<GlyphAnimationSource>>>,
    q_glyph_animations: Extract<
        Query<
            (
                Entity,
                &GlobalTransform,
                &GlyphAnimation,
                &CustomFont,
                &FontSize,
            ),
            (With<FontLoadedMarker>, Without<GlyphSprite>),
        >,
    >,
) {
    for (entity, global_transform, animation, font, font_size) in q_glyph_animations.iter() {
        let Some(animation_source) = glyph_animations.get(animation.source.id()) else {
            continue;
        };
        let transform: Transform = (*global_transform).into();

        // TODO: Add frame offsets to glyph animations
        let offset_transform: GlobalTransform = transform.into();

        // let offset_transform: GlobalTransform = (transform
        //     * Transform::from_translation(Vec3::new(
        //         sprite.offset.x as f32,
        //         sprite.offset.y as f32,
        //         0.0,
        //     )))
        // .into();

        let font = fonts.get(font.id()).unwrap();

        let atlas = atlas_cache
            .cached
            .get(&(font_size.clone(), font.key()))
            .unwrap();

        let extracted_glyph_texture = ExtractedGlyphTexture::extract_animation(
            animation_source,
            atlas,
            font.as_ref(),
            animation.frame as usize,
        );

        commands.insert_or_spawn_batch([(
            entity,
            (
                offset_transform,
                animation.clone(),
                ExtractedAtlas(atlas.clone()),
                font_size.clone(),
                extracted_glyph_texture,
            ),
        )]);
    }
}

fn prepare_atlas_buffers(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    q_glyph_sprite: Query<(Entity, &ExtractedAtlas)>,
) {
    for (entity, atlas) in q_glyph_sprite.iter() {
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
                usage: TextureUsages::STORAGE_BINDING | TextureUsages::COPY_SRC,
                view_formats: &[TextureFormat::Rgba8Unorm],
            },
            &atlas.data,
        );

        let uvs = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("gpu font atlas uv buffer"),
            usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
            contents: cast_slice(
                &atlas
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

        commands
            .entity(entity)
            .insert(AtlasGpuBuffers { data, uvs });
    }
}

fn prepare_glyph_textures(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    q_extracted_glyph_texture: Query<(Entity, &ExtractedGlyphTexture)>,
) {
    for (entity, extracted_texture) in q_extracted_glyph_texture.iter() {
        let storage_texture = render_device.create_texture_with_data(
            &render_queue,
            &TextureDescriptor {
                label: Some("glyph texture"),
                size: Extent3d {
                    width: extracted_texture.width,
                    height: extracted_texture.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::R16Uint,
                usage: TextureUsages::COPY_SRC | TextureUsages::STORAGE_BINDING,
                view_formats: &[TextureFormat::R16Uint],
            },
            &extracted_texture.data,
        );
        commands.entity(entity).insert(GpuGlyphTexture {
            storage_texture,
            width: extracted_texture.width,
            height: extracted_texture.height,
        });
    }
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

fn prepare_buffers(
    mut commands: Commands,
    query: Query<(
        Entity,
        Option<&GlyphSprite>,
        &GlobalTransform,
        &GpuGlyphTexture,
        &ExtractedGlyphTexture,
    )>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    for (entity, sprite, global_transform, gpu_glyph_texture, extracted_glyph_texture) in
        query.iter()
    {
        let mut uniform_buffer = UniformBuffer::from(GlyphUniforms {
            color: sprite
                .map(|sprite| sprite.color.into())
                .unwrap_or(Color::WHITE.into()),
            width: gpu_glyph_texture.width,
            height: gpu_glyph_texture.height,
            advance: extracted_glyph_texture.advance,
            line_spacing: extracted_glyph_texture.line_spacing,
        });
        uniform_buffer.write_buffer(&render_device, &render_queue);

        let mut model_uniform_buffer =
            UniformBuffer::from(GlyphModelUniform::new(*global_transform));
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
            GlyphUniformBuffer(uniform_buffer),
            GlyphModelUniformBuffer(model_uniform_buffer),
            GlyphTextureInfo {
                width: gpu_glyph_texture.width,
                height: gpu_glyph_texture.height,
            },
            GlyphStorageTexture(glyph_storage_texture),
            GlyphVertexBuffer(vertex_buffer),
        ));
    }
}

#[derive(Resource)]
struct GlyphPipelineData {
    glyph_generation_pipeline_id: CachedComputePipelineId,
    glyph_generation_bind_group_layout: BindGroupLayout,
    glyph_raster_pipeline_id: CachedRenderPipelineId,
    glyph_raster_bind_group_layout: BindGroupLayout,
}

impl FromWorld for GlyphPipelineData {
    fn from_world(render_world: &mut World) -> Self {
        let asset_server = render_world.get_resource::<AssetServer>().unwrap();

        let (glyph_generation_pipeline_id, glyph_generation_bind_group_layout) = {
            let glyph_generation_bind_group_layout = render_world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&get_bind_group_layout_descriptor());

            let glyph_generation_shader = asset_server.load("shaders/glyph_generation.wgsl");

            let trace_pipeline_descriptor = ComputePipelineDescriptor {
                label: Some("glyph generation pipeline".into()),
                layout: vec![glyph_generation_bind_group_layout.clone()],
                entry_point: "compute".into(),
                shader: glyph_generation_shader,
                shader_defs: Vec::new(),
                push_constant_ranges: Vec::new(),
            };

            let cache = render_world.resource::<PipelineCache>();
            let glyph_generation_pipeline_id =
                cache.queue_compute_pipeline(trace_pipeline_descriptor);

            (
                glyph_generation_pipeline_id,
                glyph_generation_bind_group_layout,
            )
        };

        let (glyph_raster_pipeline_id, glyph_raster_bind_group_layout) = {
            let glyph_raster_bind_group_layout = render_world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("glyph generation bind group layout"),
                    entries: &RASTER_BINDGROUP_LAYOUT,
                });

            let glyph_raster_shader = asset_server.load("shaders/glyph_raster.wgsl");

            let raster_pipeline_descriptor = RenderPipelineDescriptor {
                label: Some("glyph generation pipeline".into()),
                layout: vec![glyph_raster_bind_group_layout.clone()],
                push_constant_ranges: Vec::new(),
                vertex: VertexState {
                    shader: glyph_raster_shader.clone(),
                    shader_defs: Vec::new(),
                    entry_point: "vertex".into(),
                    buffers: vec![VertexBufferLayout {
                        array_stride: 4 * 4,
                        step_mode: VertexStepMode::Vertex,
                        attributes: vec![
                            VertexAttribute {
                                format: VertexFormat::Float32x2,
                                offset: 0,
                                shader_location: 0,
                            },
                            VertexAttribute {
                                format: VertexFormat::Float32x2,
                                offset: 2 * 4,
                                shader_location: 1,
                            },
                        ],
                    }],
                },
                fragment: Some(FragmentState {
                    shader: glyph_raster_shader,
                    shader_defs: Vec::new(),
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::Rgba8UnormSrgb,
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
            glyph_generation_pipeline_id,
            glyph_generation_bind_group_layout,
            glyph_raster_pipeline_id,
            glyph_raster_bind_group_layout,
        }
    }
}
