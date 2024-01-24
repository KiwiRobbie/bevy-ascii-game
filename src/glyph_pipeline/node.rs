use crate::atlas::AtlasGpuBuffers;
use bevy::{
    ecs::{
        entity::Entity,
        query::QueryState,
        world::{FromWorld, World},
    },
    render::{
        render_graph,
        render_resource::{BindGroupEntry, ComputePassDescriptor, PipelineCache},
        view::{ViewTarget, ViewUniforms},
    },
};
use wgpu::{RenderPassColorAttachment, RenderPassDescriptor};

use super::{
    GlyphGenerationPipelineData, GlyphModelUniforms, GlyphSprite, GlyphStorageTexture,
    GlyphTextureInfo, GlyphVertexBuffer, GylphGenerationUniformBuffer,
};

type Q = (
    &'static GlyphSprite,
    &'static GlyphModelUniforms,
    &'static GylphGenerationUniformBuffer,
    &'static GlyphTextureInfo,
    &'static GlyphStorageTexture,
    &'static AtlasGpuBuffers,
    &'static GlyphVertexBuffer,
);
pub struct GlyphGenerationNode {
    query: QueryState<Q>,
    q_view: QueryState<&'static ViewTarget>,
    entities: Vec<Entity>,
}

impl GlyphGenerationNode {
    pub fn new(world: &mut World) -> Self {
        Self {
            q_view: world.query(),
            query: world.query(),
            entities: world.query_filtered::<Entity, Q>().iter(world).collect(),
        }
    }
}

impl FromWorld for GlyphGenerationNode {
    fn from_world(world: &mut World) -> Self {
        Self::new(world)
    }
}

impl render_graph::Node for GlyphGenerationNode {
    fn input(&self) -> Vec<render_graph::SlotInfo> {
        vec![]
    }
    fn output(&self) -> Vec<render_graph::SlotInfo> {
        vec![]
        // vec![render_graph::SlotInfo::new(
        //     "vertex_buffer",
        //     render_graph::SlotType::Buffer,
        // )]
    }
    fn update(&mut self, world: &mut World) {
        self.q_view = world.query();
        self.query = world.query();
        self.entities = world.query_filtered::<Entity, Q>().iter(world).collect();
    }

    fn run(
        &self,
        graph: &mut render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let Some(view_entity) = graph.get_view_entity() else {
            return Ok(());
        };
        let target = match self.q_view.get_manual(world, view_entity) {
            Ok(result) => result,
            Err(_) => panic!("Camera missing component!"),
        };

        let pipeline_cache = world.resource::<PipelineCache>();
        let generation_pipeline_data = world.get_resource::<GlyphGenerationPipelineData>().unwrap();

        let generation_pipeline = match pipeline_cache
            .get_compute_pipeline(generation_pipeline_data.glyph_generation_pipeline_id)
        {
            Some(pipeline) => pipeline,
            None => return Ok(()),
        };

        let raster_pipeline = match pipeline_cache
            .get_render_pipeline(generation_pipeline_data.glyph_raster_pipeline_id)
        {
            Some(pipeline) => pipeline,
            None => return Ok(()),
        };

        let glyph_generation_compute_pass_descriptor = ComputePassDescriptor {
            label: Some("glyph generation pass"),
        };
        let glyph_raster_render_pass_descriptor = RenderPassDescriptor {
            label: Some("glyph raster pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: target.main_texture_view(),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        };

        for (
            _glyph_sprite,
            _glyph_model_uniforms,
            glyph_uniform_buffer,
            glyph_texture_info,
            glyph_storage_texture,
            atlas_buffers,
            vertex_buffer,
        ) in self
            .entities
            .iter()
            .map(|e| self.query.get_manual(world, *e).unwrap())
        {
            let render_device = render_context.render_device();
            let glyph_generation_bind_group = glyph_generation_create_bind_group(
                render_device,
                generation_pipeline_data,
                glyph_uniform_buffer,
                glyph_storage_texture,
                atlas_buffers,
                vertex_buffer,
            );
            let mut glyph_generation_compute_pass = render_context
                .command_encoder()
                .begin_compute_pass(&glyph_generation_compute_pass_descriptor);
            // render_pass.set_bind_group(0, &voxel_data.bind_group, &[]);
            glyph_generation_compute_pass.set_bind_group(0, &glyph_generation_bind_group, &[]);

            glyph_generation_compute_pass.set_pipeline(generation_pipeline);
            glyph_generation_compute_pass.dispatch_workgroups(
                glyph_texture_info.width,
                glyph_texture_info.height,
                1,
            );
        }

        for (
            _glyph_sprite,
            glyph_model_uniforms,
            _glyph_uniform_buffer,
            glyph_texture_info,
            _glyph_storage_texture,
            atlas_buffers,
            vertex_buffer,
        ) in self
            .entities
            .iter()
            .map(|e| self.query.get_manual(world, *e).unwrap())
        {
            let render_device = render_context.render_device();
            let bind_group = render_device.create_bind_group(
                Some("glyph raster bind group"),
                &generation_pipeline_data.glyph_raster_bind_group_layout,
                &[
                    BindGroupEntry {
                        binding: 0,
                        resource: world.resource::<ViewUniforms>().uniforms.binding().unwrap(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: glyph_model_uniforms.binding().unwrap(),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: bevy::render::render_resource::BindingResource::TextureView(
                            &atlas_buffers
                                .data
                                .create_view(&wgpu::TextureViewDescriptor::default()),
                        ),
                    },
                ],
            );
            let mut render_pass = render_context
                .command_encoder()
                .begin_render_pass(&glyph_raster_render_pass_descriptor);

            render_pass.set_bind_group(0, &bind_group, &[]);

            render_pass.set_pipeline(raster_pipeline);

            render_pass.set_vertex_buffer(0, *vertex_buffer.slice(..));
            render_pass.draw(
                0..6 * glyph_texture_info.width * glyph_texture_info.height,
                0..1,
            );
            vertex_buffer.unmap();
        }
        Ok(())
    }
}

fn glyph_generation_create_bind_group(
    render_device: &bevy::render::renderer::RenderDevice,
    generation_pipeline_data: &GlyphGenerationPipelineData,
    glyph_uniform_buffer: &GylphGenerationUniformBuffer,
    glyph_storage_texture: &GlyphStorageTexture,
    atlas_buffers: &AtlasGpuBuffers,
    vertex_buffer: &GlyphVertexBuffer,
) -> bevy::render::render_resource::BindGroup {
    let bind_group = render_device.create_bind_group(
        Some("glyph generation bind group"),
        &generation_pipeline_data.glyph_generation_bind_group_layout,
        &[
            BindGroupEntry {
                binding: 0,
                resource: glyph_uniform_buffer.binding().unwrap(),
            },
            BindGroupEntry {
                binding: 1,
                resource: bevy::render::render_resource::BindingResource::TextureView(
                    &glyph_storage_texture.create_view(&wgpu::TextureViewDescriptor::default()),
                ),
            },
            BindGroupEntry {
                binding: 2,
                resource: bevy::render::render_resource::BindingResource::TextureView(
                    &atlas_buffers
                        .data
                        .create_view(&wgpu::TextureViewDescriptor::default()),
                ),
            },
            BindGroupEntry {
                binding: 3,
                resource: bevy::render::render_resource::BindingResource::Buffer(
                    atlas_buffers.uvs.as_entire_buffer_binding(),
                ),
            },
            BindGroupEntry {
                binding: 4,
                resource: bevy::render::render_resource::BindingResource::Buffer(
                    vertex_buffer.as_entire_buffer_binding(),
                ),
            },
        ],
    );
    bind_group
}
