use bevy::{
    math::{IVec2, UVec2},
    utils::{HashMap, HashSet},
};
use swash::{
    scale::{Render, Scaler},
    GlyphId,
};

use super::{AtlasItem, FontAtlasSource};

struct RenderedGlyph {
    glyph_id: u16,
    offset: IVec2,
    size: UVec2,
    texture: Vec<u8>,
}

pub struct AtlasBuilder<'a> {
    font: swash::FontRef<'a>,
    render: Render<'a>,
    scaler: Scaler<'a>,
    rendered: Vec<RenderedGlyph>,
    packed_positions: Vec<UVec2>,
    characters: HashSet<char>,
    size: u32,
    font_size: f32,
}

impl<'a> AtlasBuilder<'a> {
    pub fn new(
        font: swash::FontRef<'a>,
        render: Render<'a>,
        scaler: Scaler<'a>,
        font_size: f32,
    ) -> Self {
        // let metrics = font.metrics(&[]);

        Self {
            font,
            render,
            scaler,
            rendered: vec![],
            packed_positions: vec![],
            characters: HashSet::new(),
            size: 0,
            font_size,
        }
    }

    pub fn insert_char(&mut self, character: char) -> Option<()> {
        self.characters.insert(character);
        let glyph_id = self.font.charmap().map(character);
        self.insert_glyph(glyph_id)
    }

    fn insert_glyph(&mut self, glyph_id: GlyphId) -> Option<()> {
        let mut image = self.render.render(&mut self.scaler, glyph_id)?;

        for alpha in image.data.iter_mut().skip(3).step_by(4) {
            *alpha = 0xff;
        }

        let metrics = self.font.metrics(&[]).scale(self.font_size);

        self.rendered.push(RenderedGlyph {
            glyph_id,
            offset: IVec2 {
                x: image.placement.left,
                y: image.placement.top + metrics.descent as i32,
            },
            size: UVec2 {
                x: image.placement.width,
                y: image.placement.height,
            },
            texture: image.data,
        });

        Some(())
    }

    fn create_packing(&mut self) {
        self.rendered.sort_by(|a, b| b.size.y.cmp(&a.size.y));

        self.size = 256u32;
        self.packed_positions = vec![UVec2::ZERO; self.rendered.len()];
        'retry: loop {
            let mut origin = UVec2::ZERO;
            let mut row_height: u32 = self.rendered.first().map(|x| x.size.y).unwrap_or(0);

            for (index, image) in self.rendered.iter().enumerate() {
                let mut placement = origin;
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

    pub fn build(&mut self) -> FontAtlasSource {
        const CHANNELS: usize = 4;
        self.create_packing();

        let mut data: Vec<u8> = [0xff, 0x00, 0x00, 0x00]
            .into_iter()
            .cycle()
            .take((CHANNELS as u32 * self.size * self.size) as usize)
            .collect::<Vec<_>>();

        let mut items: Vec<AtlasItem> = Vec::with_capacity(self.rendered.len());
        for (rendered, packed_position) in self.rendered.iter().zip(self.packed_positions.iter()) {
            items.push(AtlasItem {
                start: *packed_position,
                size: rendered.size,
                offset: rendered.offset,
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

        FontAtlasSource {
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
            charset: self.characters.clone(),
        }
    }
}
