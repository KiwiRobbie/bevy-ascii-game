#![feature(future_join)]

use bevy::{
    app::{App, PluginGroup, Startup, Update},
    asset::{AssetApp, AssetServer, Assets},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        system::{Commands, Local, Query, Res, ResMut},
    },
    input::gamepad::{Gamepad, GamepadConnection, GamepadConnectionEvent, Gamepads},
    math::{IVec2, UVec2, Vec2},
    render::{camera::CameraRenderGraph, color::Color, texture::ImagePlugin},
    time::Time,
    window::{ReceivedCharacter, Window, WindowPlugin, WindowResized, WindowResolution},
    DefaultPlugins,
};

use bevy_ascii_game::{
    atlas::{CharacterSet, FontAtlasPlugin, FontAtlasUser},
    font::{font_load_system, CustomFont, CustomFontLoader, CustomFontSource, FontSize},
    glyph_animation::{GlyphAnimationAssetLoader, GlyphAnimationSource},
    glyph_animation_graph::{bundle::GlyphAnimationGraphBundle, plugin::GlyphAnimationGraphPlugin},
    glyph_render_plugin::{GlyphRenderPlugin, GlyphSolidColor, GlyphSprite, GlyphTexture},
    physics::{
        actor::ActorPhysicsBundle,
        collision::{Aabb, Collider, CollisionShape},
        free::FreeMarker,
        gravity::Gravity,
        movement::Movement,
        plugin::PhysicsPlugin,
        position::{Position, PositionBundle},
        solid::{FilterSolids, SolidPhysicsBundle},
        velocity::Velocity,
    },
    player::{
        input::{controller::PlayerInputController, keyboard::PlayerInputKeyboardMarker},
        movement::{walk::PlayerWalkSpeed, PlayerMovementBundle},
        reset::{create_player, create_player_with_gamepad},
        PlayerBundle, PlayerPlugin,
    },
};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::default().with_scale_factor_override(1.0),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        PlayerPlugin,
        GlyphAnimationGraphPlugin,
    ))
    .add_plugins((FontAtlasPlugin, PhysicsPlugin, GlyphRenderPlugin))
    .init_asset::<GlyphAnimationSource>()
    .init_asset_loader::<GlyphAnimationAssetLoader>()
    .init_resource::<FontSize>()
    .add_systems(Startup, setup_system)
    .add_systems(
        Update,
        (
            keyboard_input_system,
            font_load_system,
            on_resize_system,
            // looping_animation_player_system,
            moving_platform,
            handle_gamepads,
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

fn moving_platform(
    mut q_solid_movement: Query<(&mut Movement, &mut Velocity, &Position), FilterSolids>,
    time: Res<Time>,
) {
    for (mut movement, mut velocity, position) in q_solid_movement.iter_mut() {
        if position.position.x > 50 {
            velocity.velocity.x = -10.0;
        } else if position.position.x < -50 {
            velocity.velocity.x = 10.0;
        }
        movement.add(velocity.velocity * time.delta_seconds());
    }
}

fn handle_gamepads(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut ev_gamepad: EventReader<GamepadConnectionEvent>,
    q_players: Query<(Entity, &PlayerInputController)>,
) {
    for ev in ev_gamepad.read() {
        match ev.connection {
            GamepadConnection::Connected(_) => {
                create_player_with_gamepad(&mut commands, &server, ev.gamepad);
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
    gamepads: Res<Gamepads>,
) {
    // Player

    for gamepad in gamepads.iter() {
        create_player_with_gamepad(&mut commands, &server, gamepad);
    }
    //         .insert(GlyphSolidColor {
    //         });
    // }
    create_player(&mut commands, &server)
        .insert(PlayerInputKeyboardMarker)
        .insert(GlyphSolidColor {
            color: Color::hsl(0.0, 1.0, 0.6),
        });

    // Sliding Box
    commands.spawn((
        GlyphSprite {
            texture: glyph_textures.add(GlyphTexture {
                data: (0..2).map(|_| "#".repeat(3)).collect::<Vec<String>>(),
            }),
            offset: IVec2::ZERO,
        },
        FontAtlasUser,
        CustomFont(server.load("FiraCode-Regular.ttf")),
        CharacterSet(CHARSET.chars().collect()),
        FontSize(32),
        SolidPhysicsBundle {
            collider: Collider {
                shape: CollisionShape::Aabb(Aabb {
                    min: IVec2::ZERO,
                    size: UVec2 { x: 3, y: 2 },
                }),
            },
            ..Default::default()
        },
        Movement::default(),
        Velocity {
            velocity: Vec2 { x: 50.0, y: 0.0 },
        },
    ));

    // Stationary box
    commands.spawn((
        GlyphSprite {
            texture: glyph_textures.add(GlyphTexture {
                data: (0..2).map(|_| "#".repeat(3)).collect::<Vec<String>>(),
            }),
            offset: IVec2::ZERO,
        },
        FontAtlasUser,
        CustomFont(server.load("FiraCode-Regular.ttf")),
        CharacterSet(CHARSET.chars().collect()),
        FontSize(32),
        SolidPhysicsBundle {
            position: IVec2::new(-30, 0).into(),
            collider: Collider {
                shape: CollisionShape::Aabb(Aabb {
                    min: IVec2::ZERO,
                    size: UVec2 { x: 3, y: 2 },
                }),
            },
            ..Default::default()
        },
        Movement::default(),
        Velocity {
            velocity: Vec2 { x: 0.0, y: 0.0 },
        },
    ));

    // Falling box
    commands.spawn((
        GlyphSprite {
            texture: glyph_textures.add(GlyphTexture {
                data: (0..2).map(|_| ".".repeat(3)).collect::<Vec<String>>(),
            }),
            offset: IVec2::ZERO,
        },
        FontAtlasUser,
        CustomFont(server.load("FiraCode-Regular.ttf")),
        CharacterSet(CHARSET.chars().collect()),
        FontSize(32),
        ActorPhysicsBundle {
            collider: Collider {
                shape: CollisionShape::Aabb(Aabb {
                    min: IVec2::ZERO,
                    size: UVec2 { x: 3, y: 2 },
                }),
            },
            position: IVec2::new(-30, -10).into(),
            ..Default::default()
        },
        Velocity {
            velocity: Vec2 { x: 50.0, y: 70.0 },
        },
        Gravity::default(),
        FreeMarker,
    ));

    // Keyboard display
    commands.spawn((
        CustomFont(server.load("FiraCode-Regular.ttf")),
        FontAtlasUser,
        FontSize(32),
        GlyphSprite {
            texture: glyph_textures.add(GlyphTexture {
                data: (0..16).map(|_| " ".repeat(32)).collect::<Vec<String>>(),
            }),
            offset: IVec2 { x: -16, y: -8 },
        },
        PositionBundle {
            ..Default::default()
        },
        KeyboardInputMarker,
    ));

    // Floor
    commands.spawn((
        GlyphSprite {
            texture: glyph_textures.add(GlyphTexture {
                data: (0..2).map(|_| "#".repeat(100)).collect::<Vec<String>>(),
            }),
            offset: IVec2::ZERO,
        },
        FontAtlasUser,
        CustomFont(server.load("FiraCode-Regular.ttf")),
        CharacterSet(CHARSET.chars().collect()),
        FontSize(32),
        SolidPhysicsBundle {
            position: IVec2::new(-50, -12).into(),
            collider: Collider {
                shape: CollisionShape::Aabb(Aabb {
                    min: IVec2::ZERO,
                    size: UVec2 { x: 100, y: 2 },
                }),
            },
            ..Default::default()
        },
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
    let width = glyph_texture.data.first().unwrap().len();
    let height = glyph_texture.data.len();

    fn get_pos(index: usize, width: usize, height: usize) -> (usize, usize) {
        (
            index.rem_euclid(width),
            index.div_euclid(width).rem_euclid(height),
        )
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

// #[derive(Component)]
// pub struct LoopingAnimationPlayer {
//     pub frame_rate: u32,
//     pub start_time: Option<f64>,
// }
// impl LoopingAnimationPlayer {
//     fn new(frame_rate: u32) -> Self {
//         Self {
//             frame_rate,
//             start_time: None,
//         }
//     }
// }

// fn looping_animation_player_system(
//     mut q_glyph_animation: Query<(&mut GlyphAnimation, &mut LoopingAnimationPlayer)>,
//     glyph_animation_sources: Res<Assets<GlyphAnimationSource>>,
//     time: Res<Time>,
// ) {
//     // TODO: Fix visual glitch caused by wrapping every hour!
//     let elapsed = time.elapsed_seconds_wrapped_f64();

//     for (mut animation, mut player) in q_glyph_animation.iter_mut() {
//         let Some(source) = glyph_animation_sources.get(animation.source.id()) else {
//             continue;
//         };

//         let start_time = match player.start_time {
//             Some(t) => t,
//             None => {
//                 player.start_time = Some(elapsed);
//                 elapsed
//             }
//         };

//         let frame = ((elapsed - start_time) * player.frame_rate as f64).round() as u32;
//         animation.frame = frame.rem_euclid(source.frames.len() as u32);
//     }
// }

fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut q_font_size: Query<&mut FontSize>,
    mut res_font_size: ResMut<FontSize>,
) {
    if let Some(e) = resize_reader.read().last() {
        let size = (e.width / 60.0) as u32;
        for mut font_size in q_font_size.iter_mut() {
            **font_size = size
        }
        **res_font_size = size;
    }
}
const CHARSET: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
