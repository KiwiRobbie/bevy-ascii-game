#![feature(future_join)]

use std::time::Duration;

use bevy::{
    app::{App, PluginGroup, Startup, Update},
    asset::{AssetApp, AssetServer, Assets},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::With,
        system::{Commands, Local, Query, Res, ResMut},
    },
    math::Vec3,
    render::{
        camera::CameraRenderGraph,
        color::Color,
        texture::{self, Image, ImagePlugin},
    },
    sprite::Sprite,
    time::Time,
    transform::components::{GlobalTransform, Transform},
    window::{ReceivedCharacter, Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use bevy_prng::ChaCha8Rng;
use bevy_rand::{plugin::EntropyPlugin, resource::GlobalEntropy};
use rand_core::RngCore;
use swash::{
    scale::{Render, ScaleContext, Source, StrikeWith},
    zeno::Format,
};

use bevy_ascii_game::{
    atlas::{AtlasBuilder, CharacterSet, FontAtlasPlugin, FontAtlasSource, FontAtlasUser},
    font::{font_load_system, CustomFont, CustomFontLoader, CustomFontSource, FontSize},
    glyph_animation::{GlyphAnimation, GlyphAnimationAssetLoader, GlyphAnimationSource},
    glyph_render_plugin::{GlyphRenderPlugin, GlyphSprite, GlyphTexture},
};

fn main() {
    let mut app = App::new();
    app.add_plugins(
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
    .add_plugins(FontAtlasPlugin)
    .init_asset::<GlyphAnimationSource>()
    .init_asset_loader::<GlyphAnimationAssetLoader>()
    .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
    .add_plugins(GlyphRenderPlugin)
    .add_systems(Startup, setup_system)
    .add_systems(
        Update,
        (
            keyboard_input_system,
            glitch_system,
            font_load_system,
            looping_animation_player_system,
        ),
    )
    .init_asset::<CustomFontSource>()
    .init_asset_loader::<CustomFontLoader>();

    #[cfg(debug_assertions)]
    std::fs::write(
        "render-graph.dot",
        bevy_mod_debugdump::render_graph_dot(&app, &Default::default()),
    )
    .unwrap();

    app.run();
}

const CHARSET: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

fn setup_system(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
) {
    commands.spawn((
        Transform::default(),
        GlobalTransform::default(),
        GlyphAnimation {
            source: server.load("anim/player/player_running.anim.ron"),
            frame: 0,
        },
        FontAtlasUser,
        CustomFont(server.load("FiraCode-Regular.ttf")),
        CharacterSet(CHARSET.chars().into_iter().collect()),
        FontSize(32),
        LoopingAnimationPlayer::new(15),
    ));

    commands.spawn((
        CustomFont(server.load("FiraCode-Regular.ttf")),
        FontAtlasUser,
        FontSize(32),
        GlyphSprite {
            color: Color::WHITE,
            texture: glyph_textures.add(GlyphTexture {
                data: (0..16).map(|_| " ".repeat(32)).collect::<Vec<String>>(),
            }),
        },
        Transform::from_translation(Vec3 {
            x: FONT_ADVANCE * 32.0 * -0.5,
            y: FONT_LEAD * 16.0 * -0.5,
            z: 0.0,
        }),
        GlobalTransform::default(),
        KeyboardInputMarker,
    ));

    commands.spawn(Camera2dBundle {
        camera_render_graph: CameraRenderGraph::new(bevy::core_pipeline::core_2d::graph::NAME),
        ..Default::default()
    });
}

#[derive(Component)]
struct KeyboardInputMarker;

#[derive(Component)]
struct GlitchMarker;

const FONT_SIZE: f32 = 32.0f32;
const FONT_ADVANCE: f32 = 19.0f32;
const FONT_LEAD: f32 = 32.0f32;

fn keyboard_input_system(
    mut ev_character: EventReader<ReceivedCharacter>,
    q_glyph_sprite: Query<&GlyphSprite, &KeyboardInputMarker>,
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    // atlases: Res<Assets<FontAtlasSource>>,
    fonts: Res<Assets<CustomFontSource>>,
    mut position: Local<usize>,
) {
    let Some(glyph_sprite) = q_glyph_sprite.get_single().ok() else {
        return;
    };

    let glyph_texture = glyph_textures.get_mut(glyph_sprite.texture.id()).unwrap();
    let width = glyph_texture.data.first().unwrap().len();
    let height = glyph_texture.data.len();

    fn get_pos(index: usize, width: usize, height: usize) -> (usize, usize) {
        return (
            index.rem_euclid(width),
            index.div_euclid(width).rem_euclid(height),
        );
    }

    for character in ev_character.read() {
        dbg!(character.char);
        if character.char == '\u{8}' {
            *position = (*position + width * height - 1).rem_euclid(width * height);
            let (x, y) = get_pos(*position, width, height);
            glyph_texture.data[y].replace_range(x..=x, "_");
            let (x, y) = get_pos(*position + 1, width, height);
            glyph_texture.data[y].replace_range(x..=x, " ");
        } else {
            let (x, y) = get_pos(*position, width, height);
            glyph_texture.data[y].replace_range(x..=x, character.char.to_string().as_str());
            let (x, y) = get_pos(*position + 1, width, height);
            glyph_texture.data[y].replace_range(x..=x, "_");

            *position = (*position + 1).rem_euclid(width * height);
        }
    }
}

fn glitch_system(
    mut q_glyph_sprite: Query<(&GlyphSprite, &mut Transform), &GlitchMarker>,
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    // atlases: Res<Assets<FontAtlasSource>>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    time: Res<Time>,
) {
    // for (glyph_sprite, mut transform) in q_glyph_sprite.iter_mut() {
    //     *transform = transform.with_translation(Vec3 {
    //         x: FONT_ADVANCE * 32.0 * 0.0 + 5.0 * FONT_SIZE * f32::cos(time.elapsed_seconds()),
    //         y: FONT_LEAD * 16.0 * -0.5 + 5.0 * FONT_SIZE * f32::sin(time.elapsed_seconds()),
    //         z: 0.0,
    //     });

    //     let glyph_texture = glyph_textures.get_mut(glyph_sprite.texture.id()).unwrap();
    //     let atlas = atlases.get(glyph_sprite.atlas.id()).unwrap();

    //     let glitch_position = rng.next_u32().rem_euclid(glyph_texture.width) as usize;
    //     let glitch_value = rng.next_u32().rem_euclid(atlas.glyph_ids.len() as u32) as u16;

    //     glyph_texture.data.split_at_mut(glitch_position * 2).1[..2]
    //         .copy_from_slice(&glitch_value.to_le_bytes());

    //     let src_end = ((glyph_texture.height - 1) * glyph_texture.width * 2) as usize;
    //     let dst_start = (glyph_texture.width * 2) as usize;
    //     glyph_texture.data.copy_within(..src_end, dst_start);

    //     for start_item in glyph_texture
    //         .data
    //         .iter_mut()
    //         .step_by(2)
    //         .take(glyph_texture.width as usize)
    //     {
    //         *start_item = start_item.saturating_add(1);
    //     }
    // }
}

#[derive(Component)]
pub struct LoopingAnimationPlayer {
    pub frame_rate: u32,
    pub start_time: Option<f64>,
}
impl LoopingAnimationPlayer {
    fn new(frame_rate: u32) -> Self {
        Self {
            frame_rate: frame_rate,
            start_time: None,
        }
    }
}

fn looping_animation_player_system(
    mut q_glyph_animation: Query<(&mut GlyphAnimation, &mut LoopingAnimationPlayer)>,
    glyph_animation_sources: Res<Assets<GlyphAnimationSource>>,
    time: Res<Time>,
) {
    // TODO: Fix visual glitch caused by wrapping every hour!
    let ellapsed = time.elapsed_seconds_wrapped_f64();

    for (mut animation, mut player) in q_glyph_animation.iter_mut() {
        let Some(source) = glyph_animation_sources.get(animation.source.id()) else {
            continue;
        };

        let start_time = match player.start_time {
            Some(t) => t,
            None => {
                player.start_time = Some(ellapsed);
                ellapsed
            }
        };

        let frame = ((ellapsed - start_time) * player.frame_rate as f64).round() as u32;
        animation.frame = frame.rem_euclid(source.frames.len() as u32);
    }
}
