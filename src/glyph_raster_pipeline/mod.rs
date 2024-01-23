use bevy::{
    prelude::*,
    render::{render_resource::*, renderer::RenderDevice, view::ViewUniform},
};
pub use node::GlyphRasterNode;

pub mod node;

#[derive(ShaderType)]
struct GlyphRasterUniforms {
    color: Color,
}

#[derive(Resource)]
pub struct GlyphRasterPipelineData {
    pipeline_id: CachedRenderPipelineId,
    bind_group_layout: BindGroupLayout,
}

impl FromWorld for GlyphRasterPipelineData {
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
                        visibility: ShaderStages::VERTEX_FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: BufferSize::new(ViewUniform::SHADER_SIZE.into()),
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::VERTEX_FRAGMENT,
                        ty: BindingType::StorageTexture {
                            access: StorageTextureAccess::ReadOnly,
                            format: TextureFormat::Rgba8Unorm,
                            view_dimension: TextureViewDimension::D2,
                        },
                        count: None,
                    },
                ],
            });

        let raster_glyphs_shader = asset_server.load("shaders/glyph_raster.wgsl");

        let raster_pipeline_descriptor = RenderPipelineDescriptor {
            label: Some("glyph generation pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            vertex: VertexState {
                shader: raster_glyphs_shader.clone(),
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
                shader: raster_glyphs_shader,
                shader_defs: Vec::new(),
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba8UnormSrgb,
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

        GlyphRasterPipelineData {
            pipeline_id,
            bind_group_layout,
        }
    }
}
