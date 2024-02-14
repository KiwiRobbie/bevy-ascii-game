// Extract from textures

use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res},
    },
    math::{IVec2, UVec2, Vec2},
    render::{
        color::Color,
        renderer::{RenderDevice, RenderQueue},
    },
};
use bytemuck::{cast_slice, Zeroable};
use spatial_grid::position::Position;
use wgpu::{util::BufferInitDescriptor, BufferUsages, Extent3d};

use crate::glyph_render_plugin::{
    ExtractedAtlas, ExtractedGlyphTexture, GlyphSolidColor, GpuGlyphItem, GpuGlyphTexture,
};

use super::{GlyphBuffer, TargetGlyphBuffer};
pub fn prepare_glyph_buffers(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    q_glyph_buffer: Query<(Entity, &GlyphBuffer, &ExtractedAtlas)>,
    q_textures: Query<(
        &TargetGlyphBuffer,
        &Position,
        &ExtractedGlyphTexture,
        Option<&GlyphSolidColor>,
    )>,
) {
    for (buffer_entity, buffer, atlas) in q_glyph_buffer.iter() {
        let atlas_uvs = &atlas.items;

        let mut buffer_data: Box<[u16]> =
            vec![u16::MAX; buffer.size.x as usize * buffer.size.y as usize].into();

        for (_, position, texture, solid_color) in q_textures
            .iter()
            .filter(|(target, _, _, _)| target.0 == buffer_entity)
        {
            let source_size = UVec2::new(texture.width as u32, texture.height as u32);

            let buffer_start = IVec2::ZERO;
            let buffer_end = buffer.size.as_ivec2();

            let dst_min = position.clamp(buffer_start, buffer_end);
            let dst_max = (**position + source_size.as_ivec2()).clamp(buffer_start, buffer_end);

            let size = (dst_max - dst_min).as_uvec2();

            if UVec2::ZERO.cmpeq(size).any() {
                continue;
            }

            let src_min = (dst_min - **position).as_uvec2();
            let dst_min = dst_min.as_uvec2();

            for dy in 0..size.y as usize {
                let src_y = src_min.y as usize + dy;
                let src_start_x = src_min.x as usize;
                let src_start = src_y * source_size.x as usize + src_start_x;

                let dst_y = dst_min.y as usize + dy;
                let dst_start_x = dst_min.x as usize;
                let dst_start = dst_y * buffer.size.x as usize + dst_start_x;

                for dx in 0..size.x as usize {
                    let src_index = src_start + dx;
                    let dst_index = dst_start + dx;
                    let glyph: u16 = texture.data[src_index];

                    if glyph != u16::MAX {
                        let uv = &atlas_uvs[glyph as usize];
                        buffer_data[dst_index] = glyph;
                    }
                }
            }
        }

        let buffer_texture = render_device.create_texture_with_data(
            &render_queue,
            &wgpu::TextureDescriptor {
                label: Some("glyph buffer data"),
                size: Extent3d {
                    depth_or_array_layers: 1,
                    width: buffer.size.x,
                    height: buffer.size.y,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R16Uint,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            cast_slice(&buffer_data),
        );

        // let vertex_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        //     label: Some("glyph vertex buffer"),
        //     usage: BufferUsages::STORAGE
        //         | BufferUsages::COPY_SRC
        //         | BufferUsages::COPY_DST
        //         | BufferUsages::VERTEX,
        //     contents: cast_slice(&buffer_data),
        // });

        commands.entity(buffer_entity).insert(GpuGlyphTexture {
            buffer_texture,
            width: buffer.size.x,
            height: buffer.size.y,
        });
    }
}
