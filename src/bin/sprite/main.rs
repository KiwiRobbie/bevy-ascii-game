use ascii_ui::mouse::input::MouseInput;
use bevy::{
    core_pipeline::bloom::Bloom,
    input::mouse::MouseMotion,
    prelude::*,
    render::camera::CameraRenderGraph,
    window::{PrimaryWindow, WindowResolution},
};

use bevy_ascii_game::{
    debug::DebugPlugin,
    physics_grids::{GamePhysicsGridMarker, PhysicsGridPlugin, PrimaryGlyphBufferMarker},
    tilemap::plugin::TilemapPlugin,
    tileset::plugin::TilesetPlugin,
    widgets::UiSectionsPlugin,
};
use editor_panel::plugin::TilesetPanelPlugin;
use glyph_render::{
    atlas::FontAtlasPlugin,
    font::{font_load_system, FontSize},
    glyph_animation::GlyphAnimationPlugin,
    glyph_animation_graph::plugin::GlyphAnimationGraphPlugin,
    glyph_buffer::GlyphBuffer,
    glyph_render_plugin::GlyphRenderPlugin,
};
use grid_physics::plugin::PhysicsPlugin;
use layers::{EditorLayer, EditorLayerPlugin, SelectedEditorLayer};
use spatial_grid::{
    grid::SpatialGrid,
    position::{Position, SpatialTraits},
    remainder::Remainder,
    PositionPropagationPlugin,
};
use tools::EditorToolsPlugin;

mod editor_panel;
mod input;
mod layers;
mod tools;
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
        (EditorToolsPlugin, EditorLayerPlugin),
        PositionPropagationPlugin,
        // PlayerPlugin,
        (
            GlyphRenderPlugin,
            GlyphAnimationPlugin,
            GlyphAnimationGraphPlugin,
        ),
        FontAtlasPlugin,
        (TilesetPlugin, TilemapPlugin),
        PhysicsPlugin,
        TilesetPanelPlugin,
        PhysicsGridPlugin,
        DebugPlugin,
        UiSectionsPlugin,
    ))
    .add_systems(Startup, setup_system)
    .add_systems(
        Update,
        (
            font_load_system,
            mouse_zoom_system,
            mouse_pan_system,
            // keyboard_pan_system,
        ),
    );

    app.run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn((
        EditorLayer::new(IVec2::new(0, 0), UVec2::new(64, 32)),
        SelectedEditorLayer,
        GamePhysicsGridMarker,
    ));

    // commands
    //     .spawn((
    //         Tilemap(server.load("tilemaps/sprite.tilemap.ron")),
    //         SolidPhysicsBundle {
    //             position: SpatialBundle::from(IVec2::new(20, 10)),
    //             ..Default::default()
    //         },
    //     ))
    //     .insert(GamePhysicsGridMarker);

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

    // commands
    //     .spawn((
    //         GlyphSprite {
    //             texture: glyph_textures.add(GlyphTexture::new(Arc::new(GlyphTextureSource::new(
    //                 1,
    //                 1,
    //                 Box::new(['#']),
    //             )))),
    //             offset: IVec2 { x: 0, y: 0 },
    //         },
    //         GlyphSolidColor { color: RED.into() },
    //         SpatialBundle {
    //             ..Default::default()
    //         },
    //         Depth(-1.0),
    //         EditorCursorMarker,
    //     ))
    //     .insert(GamePhysicsGridMarker);
}

fn mouse_zoom_system(
    mut mouse_input: ResMut<MouseInput>,
    mut size: Local<f32>,
    mut q_glyph_buffer: Query<
        (&mut FontSize, &mut SpatialGrid, &mut GlyphBuffer),
        With<PrimaryGlyphBufferMarker>,
    >,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let distance = mouse_input
        .consume()
        .unwrap_or_default()
        .scroll
        .unwrap_or_default()
        .y;

    let factor = (distance / 16.0).exp();
    *size *= factor;
    dbg!(factor);
    *size = size.clamp(2.0, 128.0);

    dbg!(&size);
    let window = window.get_single().unwrap();

    for (mut font_size, mut grid, mut buffer) in q_glyph_buffer.iter_mut() {
        let size = *size as u32;
        **font_size = size;
        grid.step = UVec2::new(font_size.advance(), font_size.line_spacing());

        buffer.size.x = (window.width() / grid.step.x as f32) as u32;
        buffer.size.y = (window.height() / grid.step.y as f32) as u32;
    }
}

fn mouse_pan_system(
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
    } else {
        ev_mouse.clear();
    }

    let motion = motion * Vec2::new(-1.0, 1.0) * 0.5;
    for (position, remainder, font_size) in q_grid.iter_mut() {
        (position, remainder).offset(
            motion / Vec2::new(font_size.advance() as f32, font_size.line_spacing() as f32),
        );
    }
}

// fn keyboard_pan_system(
//     mut evr_kbd: EventReader<KeyboardInput>,
//     mut q_cursor: Query<
//         (&mut Position, &mut Remainder),
//         (With<EditorCursorMarker>, Without<SpatialGrid>),
//     >,
//     mut q_grid: Query<
//         (&mut Position, &mut Remainder),
//         (With<SpatialGrid>, With<PrimaryGlyphBufferMarker>),
//     >,
// ) {
//     let Ok(mut cursor) = q_cursor.get_single_mut() else {
//         return;
//     };
//     let Ok(mut grid) = q_grid.get_single_mut() else {
//         return;
//     };

//     for ev in evr_kbd.read() {
//         if ev.state.is_pressed() {
//             if let Some(offset) = match &ev.logical_key {
//                 Key::ArrowLeft => Some(Vec2::NEG_X),
//                 Key::ArrowRight => Some(Vec2::X),
//                 Key::ArrowDown => Some(Vec2::NEG_Y),
//                 Key::ArrowUp => Some(Vec2::Y),
//                 Key::Character(ch) => match ch.as_str() {
//                     "h" => Some(Vec2::NEG_X),
//                     "l" => Some(Vec2::X),
//                     "j" => Some(Vec2::NEG_Y),
//                     "k" => Some(Vec2::Y),
//                     _ => None,
//                 },
//                 _ => None,
//             } {
//                 cursor.offset(offset);
//                 grid.offset(offset);
//             }
//         }
//     }
// }
