#![feature(future_join)]

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
        query::Added,
        system::{Commands, Local, Query, Res, ResMut},
    },
    input::gamepad::{GamepadConnection, GamepadConnectionEvent, Gamepads},
    math::{IVec2, UVec2, Vec2},
    render::{
        camera::{Camera, CameraRenderGraph},
        color::Color,
        texture::ImagePlugin,
    },
    time::Time,
    window::{ReceivedCharacter, Window, WindowPlugin, WindowResized, WindowResolution},
    DefaultPlugins,
};

use bevy_ascii_game::{
    atlas::{CharacterSet, FontAtlasPlugin, FontAtlasUser},
    font::{font_load_system, CustomFont, FontSize},
    glyph_animation::GlyphAnimationPlugin,
    glyph_animation_graph::plugin::GlyphAnimationGraphPlugin,
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
        reset::{create_player, create_player_with_gamepad},
        PlayerPlugin,
    },
};
use setup::setup_ui;
use ui::UiPlugin;

pub mod setup;
pub mod ui;

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
        GlyphAnimationPlugin,
        GlyphAnimationGraphPlugin,
        FontAtlasPlugin,
        PhysicsPlugin,
        GlyphRenderPlugin,
        UiPlugin,
    ))
    .add_systems(Startup, (setup_system, setup_ui))
    .add_systems(
        Update,
        (font_load_system, on_resize_system, set_new_font_size),
    );

    #[cfg(debug_assertions)]
    std::fs::write(
        "render-graph.dot",
        bevy_mod_debugdump::render_graph_dot(&app, &Default::default()),
    )
    .unwrap();

    app.run();
}

fn setup_system(mut commands: Commands, server: Res<AssetServer>, gamepads: Res<Gamepads>) {
    // Player
    for gamepad in gamepads.iter() {
        create_player_with_gamepad(&mut commands, &server, gamepad);
    }

    create_player(&mut commands, &server)
        .insert(PlayerInputKeyboardMarker)
        .insert(GlyphSolidColor {
            color: Color::hsl(0.0, 1.0, 0.6).as_rgba_linear() * 10.0,
        });

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
                ..Default::default()
            },
            ..Default::default()
        },
        BloomSettings {
            ..Default::default()
        },
    ));
}

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
fn set_new_font_size(
    mut q_new_font_size: Query<&mut FontSize, Added<FontSize>>,
    res_font_size: ResMut<FontSize>,
) {
    for mut font_size in q_new_font_size.iter_mut() {
        **font_size = res_font_size.0;
    }
}
const CHARSET: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";