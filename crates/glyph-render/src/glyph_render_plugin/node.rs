use bevy::{
    ecs::{
        entity::Entity,
        query::QueryState,
        world::{FromWorld, World},
    },
    render::{
        render_graph,
        render_resource::{BindGroupEntries, PipelineCache},
        view::{ViewTarget, ViewUniforms},
    },
};
use wgpu::{RenderPassColorAttachment, RenderPassDescriptor};

use crate::glyph_buffer::TargetGlyphBuffer;

use super::{
    render_resources::{GlyphBufferData, GlyphUniformBuffer},
    AtlasGpuData, GlyphModelUniformBuffer, GlyphPipelineData, GlyphRenderUniformBuffer,
    GlyphTextureInfo, GpuGlyphTexture,
};

type BufferQuery = (
    Entity,
    &'static GlyphModelUniformBuffer,
    &'static GlyphUniformBuffer,
    &'static GlyphTextureInfo,
    &'static GlyphBufferData,
    &'static AtlasGpuData,
);
type TextureQuery = (
    &'static GlyphRenderUniformBuffer,
    &'static GpuGlyphTexture,
    &'static TargetGlyphBuffer,
);

// type RenderResourceFilter = (Or<(With<GlyphSprite>, With<GlyphAnimation>)>,);

pub struct GlyphGenerationNode {
    q_buffers: QueryState<BufferQuery>,
    q_textures: QueryState<TextureQuery>,
    q_view: QueryState<&'static ViewTarget>,
    buffer_entities: Vec<Entity>,
    texture_entities: Vec<Entity>,
}

impl GlyphGenerationNode {
    pub fn new(world: &mut World) -> Self {
        Self {
            q_view: world.query(),
            q_buffers: world.query(),
            q_textures: world.query(),
            buffer_entities: world
                .query_filtered::<Entity, BufferQuery>()
                .iter(world)
                .collect(),
            texture_entities: world
                .query_filtered::<Entity, TextureQuery>()
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
        self.q_buffers = world.query();
        self.buffer_entities = world
            .query_filtered::<Entity, BufferQuery>()
            .iter(world)
            .collect();

        self.q_textures = world.query();
        self.texture_entities = world
            .query_filtered::<Entity, TextureQuery>()
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

        let Some(render_pipeline) =
            pipeline_cache.get_render_pipeline(glyph_pipeline_data.glyph_render_pipeline_id)
        else {
            dbg!("Early return!");
            return Ok(());
        };

        let Some(raster_pipeline) =
            pipeline_cache.get_render_pipeline(glyph_pipeline_data.glyph_raster_pipeline_id)
        else {
            dbg!("Early return!");
            return Ok(());
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
            buffer_entity,
            glyph_model_uniforms,
            glyph_uniform_buffer,
            glyph_texture_info,
            buffer_data,
            atlas_data,
        ) in self
            .buffer_entities
            .iter()
            .map(|e| self.q_buffers.get_manual(world, *e).unwrap())
        {
            {
                let view = buffer_data.buffer.create_view(&Default::default());
                let glyph_render_render_pass_descriptor = RenderPassDescriptor {
                    label: Some("glyph render pass"),
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                };

                // Render textures to buffers
                for (uniforms, texture, _) in self
                    .texture_entities
                    .iter()
                    .flat_map(|e| self.q_textures.get_manual(world, *e))
                    .filter(|(_, _, target_entity)| ***target_entity == buffer_entity)
                {
                    let render_device = render_context.render_device();
                    let bind_group = render_device.create_bind_group(
                        Some("render bind group".into()),
                        &glyph_pipeline_data.glyph_render_bind_group_layout,
                        &BindGroupEntries::sequential((
                            uniforms.binding().unwrap(),
                            &texture
                                .buffer_texture
                                .create_view(&wgpu::TextureViewDescriptor::default()),
                        )),
                    );
                    let mut render_pass = render_context
                        .command_encoder()
                        .begin_render_pass(&glyph_render_render_pass_descriptor);

                    render_pass.set_bind_group(0, &bind_group, &[]);
                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.draw(0..6, 0..1);
                }
            }

            {
                let render_device = render_context.render_device();
                let bind_group = render_device.create_bind_group(
                    Some("raster bind group".into()),
                    &glyph_pipeline_data.glyph_raster_bind_group_layout,
                    &BindGroupEntries::sequential((
                        glyph_uniform_buffer.binding().unwrap(),
                        world.resource::<ViewUniforms>().uniforms.binding().unwrap(),
                        glyph_model_uniforms.binding().unwrap(),
                        &atlas_data
                            .data
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                        &atlas_data
                            .uvs
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                        &buffer_data
                            .buffer
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                    )),
                );

                let mut render_pass = render_context
                    .command_encoder()
                    .begin_render_pass(&glyph_raster_render_pass_descriptor);

                render_pass.set_bind_group(0, &bind_group, &[]);
                render_pass.set_pipeline(raster_pipeline);

                render_pass.draw(
                    0..6,
                    0..glyph_texture_info.width * glyph_texture_info.height,
                );
            }
        }
        Ok(())
    }
}
