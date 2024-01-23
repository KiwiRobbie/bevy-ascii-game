use std::slice::SliceIndex;

use bevy::{
    app::{App, PluginGroup, Startup, Update},
    asset::{AssetApp, AssetServer, Assets, Handle},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        system::{Commands, Local, Query, Res, ResMut},
    },
    input::{keyboard::KeyboardInput, ButtonState},
    math::{Vec2, Vec3},
    render::{
        camera::CameraRenderGraph,
        color::Color,
        render_resource::{Extent3d, TextureFormat},
        texture::{Image, ImagePlugin},
    },
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    window::{ReceivedCharacter, Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use swash::scale::{Render, ScaleContext, Source, StrikeWith};

use bevy_ascii_game::{
    atlas::{Atlas, AtlasBuilder},
    font::{CustomFont, CustomFontLoader},
    glyph_gen_pipeline::{FontRenderPlugin, GlyphSprite, GlyphTexture},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::default().with_scale_factor_override(1.0),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(FontRenderPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, font_ready_system)
        .add_systems(Update, keyboard_input)
        .init_asset::<CustomFont>()
        .init_asset::<Atlas>()
        .init_asset_loader::<CustomFontLoader>()
        .run();
}

const CHARSET: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

#[derive(Component)]
struct LoadingCustomFont(Handle<CustomFont>);

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera_render_graph: CameraRenderGraph::new(bevy::core_pipeline::core_2d::graph::NAME),
        ..Default::default()
    });
    commands.spawn(LoadingCustomFont(
        server.load::<CustomFont>("FiraCode-Regular.ttf"),
    ));
}

fn keyboard_input(
    mut ev_character: EventReader<ReceivedCharacter>,
    q_glyph_sprite: Query<&GlyphSprite>,
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    atlases: Res<Assets<Atlas>>,
    fonts: Res<Assets<CustomFont>>,
    mut position: Local<usize>,
) {
    let Some(glyph_sprite) = q_glyph_sprite.get_single().ok() else {
        return;
    };

    let glyph_texture = glyph_textures.get_mut(glyph_sprite.texture.id()).unwrap();
    let atlas = atlases.get(glyph_sprite.atlas.id()).unwrap();
    let font = fonts.get(glyph_sprite.font.id()).unwrap();

    let cusror_glyph_id = font.as_ref().charmap().map('_');
    let cursor_glyph_index = atlas.local_index.get(&cusror_glyph_id).unwrap_or(&u16::MAX);

    for character in ev_character.read() {
        dbg!(character.char);
        if character.char == '\u{8}' {
            glyph_texture.data.split_at_mut(*position).1[..2]
                .copy_from_slice(&u16::MAX.to_le_bytes());
            *position = position
                .wrapping_sub(2)
                .rem_euclid(glyph_texture.data.len());
            glyph_texture.data.split_at_mut(*position).1[..2]
                .copy_from_slice(&cursor_glyph_index.to_le_bytes());
        } else {
            let glyph_id = font.as_ref().charmap().map(character.char);
            let glyph_index = atlas.local_index.get(&glyph_id).unwrap_or(&u16::MAX);
            glyph_texture.data.split_at_mut(*position).1[..2]
                .copy_from_slice(&glyph_index.to_le_bytes());
            *position = (*position + 2).rem_euclid(glyph_texture.data.len());
            glyph_texture.data.split_at_mut(*position).1[..2]
                .copy_from_slice(&cursor_glyph_index.to_le_bytes());
        }
    }
}

fn font_ready_system(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    fonts: ResMut<Assets<CustomFont>>,
    mut atlases: ResMut<Assets<Atlas>>,
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    server: Res<AssetServer>,
    q_fonts_loading: Query<(Entity, &LoadingCustomFont)>,
) {
    use bevy::asset::LoadState;
    for (entity, LoadingCustomFont(font_handle)) in q_fonts_loading.iter() {
        match server.get_load_state(font_handle).unwrap() {
            LoadState::Failed => {
                // one of our assets had an error
            }
            LoadState::Loaded => {
                commands.entity(entity).despawn();

                let font_ref = fonts.get(font_handle).unwrap().as_ref();

                let font_size = 32.0f32;

                let mut context = ScaleContext::new();
                let scaler = context.builder(font_ref).hint(true).size(font_size).build();
                let render = Render::new(&[
                    Source::ColorOutline(0),
                    Source::ColorBitmap(StrikeWith::BestFit),
                    Source::Outline,
                ]);

                let mut atlas_builder = AtlasBuilder::new(font_ref, render, scaler);

                for glyph in CHARSET.chars() {
                    atlas_builder.insert_char(glyph);
                }
                let atlas_handle = atlases.add(atlas_builder.build());
                let atlas = atlases.get(atlas_handle.clone()).unwrap();

                commands.spawn((
                    GlyphSprite {
                        color: Color::WHITE,
                        atlas: atlas_handle,
                        font: font_handle.clone(),
                        texture: glyph_textures.add(GlyphTexture::from_text(
                            (0..32).map(|_| " ".repeat(32)).collect::<Box<[_]>>(),
                            atlas,
                            font_ref,
                        )),
                    },
                    Transform::default(),
                ));

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
            _ => {
                // NotLoaded/Loading: not fully ready yet
            }
        }
    }
}
