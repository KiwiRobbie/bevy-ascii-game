use bevy::{
    asset::Assets,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, ResMut},
    math::{IVec2, UVec2, Vec2, Vec3},
    render::{
        render_resource::{Extent3d, TextureFormat},
        texture::Image,
    },
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::HashMap,
};
use swash::{
    scale::{Render, ScaleContext, Scaler, Source, StrikeWith},
    zeno::Format,
    GlyphId,
};

use super::{Atlas, AtlasItem};

struct RenderedGlyph {
    glyph_id: u16,
    ofset: IVec2,
    size: UVec2,
    texture: Vec<u8>,
}

pub struct AtlasBuilder<'a> {
    font: swash::FontRef<'a>,
    render: Render<'a>,
    scaler: Scaler<'a>,
    rendered: Vec<RenderedGlyph>,
    packed_positions: Vec<UVec2>,
    size: u32,
}

impl<'a> AtlasBuilder<'a> {
    pub fn new(font: swash::FontRef<'a>, render: Render<'a>, scaler: Scaler<'a>) -> Self {
        let metrics = font.metrics(&[]);
        dbg!(metrics);

        Self {
            font,
            render,
            scaler,
            rendered: vec![],
            packed_positions: vec![],
            size: 0,
        }
    }

    pub fn insert_char(&mut self, glyph: char) -> Option<()> {
        let glyph_id = self.font.charmap().map(glyph);
        self.insert_glyph(glyph_id)
    }

    pub fn insert_glyph(&mut self, glyph_id: GlyphId) -> Option<()> {
        let mut image = self
            .render
            .format(Format::Subpixel)
            .render(&mut self.scaler, glyph_id)?;

        for alpha in image.data.iter_mut().skip(3).step_by(4) {
            *alpha = 0xff;
        }

        self.rendered.push(RenderedGlyph {
            glyph_id,
            ofset: IVec2 {
                x: image.placement.top,
                y: image.placement.left,
            },
            size: UVec2 {
                x: image.placement.width,
                y: image.placement.height,
            },
            texture: image.data,
        });

        return Some(());
    }

    fn create_packing(&mut self) {
        self.rendered.sort_by(|a, b| b.size.y.cmp(&a.size.y));

        self.size = 256u32;
        self.packed_positions = vec![UVec2::ZERO; self.rendered.len()];
        'retry: loop {
            let mut origin = UVec2::ZERO;
            let mut row_height: u32 = self
                .rendered
                .first()
                .and_then(|x| Some(x.size.y))
                .unwrap_or(0);

            for (index, image) in self.rendered.iter().enumerate() {
                let mut placement = origin.clone();
                if origin.x + image.size.x <= self.size {
                    origin.x += image.size.x;
                } else {
                    if image.size.x > self.size {
                        self.size *= 2;
                        continue 'retry;
                    }

                    if origin.y + row_height + image.size.y > self.size {
                        self.size *= 2;
                        continue 'retry;
                    }
                    placement.x = 0;
                    placement.y += row_height;

                    origin.x = image.size.x;
                    origin.y += row_height;

                    row_height = image.size.y;
                }
                self.packed_positions[index] = placement;
            }
            break;
        }
    }

    pub fn build(&mut self) -> Atlas {
        const CHANNELS: usize = 4;
        self.create_packing();

        let mut data: Vec<u8> = [0xff, 0x00, 0x00, 0xff]
            .into_iter()
            .cycle()
            .take((self.size * self.size * 4) as usize)
            .collect::<Vec<_>>();

        let mut items: Vec<AtlasItem> = Vec::with_capacity(self.rendered.len());
        for (rendered, packed_position) in self.rendered.iter().zip(self.packed_positions.iter()) {
            items.push(AtlasItem {
                start: *packed_position,
                size: rendered.size,
                offset: rendered.ofset,
            });

            for y in 0..rendered.size.y {
                for x in 0..rendered.size.x {
                    let source = 4usize * (x + rendered.size.x * y) as usize;
                    let destination = 4usize
                        * (x + packed_position.x + (y + packed_position.y) * self.size) as usize;

                    data[destination..destination + CHANNELS]
                        .copy_from_slice(&rendered.texture[source..source + CHANNELS]);
                }
            }
        }

        Atlas {
            size: self.size,
            data: data.into(),
            items: items.into(),
            glyph_ids: self.rendered.iter().map(|image| image.glyph_id).collect(),
            local_index: HashMap::from_iter(
                self.rendered
                    .iter()
                    .enumerate()
                    .map(|(a, b)| (b.glyph_id, a as u16)),
            ),
        }
    }
}

const CHARSET: &str = "!\\\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
