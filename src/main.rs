use bevy::{
    app::{App, PluginGroup, Startup, Update},
    asset::{AssetServer, Assets},
    core_pipeline::{
        bloom::BloomSettings,
        core_2d::{Camera2d, Camera2dBundle},
    },
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::With,
        system::{Commands, Local, Query, Res, ResMut},
    },
    input::gamepad::{GamepadConnection, GamepadConnectionEvent},
    log::{self, Level, LogPlugin},
    math::{IVec2, UVec2},
    render::{
        camera::{Camera, CameraRenderGraph},
        color::Color,
        texture::ImagePlugin,
    },
    window::{ReceivedCharacter, Window, WindowPlugin, WindowResolution},
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
    glyph_render_plugin::{
        GlyphRenderPlugin, GlyphSolidColor, GlyphSprite, GlyphSpriteMirrored, GlyphTexture,
    },
};
use grid_physics::{
    actor::ActorPhysicsBundle,
    collision::{Aabb, Collider, CollisionShape},
    free::FreeMarker,
    gravity::Gravity,
    plugin::PhysicsPlugin,
    solid::SolidPhysicsBundle,
    velocity::Velocity,
};
use spatial_grid::position::SpatialBundle;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    resolution: WindowResolution::default().with_scale_factor_override(1.0),
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        // .set(LogPlugin {
        //     filter: "wgpu=info".to_string(),
        //     level: Level::DEBUG,
        // }),
        PlayerPlugin,
        GlyphAnimationPlugin,
        GlyphAnimationGraphPlugin,
        FontAtlasPlugin,
        TilesetPlugin,
        TilemapPlugin,
        PhysicsPlugin,
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

    #[cfg(debug_assertions)]
    std::fs::write(
        "render-graph.dot",
        bevy_mod_debugdump::render_graph_dot(&app, &Default::default()),
    )
    .unwrap();

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
            color: Color::Hsla {
                hue: 0.0,
                saturation: 0.0,
                lightness: 0.2,
                alpha: 1.0,
            },
        },
        SolidPhysicsBundle {
            position: SpatialBundle::from(IVec2::new(20, 10)),
            ..Default::default()
        },
        GamePhysicsGridMarker,
    ));

    create_player(&mut commands, &server).insert((
        PlayerInputKeyboardMarker,
        GlyphSolidColor {
            color: Color::hsl(0.0, 1.0, 0.6).as_rgba_linear() * 10.0,
        },
        GamePhysicsGridMarker,
    ));
    commands.spawn((
        GlyphAnimation {
            source: server.load("anim/horse/states/gallop.anim.ron"),
            frame: 0,
        },
        GlyphAnimationPlayer {
            framerate: 10.0,
            repeat: true,
            frame_timer: 0.0,
        },
        ActorPhysicsBundle {
            collider: Collider {
                shape: CollisionShape::Aabb(Aabb {
                    min: IVec2::new(0, 0),
                    size: UVec2 { x: 30, y: 10 },
                }),
            },
            position: IVec2::new(10, 10).into(),
            ..Default::default()
        },
        FreeMarker,
        Gravity::default(),
        Velocity::default(),
        GamePhysicsGridMarker,
    ));
    commands.spawn((
        GlyphAnimation {
            source: server.load("anim/horse/states/gallop.anim.ron"),
            frame: 0,
        },
        GlyphAnimationPlayer {
            framerate: 10.0,
            repeat: true,
            frame_timer: 0.0,
        },
        ActorPhysicsBundle {
            collider: Collider {
                shape: CollisionShape::Aabb(Aabb {
                    min: IVec2::new(0, 0),
                    size: UVec2 { x: 30, y: 10 },
                }),
            },
            position: IVec2::new(-30, 0).into(),
            ..Default::default()
        },
        FreeMarker,
        Gravity::default(),
        Velocity::default(),
        GlyphSpriteMirrored,
        GamePhysicsGridMarker,
    ));

    // Keyboard display
    commands.spawn((
        GlyphSprite {
            texture: glyph_textures.add(GlyphTexture::new(
                (0..16).map(|_| " ".repeat(32)).collect::<Vec<String>>(),
            )),
            offset: IVec2 { x: 0, y: 0 },
        },
        SpatialBundle {
            ..Default::default()
        },
        KeyboardInputMarker,
        GamePhysicsGridMarker,
    ));

    // Floor
    commands.spawn((
        SolidPhysicsBundle {
            position: IVec2::new(0, 0).into(),
            collider: Collider {
                shape: CollisionShape::Aabb(Aabb {
                    min: IVec2::ZERO,
                    size: UVec2 { x: 100, y: 2 },
                }),
            },
            ..Default::default()
        },
        GamePhysicsGridMarker,
    ));
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            camera_render_graph: CameraRenderGraph::new(bevy::core_pipeline::core_2d::graph::NAME),
            camera_2d: Camera2d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                    Color::BLACK,
                ),
            },
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
    mut ev_character: EventReader<ReceivedCharacter>,
    q_glyph_sprite: Query<&GlyphSprite, &KeyboardInputMarker>,
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut position: Local<usize>,
) {
    let Some(glyph_sprite) = q_glyph_sprite.get_single().ok() else {
        return;
    };

    let glyph_texture = glyph_textures.get_mut(glyph_sprite.texture.id()).unwrap();
    let width = glyph_texture.source.data.first().unwrap().len();
    let height = glyph_texture.source.data.len();

    fn get_pos(index: usize, width: usize, height: usize) -> (usize, usize) {
        (
            index.rem_euclid(width),
            index.div_euclid(width).rem_euclid(height),
        )
    }

    for character in ev_character.read() {
        log::info!("{:?}", character);
        let mut data = glyph_texture.source.data.clone();
        if character.char == '\u{8}' {
            *position = (*position + width * height - 1).rem_euclid(width * height);
            let (x, y) = get_pos(*position, width, height);
            data[y].replace_range(x..=x, "_");
            let (x, y) = get_pos(*position + 1, width, height);
            data[y].replace_range(x..=x, " ");
        } else {
            let (x, y) = get_pos(*position, width, height);
            data[y].replace_range(x..=x, character.char.to_string().as_str());
            let (x, y) = get_pos(*position + 1, width, height);
            data[y].replace_range(x..=x, "_");

            *position = (*position + 1).rem_euclid(width * height);
        }
        *glyph_texture = GlyphTexture::new(data);
    }
}
