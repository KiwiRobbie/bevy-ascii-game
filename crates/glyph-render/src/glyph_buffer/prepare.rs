// Extract from textures

use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res},
    },
    math::{IVec2, UVec2},
    render::renderer::{RenderDevice, RenderQueue},
};
use spatial_grid::position::Position;
use wgpu::{
    util::BufferInitDescriptor, BufferDescriptor, BufferUsages, Extent3d, TextureDescriptor,
    TextureDimension, TextureFormat, TextureUsages,
};

use crate::glyph_render_plugin::{ExtractedGlyphTexture, GpuGlyphTexture};

use super::GlyphBuffer;
pub fn prepare_glyph_buffers(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    q_glyph_buffer: Query<(Entity, &GlyphBuffer)>,

    q_textures: Query<(&Position, &ExtractedGlyphTexture)>,
) {
    for (buffer_entity, buffer) in q_glyph_buffer.iter() {
        let buffer_width = buffer.size.x as usize;
        let buffer_height = buffer.size.y as usize;
        let mut buffer_data: Box<[u8]> = vec![u8::MAX; 4 * 4 * buffer_height * buffer_width].into();

        for (position, texture) in buffer.textures.iter().flat_map(|t| q_textures.get(*t)) {
            let source_size = UVec2::new(texture.width as u32, texture.height as u32);

            let buffer_start = IVec2::ZERO;
            let buffer_end = buffer.size.as_ivec2();

            let dst_min = position.position.clamp(buffer_start, buffer_end);
            let dst_max =
                (position.position + source_size.as_ivec2()).clamp(buffer_start, buffer_end);

            let size = (dst_max - dst_min).as_uvec2();

            if UVec2::ZERO.cmpeq(size).any() {
                continue;
            }

            let src_min = (dst_min - position.position).as_uvec2();
            let dst_min = dst_min.as_uvec2();

            for dy in 0..size.y as usize {
                let src_y = src_min.y as usize + dy;
                let src_start_x = src_min.x as usize;
                let src_start = src_y * source_size.x as usize + src_start_x;

                let dst_y = dst_min.y as usize + dy;
                let dst_start_x = dst_min.x as usize;
                let dst_start = dst_y * buffer_width + dst_start_x;

                for dx in 0..size.x as usize {
                    let src_index = src_start + dx;
                    let dst_index = dst_start + dx;
                    let data: [u8; 2] =
                        [texture.data[2 * src_index], texture.data[2 * src_index + 1]];

                    if data != [u8::MAX, u8::MAX] {
                        buffer_data[4 * 4 * dst_index] = data[0];
                        buffer_data[4 * 4 * dst_index + 1] = data[1];
                        buffer_data[4 * 4 * dst_index + 2] = 0;
                        buffer_data[4 * 4 * dst_index + 3] = 0;
                    }
                }
            }
        }

        let vertex_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("glyph vertex buffer"),
            usage: BufferUsages::STORAGE
                | BufferUsages::COPY_SRC
                | BufferUsages::COPY_DST
                | BufferUsages::VERTEX,
            contents: &buffer_data,
        });

        commands.entity(buffer_entity).insert(GpuGlyphTexture {
            vertex_buffer,
            width: buffer_width as u32,
            height: buffer_height as u32,
        });
    }
}
