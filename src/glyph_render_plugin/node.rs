use bevy::{
    ecs::{
        entity::Entity,
        query::{Or, QueryState, With},
        world::{FromWorld, World},
    },
    render::{
        render_graph,
        render_resource::{ComputePassDescriptor, PipelineCache},
        view::ViewTarget,
    },
};
use wgpu::{RenderPassColorAttachment, RenderPassDescriptor};

use crate::glyph_animation::GlyphAnimation;

use super::{
    generation_descriptors::{self},
    raster_descriptors,
    render_resources::{GlyphStorageTexture, GlyphUniformBuffer, GlyphVertexBuffer},
    AtlasGpuBuffers, GlyphModelUniformBuffer, GlyphPipelineData, GlyphSprite, GlyphTextureInfo,
};

type RenderResourceQuery = (
    &'static GlyphModelUniformBuffer,
    &'static GlyphUniformBuffer,
    &'static GlyphTextureInfo,
    &'static GlyphStorageTexture,
    &'static AtlasGpuBuffers,
    &'static GlyphVertexBuffer,
);

type RenderResourceFilter = (Or<(With<GlyphSprite>, With<GlyphAnimation>)>,);

pub struct GlyphGenerationNode {
    query: QueryState<RenderResourceQuery, RenderResourceFilter>,
    q_view: QueryState<&'static ViewTarget>,
    entities: Vec<Entity>,
}

impl GlyphGenerationNode {
    pub fn new(world: &mut World) -> Self {
        Self {
            q_view: world.query(),
            query: world.query_filtered(),
            entities: world
                .query_filtered::<Entity, RenderResourceQuery>()
                .iter(world)
                .collect(),
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
    }
    fn update(&mut self, world: &mut World) {
        self.q_view = world.query();
        self.query = world.query_filtered();
        self.entities = world
            .query_filtered::<Entity, RenderResourceQuery>()
            .iter(world)
            .collect();
    }

    fn run(
        &self,
        graph: &mut render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let view_entity = graph.get_view_entity().expect("Missing View Entity");
        let target = self
            .q_view
            .get_manual(world, view_entity)
            .expect("Missing ViewTarget");

        let pipeline_cache = world.resource::<PipelineCache>();
        let glyph_pipeline_data = world.get_resource::<GlyphPipelineData>().unwrap();

        let (generation_pipeline, raster_pipeline) = (
            pipeline_cache
                .get_compute_pipeline(glyph_pipeline_data.glyph_generation_pipeline_id)
                .unwrap(),
            pipeline_cache
                .get_render_pipeline(glyph_pipeline_data.glyph_raster_pipeline_id)
                .unwrap(),
        );

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
            glyph_model_uniforms,
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
            {
                let render_device = render_context.render_device();
                let glyph_generation_bind_group = generation_descriptors::create_bind_group(
                    render_device,
                    glyph_pipeline_data,
                    glyph_uniform_buffer,
                    glyph_storage_texture,
                    atlas_buffers,
                    vertex_buffer,
                );
                let mut glyph_generation_compute_pass = render_context
                    .command_encoder()
                    .begin_compute_pass(&glyph_generation_compute_pass_descriptor);

                glyph_generation_compute_pass.set_bind_group(0, &glyph_generation_bind_group, &[]);
                glyph_generation_compute_pass.set_pipeline(generation_pipeline);

                glyph_generation_compute_pass.dispatch_workgroups(
                    glyph_texture_info.width,
                    glyph_texture_info.height,
                    1,
                );
            }

            {
                let render_device = render_context.render_device();
                let bind_group = raster_descriptors::create_bind_group(
                    render_device,
                    glyph_pipeline_data,
                    world,
                    glyph_model_uniforms,
                    atlas_buffers,
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
        }
        Ok(())
    }
}
