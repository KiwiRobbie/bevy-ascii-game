use crate::atlas::AtlasGpuBuffers;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::QueryIter;
use bevy::ecs::query::QueryState;
use bevy::ecs::world::World;
use bevy::render::render_graph;
use bevy::render::render_resource::BindGroupEntry;
use bevy::render::render_resource::ComputePassDescriptor;
use bevy::render::render_resource::LoadOp;
use bevy::render::render_resource::Operations;
use bevy::render::render_resource::PipelineCache;
use bevy::render::render_resource::RenderPassColorAttachment;

use super::GlyphGenerationPipelineData;
use super::GlyphSprite;
use super::GlyphStorageTexture;
use super::GlyphTextureInfo;
use super::GlyphVertexBuffer;
use super::GylphGenerationUniformBuffer;

type Q = (
    &'static GlyphSprite,
    &'static GylphGenerationUniformBuffer,
    &'static GlyphTextureInfo,
    &'static GlyphStorageTexture,
    &'static AtlasGpuBuffers,
    &'static GlyphVertexBuffer,
);
pub struct GlyphGenerationNode {
    query: QueryState<Q>,
    entities: Vec<Entity>,
}

impl GlyphGenerationNode {
    pub fn new(world: &mut World) -> Self {
        Self {
            query: world.query(),
            entities: world.query_filtered::<Entity, Q>().iter(world).collect(),
        }
    }
}

impl render_graph::Node for GlyphGenerationNode {
    fn input(&self) -> Vec<render_graph::SlotInfo> {
        vec![]
    }
    fn output(&self) -> Vec<render_graph::SlotInfo> {
        vec![
        //     render_graph::SlotInfo::new(
        //     "vertex_buffer",
        //     render_graph::SlotType::Buffer,
        // )
        ]
    }
    fn update(&mut self, world: &mut World) {
        self.query = world.query();
        self.entities = world.query_filtered::<Entity, Q>().iter(world).collect();
    }

    fn run(
        &self,
        graph: &mut render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        // let view_entity = graph.get_input_entity("view")?;
        let pipeline_cache = world.resource::<PipelineCache>();
        let generation_pipeline_data = world.get_resource::<GlyphGenerationPipelineData>().unwrap();

        let generation_pipeline =
            match pipeline_cache.get_compute_pipeline(generation_pipeline_data.pipeline_id) {
                Some(pipeline) => pipeline,
                None => return Ok(()),
            };

        let render_pass_descriptor = ComputePassDescriptor {
            label: Some("glyph generation pass"),
        };

        for (
            glyph_sprite,
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
            let bind_group = render_device.create_bind_group(
                Some("glyph generation bind group"),
                &generation_pipeline_data.bind_group_layout,
                &[
                    BindGroupEntry {
                        binding: 0,
                        resource: glyph_uniform_buffer.binding().unwrap(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: bevy::render::render_resource::BindingResource::TextureView(
                            &glyph_storage_texture
                                .create_view(&wgpu::TextureViewDescriptor::default()),
                        ),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: bevy::render::render_resource::BindingResource::TextureView(
                            &atlas_buffers
                                .data
                                .create_view(&wgpu::TextureViewDescriptor {
                                    label: Some("Atlas storage texture view"),
                                    format: Some(wgpu::TextureFormat::R16Uint),
                                    dimension: Some(wgpu::TextureViewDimension::D2),
                                    aspect: wgpu::TextureAspect::All,
                                    base_mip_level: 0,
                                    mip_level_count: None,
                                    base_array_layer: 0,
                                    array_layer_count: None,
                                }),
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
            let mut compute_pass = render_context
                .command_encoder()
                .begin_compute_pass(&render_pass_descriptor);

            // render_pass.set_bind_group(0, &voxel_data.bind_group, &[]);
            compute_pass.set_bind_group(0, &bind_group, &[]);

            compute_pass.set_pipeline(generation_pipeline);
            compute_pass.dispatch_workgroups(
                glyph_texture_info.width,
                glyph_texture_info.height,
                1,
            );
            vertex_buffer.unmap();
            // atlas_buffers.uvs.unmap();
        }
        Ok(())
    }
}
