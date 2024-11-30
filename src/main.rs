use std::sync::Arc;

use bevy::{
    app::{App, PluginGroup, Startup, Update},
    asset::{AssetServer, Assets},
    color::{Color, Hsla},
    core_pipeline::{bloom::BloomSettings, core_2d::Camera2dBundle},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::With,
        system::{Commands, Local, Query, Res, ResMut},
    },
    input::{
        gamepad::{GamepadConnection, GamepadConnectionEvent},
        keyboard::{Key, KeyboardInput},
    },
    math::{IVec2, UVec2},
    render::{
        camera::{Camera, CameraRenderGraph},
        texture::ImagePlugin,
    },
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};

use bevy_ascii_game::{
    debug::DebugPlugin,
    debug_menu::plugin::DebugMenuPlugin,
    physics_grids::{GamePhysicsGridMarker, PhysicsGridPlugin, PrimaryGlyphBufferMarker},
    player::{
        input::{controller::PlayerInputController, keyboard::PlayerInputKeyboardMarker},
        reset::{create_player, create_player_with_gamepad},
        PlayerPlugin,
    },
    tilemap::{component::Tilemap, plugin::TilemapPlugin},
    tileset::plugin::TilesetPlugin,
    widgets::UiSectionsPlugin,
};
use glyph_render::{
    atlas::FontAtlasPlugin,
    font::font_load_system,
    glyph_animation::{player::GlyphAnimationPlayer, GlyphAnimation, GlyphAnimationPlugin},
    glyph_animation_graph::plugin::GlyphAnimationGraphPlugin,
    glyph_render_plugin::{GlyphRenderPlugin, GlyphSolidColor, GlyphTexture, GlyphTextureSource},
    glyph_sprite::{GlyphSprite, GlyphTexturePlugin},
};
use grid_physics::{
    actor::ActorPhysicsBundle,
    collision::{Aabb, Collider},
    free::FreeMarker,
    gravity::Gravity,
    plugin::PhysicsPlugin,
    solid::SolidPhysicsBundle,
    velocity::Velocity,
};
use spatial_grid::{depth::Depth, position::SpatialBundle};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::default().with_scale_factor_override(1.0),
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        PlayerPlugin,
        GlyphAnimationPlugin,
        GlyphAnimationGraphPlugin,
        FontAtlasPlugin,
        TilesetPlugin,
        TilemapPlugin,
        PhysicsPlugin,
        GlyphTexturePlugin,
        GlyphRenderPlugin,
        DebugPlugin,
        DebugMenuPlugin,
        PhysicsGridPlugin,
        UiSectionsPlugin,
    ))
    .add_systems(Startup, setup_system)
    .add_systems(
        Update,
        (keyboard_input_system, font_load_system, handle_gamepads),
    );

    app.run();
}

fn handle_gamepads(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut ev_gamepad: EventReader<GamepadConnectionEvent>,
    q_players: Query<(Entity, &PlayerInputController)>,
    q_main_glyph_buffer: Query<Entity, With<PrimaryGlyphBufferMarker>>,
) {
    for ev in ev_gamepad.read() {
        match ev.connection {
            GamepadConnection::Connected(_) => {
                create_player_with_gamepad(
                    &mut commands,
                    &server,
                    ev.gamepad,
                    q_main_glyph_buffer.get_single().unwrap(),
                );
            }
            GamepadConnection::Disconnected => {
                for (player, PlayerInputController(gamepad)) in q_players.iter() {
                    if gamepad.id == ev.gamepad.id {
                        commands.entity(player).despawn();
                    }
                }
            }
        }
    }
}

fn setup_system(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
) {
    commands.spawn((
        Tilemap(server.load("tilemaps/output.tilemap.ron")),
        GlyphSolidColor {
            color: Hsla {
                hue: 0.0,
                saturation: 0.0,
                lightness: 0.2,
                alpha: 1.0,
            }
            .into(),
        },
        SolidPhysicsBundle {
            position: SpatialBundle::from(IVec2::new(20, 10)),
            ..Default::default()
        },
        GamePhysicsGridMarker,
        Depth(-1.0),
    ));

    create_player(&mut commands, &server).insert((
        PlayerInputKeyboardMarker,
        GlyphSolidColor {
            color: Color::LinearRgba(Color::hsl(0.0, 1.0, 0.6).to_linear() * 10.0),
        },
        GamePhysicsGridMarker,
    ));
    commands.spawn((
        GlyphAnimation {
            source: server.load("anim/horse/states/mounted/gallop.anim.ron"),
            frame: 0,
        },
        GlyphAnimationPlayer {
            framerate: 10.0,
            repeat: true,
            frame_timer: 0.0,
        },
        ActorPhysicsBundle {
            collider: Collider {
                shape: Aabb {
                    start: IVec2::new(0, 0),
                    size: UVec2 { x: 30, y: 10 },
                }
                .into(),
            },
            position: IVec2::new(10, 10).into(),
            ..Default::default()
        },
        FreeMarker,
        Gravity::default(),
        Velocity::default(),
        GamePhysicsGridMarker,
        Depth(0.5),
    ));

    commands.spawn((
        GlyphSprite {
            texture: server.load("art/dj/dj.art"),
            offset: IVec2::ZERO,
        },
        ActorPhysicsBundle {
            collider: Collider {
                shape: Aabb {
                    start: IVec2::new(0, 0),
                    size: UVec2 { x: 50, y: 10 },
                }
                .into(),
            },
            position: IVec2::new(40, 10).into(),
            ..Default::default()
        },
        GlyphSolidColor {
            color: Color::srgba_u8(0xff, 0x61, 0x88, 0xff),
        },
        FreeMarker,
        Gravity::default(),
        Velocity::default(),
        GamePhysicsGridMarker,
        Depth(0.5),
    ));

    // Keyboard display
    commands.spawn((
        GlyphSprite {
            texture: glyph_textures.add(GlyphTextureSource::new_iter(
                32,
                16,
                std::iter::repeat(' '),
            )),
            offset: IVec2 { x: 0, y: 0 },
        },
        SpatialBundle {
            ..Default::default()
        },
        KeyboardInputMarker,
        GamePhysicsGridMarker,
        Depth(2.0),
    ));

    // Floor
    commands.spawn((
        SolidPhysicsBundle {
            position: IVec2::new(0, 0).into(),
            collider: Collider {
                shape: Aabb {
                    start: IVec2::ZERO,
                    size: UVec2 { x: 100, y: 2 },
                }
                .into(),
            },
            ..Default::default()
        },
        GamePhysicsGridMarker,
    ));

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: bevy::render::camera::ClearColorConfig::Custom(Color::BLACK),
                hdr: true,
                ..Default::default()
            },
            camera_render_graph: CameraRenderGraph::new(
                bevy::core_pipeline::core_2d::graph::Core2d,
            ),
            ..Default::default()
        },
        BloomSettings {
            ..Default::default()
        },
    ));
}

#[derive(Component)]
struct KeyboardInputMarker;

fn keyboard_input_system(
    mut ev_keyboard: EventReader<KeyboardInput>,
    q_glyph_sprite: Query<&GlyphSprite, With<KeyboardInputMarker>>,
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut position: Local<usize>,
) {
    let Some(glyph_sprite) = q_glyph_sprite.get_single().ok() else {
        return;
    };

    let glyph_texture = glyph_textures.get_mut(glyph_sprite.texture.id()).unwrap();
    let width = glyph_texture.source.width;
    let height = glyph_texture.source.height;
    let mut data = glyph_texture.source.data.clone();

    fn get_pos(index: usize, width: usize, height: usize) -> (usize, usize) {
        (
            index.rem_euclid(width),
            index.div_euclid(width).rem_euclid(height),
        )
    }

    for key in ev_keyboard.read() {
        if !key.state.is_pressed() {
            continue;
        }
        match &key.logical_key {
            Key::Backspace => {
                *position = (*position + width * height - 1).rem_euclid(width * height);

                let (x, y) = get_pos(*position, width, height);
                let index = x + width * y;
                data[index] = '_';

                let (x, y) = get_pos(*position + 1, width, height);
                let index = x + width * y;
                data[index] = ' ';
            }
            Key::Character(key) => {
                let (x, y) = get_pos(*position, width, height);
                let index = x + width * y;
                data[index] = key.as_str().chars().next().unwrap();

                let (x, y) = get_pos(*position + 1, width, height);
                let index = x + width * y;
                data[index] = '_';

                *position = (*position + 1).rem_euclid(width * height);
            }
            _ => {}
        }
    }
    *glyph_texture = GlyphTexture::new(Arc::new(GlyphTextureSource::new(width, height, data)));
}
