use bevy::{
    app::{App, PluginGroup, Startup},
    asset::Assets,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, ResMut},
    math::{IVec2, UVec2, Vec2, Vec3},
    render::{
        render_resource::{Extent3d, TextureFormat},
        texture::{Image, ImagePlugin},
    },
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::HashMap,
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use swash::{
    scale::{Render, ScaleContext, Scaler, Source, StrikeWith},
    zeno::Format,
};

use bevy_ascii_game::atlas::AtlasBuilder;

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

const CHARSET: &str = "!\\\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());

    let font =
        swash::FontRef::from_index(include_bytes!("../assets/FiraCode-Regular.ttf"), 0).unwrap();

    let font_size = 32.0f32;

    let mut context = ScaleContext::new();
    let scaler = context.builder(font).hint(true).size(font_size).build();
    let render = Render::new(&[
        Source::ColorOutline(0),
        Source::ColorBitmap(StrikeWith::BestFit),
        Source::Outline,
    ]);

    let mut atlas_builder = AtlasBuilder::new(font, render, scaler);

    for glyph in CHARSET.chars() {
        atlas_builder.insert_char(glyph);
    }

    let atlas = atlas_builder.build();

    let image_handle = images.add(Image::new(
        Extent3d {
            width: atlas.size,
            height: atlas.size,
            ..Default::default()
        },
        bevy::render::render_resource::TextureDimension::D2,
        atlas.data.to_vec(),
        TextureFormat::Rgba8Unorm,
    ));

    commands.spawn(SpriteBundle {
        texture: image_handle.clone(),
        transform: Transform::from_translation(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }),
        sprite: Sprite {
            anchor: bevy::sprite::Anchor::BottomLeft,
            custom_size: Some(Vec2 {
                x: atlas.size as f32,
                y: atlas.size as f32,
            }),
            ..Default::default()
        },
        ..Default::default()
    });
}
