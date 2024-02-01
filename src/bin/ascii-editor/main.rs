#![feature(future_join)]

use bevy::{
    app::{App, PluginGroup, Startup, Update},
    asset::AssetServer,
    core_pipeline::{
        bloom::BloomSettings,
        core_2d::{Camera2d, Camera2dBundle},
    },
    ecs::{
        entity::Entity,
        event::EventReader,
        query::{Added, With},
        reflect::AppTypeRegistry,
        system::{Commands, Query, Res, ResMut},
        world::World,
    },
    input::gamepad::Gamepads,
    math::UVec2,
    reflect::TypeRegistryArc,
    render::{
        camera::{Camera, CameraRenderGraph},
        color::Color,
        texture::ImagePlugin,
    },
    scene::{DynamicScene, DynamicSceneBuilder},
    transform::components::{GlobalTransform, Transform},
    window::{Window, WindowPlugin, WindowResized, WindowResolution},
    DefaultPlugins,
};

use ascii_ui as ui;
use bevy_ascii_game::player::{
    input::keyboard::PlayerInputKeyboardMarker,
    reset::{create_player, create_player_with_gamepad},
    PlayerPlugin,
};
use glyph_render::{
    atlas::FontAtlasPlugin,
    font::{font_load_system, FontSize},
    glyph_animation::GlyphAnimationPlugin,
    glyph_animation_graph::plugin::GlyphAnimationGraphPlugin,
    glyph_render_plugin::{GlyphRenderPlugin, GlyphSolidColor},
};
use grid_physics::{plugin::PhysicsPlugin, position::GridSize};
use setup::setup_ui;
use ui::{
    layout::positioned::Positioned,
    plugin::{UiPlugin, UiTypesPlugin},
    widgets::container::Container,
};

pub mod setup;

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
        UiTypesPlugin,
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
    mut grid_size: ResMut<GridSize>,
) {
    if let Some(e) = resize_reader.read().last() {
        let size = (e.width / 60.0) as u32;
        for mut font_size in q_font_size.iter_mut() {
            **font_size = size
        }
        **res_font_size = size;
        **grid_size = UVec2::new(res_font_size.advance(), res_font_size.line_spacing());
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
