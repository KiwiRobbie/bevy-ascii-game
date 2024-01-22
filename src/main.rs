use bevy::{
    app::{App, PluginGroup, Startup},
    asset::Assets,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, ResMut},
    math::{IVec2, UVec2, Vec2, Vec3},
    render::{
        color::Color,
        render_resource::{Extent3d, TextureFormat},
        texture::{BevyDefault, Image, ImagePlugin},
    },
    sprite::{Sprite, SpriteBundle},
    text::Text,
    transform::components::Transform,
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use swash::{
    scale::{Render, ScaleContext, Scaler, Source, StrikeWith},
    zeno::{Format, Placement},
    GlyphMetrics,
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(
                    // This sets image filtering to nearest
                    // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
                    // by linear filtering.
                    ImagePlugin::default_nearest(),
                )
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::default().with_scale_factor_override(1.0),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_systems(Startup, setup)
        .run();
}

struct AtlasRenderedGlyph {
    ofset: IVec2,
    size: UVec2,
    texture: Vec<u8>,
}

struct AtlasItem {
    start: UVec2,
    size: UVec2,
    offset: IVec2,
}
struct AtlasData {
    data: Vec<u8>,
    size: u32,
    items: Vec<AtlasItem>,
}

struct AtlasBuilder<'a> {
    font: swash::FontRef<'a>,
    render: Render<'a>,
    scaler: Scaler<'a>,
    rendered: Vec<AtlasRenderedGlyph>,
    packed_positions: Vec<UVec2>,
    size: u32,
}

impl<'a> AtlasBuilder<'a> {
    fn new(font: swash::FontRef<'a>, render: Render<'a>, scaler: Scaler<'a>) -> Self {
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

    fn render_char(&mut self, glyph: char) -> Option<()> {
        let mut pen_x = 0.0f32;

        let glyph_id = self.font.charmap().map(glyph);

        let image = self
            .render
            .format(Format::Subpixel)
            .render(&mut self.scaler, glyph_id)?;
        self.rendered.push(AtlasRenderedGlyph {
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
        self.rendered
            .iter()
            .enumerate()
            .collect::<Vec<_>>()
            .sort_by(|(_, a), (_, b)| a.size.y.cmp(&b.size.y));

        self.size = 256u32;

        self.packed_positions = vec![UVec2::ZERO; self.rendered.len()];
        'retry: loop {
            let mut origin = UVec2::ZERO;

            for (index, image) in self.rendered.iter().enumerate() {
                origin.x += image.size.x;
                if origin.x >= self.size {
                    origin.x = 0;
                    origin.y += image.size.y;
                    if origin.y >= self.size {
                        self.size *= 2;
                        continue 'retry;
                    }
                }
                self.packed_positions[index] = origin;
            }
        }
    }
    fn pack_glyphs(&self) -> AtlasData {
        self.create_packing();

        let mut data: Vec<u8> = vec![0; (self.size * self.size * 4) as usize];
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

                    data[destination..destination + 4]
                        .copy_from_slice(&rendered.texture[source..source + c]);
                }
            }
        }

        AtlasData {
            size: self.size,
            data,
            items,
        }
    }
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());

    let font =
        swash::FontRef::from_index(include_bytes!("../assets/FiraCode-Regular.ttf"), 0).unwrap();

    let font_size = 48.0f32;

    let mut context = ScaleContext::new();
    let mut scaler = context.builder(font).hint(true).size(font_size).build();

    let metrics = font.metrics(&[]);
    dbg!(metrics);

    let mut pen_x = 0.0f32;

    for (index, glpyh) in "ABC 123 =#@$<>=".chars().enumerate() {
        let glyph_id = font.charmap().map(glpyh);

        let advance_x = font
            .glyph_metrics(&[])
            .scale(font_size)
            .advance_width(glyph_id);

        pen_x += advance_x;

        let outline = Render::new(&[
            Source::ColorOutline(0),
            Source::ColorBitmap(StrikeWith::BestFit),
            Source::Outline,
        ])
        .format(Format::Subpixel)
        .render(&mut scaler, glyph_id);

        match outline {
            Some(image) => {
                if image.placement.width <= 0 || image.placement.height <= 0 {
                    continue;
                }

                let mut data = image.data;
                for byte in data.iter_mut().skip(3).step_by(4) {
                    *byte = 255;
                }

                let image_handle = images.add(Image::new(
                    Extent3d {
                        width: image.placement.width,
                        height: image.placement.height,
                        ..Default::default()
                    },
                    bevy::render::render_resource::TextureDimension::D2,
                    data,
                    TextureFormat::Rgba8Unorm,
                ));

                commands.spawn(SpriteBundle {
                    texture: image_handle.clone(),
                    transform: Transform::from_translation(Vec3 {
                        x: pen_x + image.placement.left as f32,
                        y: -image.placement.top as f32,
                        z: 0.0,
                    }),
                    sprite: Sprite {
                        anchor: bevy::sprite::Anchor::BottomLeft,
                        custom_size: Some(Vec2 {
                            x: image.placement.width as f32,
                            y: image.placement.height as f32,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
            None => {}
        };
    }
}
