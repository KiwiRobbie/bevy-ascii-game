use crate::{atlas::AtlasGpuBuffers, glyph_gen_pipeline::GlyphTextureInfo};
use bevy::{
    ecs::{
        entity::Entity,
        query::QueryState,
        world::{FromWorld, World},
    },
    render::{
        render_graph,
        render_resource::{BindGroupEntry, PipelineCache},
        view::{ViewTarget, ViewUniforms},
    },
};
use wgpu::{RenderPassColorAttachment, RenderPassDescriptor};

use super::GlyphRasterPipelineData;

pub struct GlyphRasterNode {
    q_view: QueryState<&'static ViewTarget>,
    q_sprite: QueryState<(&'static AtlasGpuBuffers, &'static GlyphTextureInfo)>,
    atlas_entity: Option<Entity>,
}

impl GlyphRasterNode {
    pub fn new(world: &mut World) -> Self {
        Self {
            q_view: world.query(),
            q_sprite: world.query(),
            atlas_entity: world
                .query_filtered::<Entity, &AtlasGpuBuffers>()
                .get_single(world)
                .ok(),
        }
    }
}

impl FromWorld for GlyphRasterNode {
    fn from_world(world: &mut World) -> Self {
        Self::new(world)
    }
}

impl render_graph::Node for GlyphRasterNode {
    fn input(&self) -> Vec<render_graph::SlotInfo> {
        vec![render_graph::SlotInfo::new(
            "vertex_buffer",
            render_graph::SlotType::Buffer,
        )]
    }
    fn output(&self) -> Vec<render_graph::SlotInfo> {
        vec![]
    }
    fn update(&mut self, world: &mut World) {
        self.q_view = world.query();
        self.q_sprite = world.query();
        self.atlas_entity = world
            .query_filtered::<Entity, &AtlasGpuBuffers>()
            .get_single(world)
            .ok();
    }

    fn run(
        &self,
        graph: &mut render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let vertex_buffer = graph.get_input_buffer("vertex_buffer").unwrap();
        let Some(view_entity) = graph.get_view_entity() else {
            return Ok(());
        };

        let Some(atlas_entity) = self.atlas_entity else {
            return Ok(());
        };

        let Some((atlas, glyph_texture_info)) = self.q_sprite.get_manual(world, atlas_entity).ok()
        else {
            return Ok(());
        };

        let pipeline_cache = world.resource::<PipelineCache>();
        let generation_pipeline_data = world.get_resource::<GlyphRasterPipelineData>().unwrap();

        let target = match self.q_view.get_manual(world, view_entity) {
            Ok(result) => result,
            Err(_) => panic!("Camera missing component!"),
        };

        let raster_pipeline =
            match pipeline_cache.get_render_pipeline(generation_pipeline_data.pipeline_id) {
                Some(pipeline) => pipeline,
                None => return Ok(()),
            };

        let render_pass_descriptor = RenderPassDescriptor {
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

        // for () in [()] {
        let render_device = render_context.render_device();
        let bind_group = render_device.create_bind_group(
            Some("glyph generation bind group"),
            &generation_pipeline_data.bind_group_layout,
            &[
                BindGroupEntry {
                    binding: 0,
                    resource: world.resource::<ViewUniforms>().uniforms.binding().unwrap(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: bevy::render::render_resource::BindingResource::TextureView(
                        &atlas
                            .data
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                    ),
                },
            ],
        );
        let mut render_pass = render_context
            .command_encoder()
            .begin_render_pass(&render_pass_descriptor);

        render_pass.set_bind_group(0, &bind_group, &[]);

        render_pass.set_pipeline(raster_pipeline);

        render_pass.set_vertex_buffer(0, *vertex_buffer.slice(..));
        render_pass.draw(
            0..6 * glyph_texture_info.width * glyph_texture_info.height,
            0..1,
        );
        // }
        Ok(())
    }
}
