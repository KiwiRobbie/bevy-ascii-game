#![feature(future_join)]
use bevy::{
    app::{App, PluginGroup, Startup, Update},
    asset::AssetServer,
    core_pipeline::{
        bloom::BloomSettings,
        core_2d::{Camera2d, Camera2dBundle},
    },
    ecs::{
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    math::IVec2,
    render::{
        camera::{Camera, CameraRenderGraph},
        color::Color,
        texture::ImagePlugin,
    },
    time::Time,
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};

use bevy_ascii_game::{
    debug_menu::plugin::DebugMenuPlugin,
    physics_grids::{GamePhysicsGridMarker, PhysicsGridPlugin},
    player::PlayerPlugin,
    tilemap::{component::Tilemap, plugin::TilemapPlugin},
    tileset::plugin::TilesetPlugin,
};
use glyph_render::{
    atlas::FontAtlasPlugin, font::font_load_system, glyph_animation::GlyphAnimationPlugin,
    glyph_animation_graph::plugin::GlyphAnimationGraphPlugin,
    glyph_render_plugin::GlyphRenderPlugin,
};
use grid_physics::{
    movement::Movement,
    plugin::PhysicsPlugin,
    sets::physics_systems_enabled,
    solid::{FilterSolids, SolidPhysicsBundle},
    velocity::Velocity,
};
use spatial_grid::position::{Position, PositionBundle};
use tileset_panel::{plugin::TilesetPanelPlugin, state::TilesetPanelState};

mod list_builder_widget;
mod tileset_panel;
mod tileset_widget;
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
    ))
    .add_systems(Startup, setup_system)
    .add_systems(
        Update,
        (
            font_load_system,
            moving_platform.run_if(physics_systems_enabled),
        ),
    );

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

fn setup_system(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut tileset_panel: ResMut<TilesetPanelState>,
) {
    commands
        .spawn((
            Tilemap(server.load("tilemaps/cave_map.tilemap.ron")),
            SolidPhysicsBundle {
                position: PositionBundle::from(IVec2::new(20, 10)),
                ..Default::default()
            },
        ))
        .insert(GamePhysicsGridMarker);

    tileset_panel.tilesets = vec![server.load("tilemaps/cave_map.tilemap.ron")];

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
