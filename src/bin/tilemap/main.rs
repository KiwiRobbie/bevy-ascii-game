#![feature(future_join)]
use bevy::{
    app::{App, PluginGroup, Startup, Update},
    asset::AssetServer,
    core_pipeline::{
        bloom::BloomSettings,
        core_2d::{Camera2d, Camera2dBundle},
    },
    ecs::system::{Commands, Res},
    math::IVec2,
    render::{
        camera::{Camera, CameraRenderGraph},
        color::Color,
        texture::ImagePlugin,
    },
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};

use bevy_ascii_game::{
    debug::DebugPlugin,
    physics_grids::{GamePhysicsGridMarker, PhysicsGridPlugin},
    player::PlayerPlugin,
    tilemap::{component::Tilemap, plugin::TilemapPlugin},
    tileset::plugin::TilesetPlugin,
    widgets::UiSectionsPlugin,
};
use glyph_render::{
    atlas::FontAtlasPlugin, font::font_load_system, glyph_animation::GlyphAnimationPlugin,
    glyph_animation_graph::plugin::GlyphAnimationGraphPlugin,
    glyph_render_plugin::GlyphRenderPlugin,
};
use grid_physics::{plugin::PhysicsPlugin, solid::SolidPhysicsBundle};
use spatial_grid::position::SpatialBundle;
use tileset_panel::plugin::TilesetPanelPlugin;

mod list_builder_widget;
mod tileset_panel;
fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    resolution: WindowResolution::default().with_scale_factor_override(1.0),
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
        GlyphRenderPlugin,
        TilesetPanelPlugin,
        PhysicsGridPlugin,
        DebugPlugin,
        UiSectionsPlugin,
    ))
    .add_systems(Startup, setup_system)
    .add_systems(Update, (font_load_system,));

    #[cfg(debug_assertions)]
    std::fs::write(
        "render-graph.dot",
        bevy_mod_debugdump::render_graph_dot(&app, &Default::default()),
    )
    .unwrap();

    app.run();
}

fn setup_system(mut commands: Commands, server: Res<AssetServer>) {
    commands
        .spawn((
            Tilemap(server.load("tilemaps/output.tilemap.ron")),
            SolidPhysicsBundle {
                position: SpatialBundle::from(IVec2::new(20, 10)),
                ..Default::default()
            },
        ))
        .insert(GamePhysicsGridMarker);

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
