use std::sync::Arc;

use bevy::{
    app::{App, PluginGroup, Startup, Update},
    asset::{AssetServer, Assets},
    color::{Color, Hsla},
    core_pipeline::bloom::Bloom,
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
    prelude::Camera2d,
    render::{
        camera::{Camera, CameraRenderGraph, ClearColorConfig},
        sync_world::SyncWorldPlugin,
        texture::ImagePlugin,
    },
    window::{Window, WindowPlugin},
    DefaultPlugins,
};

use bevy_ascii_game::{
    debug::DebugPlugin,
    debug_menu::plugin::DebugMenuPlugin,
    mount::{horse::spawn::create_horse, HorsePlugin},
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
    glyph_animation::GlyphAnimationPlugin,
    glyph_animation_graph::plugin::GlyphAnimationGraphPlugin,
    glyph_render_plugin::{GlyphRenderPlugin, GlyphSolidColor, GlyphTexture, GlyphTextureSource},
    glyph_sprite::{GlyphSprite, GlyphTexturePlugin},
};
use grid_physics::{collision::Aabb, plugin::PhysicsPlugin, solid::SolidPhysicsBundle};
use spatial_grid::{depth::Depth, position::SpatialBundle};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        PlayerPlugin,
        HorsePlugin,
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
        match &ev.connection {
            GamepadConnection::Connected {
                name: _,
                vendor_id: _,
                product_id: _,
            } => {
                create_player_with_gamepad(
                    &mut commands,
                    &server,
                    ev.gamepad,
                    q_main_glyph_buffer.get_single().unwrap(),
                );
            }
            GamepadConnection::Disconnected => {
                for (player, PlayerInputController(gamepad)) in q_players.iter() {
                    if *gamepad == ev.gamepad {
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
        Tilemap(server.load("tilemaps/bridge_base.tilemap.ron")),
        GlyphSolidColor {
            color: Hsla::new(0., 0., 0.15, 1.).into(),
        },
        SolidPhysicsBundle {
            position: IVec2::new(20, 8).into(),
            ..Default::default()
        },
        GamePhysicsGridMarker,
        Depth(-5.0),
    ));
    commands.spawn((
        Tilemap(server.load("tilemaps/output.tilemap.ron")),
        GlyphSolidColor {
            color: Hsla::new(0., 0., 0.25, 1.).into(),
        },
        SolidPhysicsBundle {
            position: IVec2::new(12, 8).into(),
            ..Default::default()
        },
        GamePhysicsGridMarker,
        Depth(5.0),
    ));
    create_player(&mut commands, &server).insert((
        PlayerInputKeyboardMarker,
        GlyphSolidColor {
            color: Color::LinearRgba(Color::hsl(0.0, 1.0, 0.6).to_linear() * 10.0),
        },
        GamePhysicsGridMarker,
    ));

    create_horse(&mut commands, &server);

    // Keyboard display
    commands.spawn((
        GlyphSprite {
            texture: glyph_textures.add(GlyphTextureSource::new_iter(
                32,
                16,
                std::iter::repeat(' '),
            )),
            offset: IVec2::ZERO,
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
            collider: Aabb {
                start: IVec2::new(-1024, 0),
                size: UVec2::new(2048, 2),
            }
            .into(),
            ..Default::default()
        },
        GamePhysicsGridMarker,
    ));
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            hdr: true,
            ..Default::default()
        },
        CameraRenderGraph::new(bevy::core_pipeline::core_2d::graph::Core2d),
        Bloom::default(),
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
