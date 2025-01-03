use bevy::{
    core_pipeline::bloom::Bloom,
    input::mouse::MouseMotion,
    prelude::*,
    render::camera::CameraRenderGraph,
    window::{PrimaryWindow, WindowResolution},
};

use ascii_ui::mouse::input::MouseInput;
use bevy_ascii_game::{
    debug::DebugPlugin,
    physics_grids::{GamePhysicsGridMarker, PhysicsGridPlugin, PrimaryGlyphBufferMarker},
    player::PlayerPlugin,
    tilemap::{component::Tilemap, plugin::TilemapPlugin},
    tileset::plugin::TilesetPlugin,
    widgets::UiSectionsPlugin,
};
use glyph_render::{
    atlas::FontAtlasPlugin,
    font::{font_load_system, FontSize},
    glyph_animation::GlyphAnimationPlugin,
    glyph_animation_graph::plugin::GlyphAnimationGraphPlugin,
    glyph_buffer::GlyphBuffer,
    glyph_render_plugin::GlyphRenderPlugin,
};
use grid_physics::{plugin::PhysicsPlugin, solid::SolidPhysicsBundle};
use spatial_grid::{
    grid::SpatialGrid,
    position::{Position, SpatialBundle},
    remainder::Remainder,
    PositionPropagationPlugin,
};
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
                    resolution: WindowResolution::default().with_scale_factor_override(1.0),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        PlayerPlugin,
        (
            FontAtlasPlugin,
            GlyphRenderPlugin,
            GlyphAnimationPlugin,
            GlyphAnimationGraphPlugin,
        ),
        (TilesetPlugin, TilemapPlugin),
        (PositionPropagationPlugin, PhysicsPlugin, PhysicsGridPlugin),
        (TilesetPanelPlugin, DebugPlugin, UiSectionsPlugin),
    ))
    .add_systems(Startup, setup_system)
    .add_systems(Update, (font_load_system, zoom_system, pan_system));

    app.run();
}

fn setup_system(mut commands: Commands, server: Res<AssetServer>) {
    commands
        .spawn((
            Tilemap(server.load("tilemaps/output_bridge.tilemap.ron")),
            SolidPhysicsBundle {
                position: SpatialBundle::from(IVec2::new(20, 10)),
                ..Default::default()
            },
        ))
        .insert(GamePhysicsGridMarker);

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

fn zoom_system(
    // mut ev_scroll: EventReader<MouseWheel>,
    mut mouse_input: ResMut<MouseInput>,
    mut size: Local<f32>,
    mut q_glyph_buffer: Query<
        (&mut FontSize, &mut SpatialGrid, &mut GlyphBuffer),
        With<PrimaryGlyphBufferMarker>,
    >,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    // let distance = mouse_input.scroll().unwrap_or_default().y;
    let distance = mouse_input
        .consume()
        .unwrap_or_default()
        .scroll
        .unwrap_or_default()
        .y;

    let factor = (distance / 16.0).exp();
    *size *= factor;
    *size = size.clamp(2.0, 128.0);

    let window = window.get_single().unwrap();

    for (mut font_size, mut grid, mut buffer) in q_glyph_buffer.iter_mut() {
        let size = *size as u32;
        **font_size = size;
        grid.step = UVec2::new(font_size.advance(), font_size.line_spacing());

        buffer.size.x = (window.width() / grid.step.x as f32) as u32;
        buffer.size.y = (window.height() / grid.step.y as f32) as u32;
    }
}

fn pan_system(
    mut q_grid: Query<
        (&mut Position, &mut Remainder, &FontSize),
        (With<SpatialGrid>, With<PrimaryGlyphBufferMarker>),
    >,
    mut ev_mouse: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    let mut motion = Vec2::ZERO;

    if mouse_buttons.pressed(MouseButton::Middle) {
        for ev in ev_mouse.read() {
            motion += ev.delta;
        }
    }

    let motion = motion * Vec2::new(-1.0, 1.0) * 0.5;
    for (mut position, mut remainder, font_size) in q_grid.iter_mut() {
        **remainder +=
            motion / Vec2::new(font_size.advance() as f32, font_size.line_spacing() as f32);

        let delta = remainder.round();
        **remainder -= delta;
        **position += delta.as_ivec2();
    }
}
