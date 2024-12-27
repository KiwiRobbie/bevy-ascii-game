use ascii_ui::mouse::input::MouseInput;
use bevy::{
    core_pipeline::bloom::Bloom,
    input::mouse::{MouseMotion, MouseWheel},
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
use bevy_remote_inspector::RemoteInspectorPlugins;
use editor_panel::plugin::TilesetPanelPlugin;
use glyph_render::{
    atlas::{CharacterSet, FontAtlasPlugin},
    font::{font_load_system, CustomFont, CustomFontSource, FontSize},
    glyph_animation::GlyphAnimationPlugin,
    glyph_animation_graph::plugin::GlyphAnimationGraphPlugin,
    glyph_buffer::GlyphBuffer,
    glyph_render_plugin::GlyphRenderPlugin,
};
use grid_physics::plugin::PhysicsPlugin;
use layers::{EditorLayer, EditorLayerPlugin, EditorLayers, SelectedEditorLayer};
use spatial_grid::{
    depth::Depth,
    global_position::GlobalPosition,
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
    .add_systems(Startup, (setup, testing_setup))
    .add_systems(
        Update,
        (
            font_load_system,
            mouse_zoom_system,
            mouse_pan_system,
            testing_update,
        ),
    );
    app.add_plugins(RemoteInspectorPlugins);
    app.run();
}

fn setup(mut commands: Commands) {
    let layers_entity = commands.spawn(EditorLayers).id();

    commands
        .spawn((
            EditorLayer::new("foreground"),
            SelectedEditorLayer,
            GamePhysicsGridMarker,
            Depth(0.1),
        ))
        .set_parent(layers_entity);

    commands
        .spawn((
            EditorLayer::new("background"),
            GamePhysicsGridMarker,
            Depth(0.0),
        ))
        .set_parent(layers_entity);

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

#[derive(Debug, Deref, DerefMut)]
struct ZoomSizeLocal(f32);
impl Default for ZoomSizeLocal {
    fn default() -> Self {
        Self(11.)
    }
}

fn mouse_zoom_system(
    mut mouse_input: ResMut<MouseInput>,
    mut size: Local<ZoomSizeLocal>,
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
    **size *= factor;
    **size = size.clamp(2.0, 128.0);

    let window = window.get_single().unwrap();

    for (mut font_size, mut grid, mut buffer) in q_glyph_buffer.iter_mut() {
        let size = **size as u32;
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

#[derive(Debug, Component)]
pub struct TestingMarker;
fn testing_setup(mut commands: Commands, server: ResMut<AssetServer>) {
    commands.spawn((
        FontSize(14),
        CustomFont(server.load("FiraCode-Regular.ttf")),
        CharacterSet::default(),
        TestingMarker,
    ));
}

fn create_target_angle_offset(
    theta: f32,
    offset: Vec2,
    thickness: f32,
    padding: UVec2,
    size: UVec2,
    image_size: UVec2,
) -> Vec<f32> {
    let center = padding.as_vec2() + 0.5 * size.as_vec2() + offset;

    let mut image: Vec<f32> = vec![0.; image_size.x as usize * image_size.y as usize];

    for (i, pixel) in image.iter_mut().enumerate() {
        let x = i % image_size.x as usize;
        let y = i / image_size.x as usize;
        let pos = Vec2::new(x as f32, y as f32);
        let local = Vec2::from_angle(theta).rotate(pos - center);

        let frac = (thickness - local.x.abs()).clamp(0.0, 1.0);
        *pixel = frac;
    }

    image
}

fn create_target_endpoints(
    start: Vec2,
    end: Vec2,
    thickness: f32,
    padding: UVec2,
    size: UVec2,
    image_size: UVec2,
) -> Vec<f32> {
    dbg!(start, end);
    let pixel_start = start * size.as_vec2() + padding.as_vec2();
    let pixel_end = end * size.as_vec2() + padding.as_vec2();

    let mut image: Vec<f32> = vec![0.; image_size.x as usize * image_size.y as usize];

    for (i, pixel) in image.iter_mut().enumerate() {
        let x = i % image_size.x as usize;
        let y = i / image_size.x as usize;
        let pixel_pos = Vec2::new(x as f32, y as f32);

        let line_length = (pixel_end - pixel_start).length();
        let primary_axis = (pixel_end - pixel_start) / line_length;
        let pixel_local_pos = pixel_pos - pixel_start;
        let t = primary_axis.dot(pixel_local_pos).clamp(0., line_length);

        let orthogonal_component = pixel_local_pos - primary_axis * t;
        let distance = orthogonal_component.length();

        let frac = (thickness - distance).clamp(0.0, 1.0);
        *pixel = frac;
    }

    image
}

fn testing_update(
    q_testing: Query<(&CustomFont, &FontSize), With<TestingMarker>>,
    fonts: Res<Assets<CustomFontSource>>,
    time: Res<Time>,
    q_physics_grid: Query<
        (&SpatialGrid, &GlobalPosition, &GlobalTransform),
        With<PrimaryGlyphBufferMarker>,
    >,

    mut ev_mouse_wheel: EventReader<MouseWheel>,
    mouse_input: Res<MouseInput>,
    mut offset: Local<Vec2>,
    mut q_layer: Query<(&mut EditorLayer, &GlobalPosition), With<SelectedEditorLayer>>,
) {
    for ev in ev_mouse_wheel.read() {
        offset.y += ev.y;
    }

    let Ok((font, font_size)) = q_testing.get_single() else {
        return;
    };
    let Some(font_source) = fonts.get(font.id()) else {
        return;
    };

    let grid_size = UVec2::new(font_size.advance(), font_size.line_spacing());
    let padding_start = grid_size / 4;
    let padding_end = UVec2::new(grid_size.x / 4, 3 * grid_size.y / 4);
    let image_size = padding_start + grid_size + padding_end;
    let image_pixel_count = (image_size.x * image_size.y) as usize;

    let mut context = swash::scale::ScaleContext::new();
    let mut scaler = context
        .builder(font_source.as_ref())
        .hint(false)
        .size(font_size.0 as f32)
        .build();
    let mut render = swash::scale::Render::new(&[
        swash::scale::Source::Bitmap(swash::scale::StrikeWith::BestFit),
        swash::scale::Source::Outline,
    ]);
    render.format(swash::zeno::Format::Alpha);

    let mut rendered_characters = vec![];

    for character in "!\"'()*+,-./:;=IJLT_´`fijlrt{|}~".chars() {
        let glyph_id = font_source.as_ref().charmap().map(character);
        let Some(image) = render.render(&mut scaler, glyph_id) else {
            continue;
        };

        let mut new_image_data: Vec<f32> = vec![0.; image_pixel_count];
        for (i, value) in image.data.iter().enumerate() {
            let x = padding_start.x as i32
                + image.placement.left
                + (i as i32 % image.placement.width as i32);
            let y = padding_start.y as i32
                + (image.placement.top)
                + (i as i32 / image.placement.width as i32);
            if !(0 <= x && x < image_size.x as i32 && 0 <= y && y < image_size.y as i32) {
                dbg!(x, y, image_size);
                continue;
            }

            let dst_index = x as usize + y as usize * image_size.x as usize;
            new_image_data[dst_index] = *value as f32 / 255.;
        }

        rendered_characters.push((character, new_image_data));
    }

    let Some(world_cursor_position) = mouse_input.world_position() else {
        return;
    };
    let Ok((grid, buffer_position, transform)) = q_physics_grid.get_single() else {
        return;
    };

    let grid_cursor_position =
        ((transform.compute_matrix().inverse() * world_cursor_position.extend(1.0)).xy()
            / grid.step.as_vec2()
            + 0.5)
            .as_ivec2()
            + **buffer_position;
    let start = Vec2::new(16.0, 0.0);
    let end = grid_cursor_position.as_vec2();

    let (mut layer, layer_pos) = q_layer.get_single_mut().unwrap();
    layer.clear_tiles();
    for pos in dda_iter(start, end) {
        let local_start = start - pos.as_vec2();
        let local_end = end - pos.as_vec2();

        // let target = create_target_endpoints(
        //     local_start,
        //     local_end,
        //     1.5,
        //     padding_start,
        //     grid_size,
        //     image_size,
        // );
        // print_image(&target, image_size);
        // let character = find_best_character(&target, 'x');
        let _ = layer.write_character(pos, 'x');
    }
    let start = Vec2::new(0.0, 16.0);
    for pos in dda_iter(start, end) {
        let local_start = start - pos.as_vec2();
        let local_end = end - pos.as_vec2();

        // let target = create_target_endpoints(
        //     local_start,
        //     local_end,
        //     1.5,
        //     padding_start,
        //     grid_size,
        //     image_size,
        // );
        // print_image(&target, image_size);
        let _ = layer.write_character(pos, '#');
    }
}

fn find_best_character(target: &[f32], rendered_characters: &[(char, Vec<f32>)]) -> char {
    let mut differences: Vec<(f64, &char)> = rendered_characters
        .iter()
        .map(|(character, image)| (image_difference(&target, &image), character))
        .filter(|(diff, _)| diff.is_finite())
        .collect();

    differences.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
    return differences.first().map(|(_, c)| **c).unwrap_or('x');
}

enum DdaAxis {
    X,
    Y,
}
enum DdaDirection {
    Inc,
    Dec,
}

// Origin: smallest coordinate
// Axis:  primary axis
// Direction: Is secondary axis increasing or decreasing?
struct DdaItem {
    origin: IVec2,
    axis: DdaAxis,
    direction: DdaDirection,
}

fn dda_iter(start: Vec2, end: Vec2) -> impl Iterator<Item = IVec2> {
    let delta = end - start;

    if delta.abs().x > delta.abs().y {
        let (start, end) = match start.x > end.x {
            true => (end, start),
            false => (start, end),
        };
        let delta = end - start;

        let x_min = start.x.floor() as i32;
        let x_max = end.x.ceil() as i32;
        let m = delta.y / delta.x;
        itertools::Either::Left(
            (x_min..x_max).map(move |x| IVec2::new(x, (start.y + m * (x as f32 - start.x)) as i32)),
        )
    } else {
        let (start, end) = match start.y > end.y {
            true => (end, start),
            false => (start, end),
        };
        let delta = end - start;

        let y_min = start.y.floor() as i32;
        let y_max = end.y.ceil() as i32;
        let m = delta.x / delta.y;
        itertools::Either::Right(
            (y_min..y_max).map(move |y| IVec2::new((start.x + m * (y as f32 - start.y)) as i32, y)),
        )
    }
}

fn pixel_difference(target: f32, image: f32) -> f32 {
    let difference = target - image;
    if difference < 0. {
        difference * difference * 4.
    } else {
        difference * difference
    }
}

fn image_difference(target: &[f32], image: &[f32]) -> f64 {
    assert_eq!(target.len(), image.len());
    target
        .iter()
        .zip(image.iter())
        .map(|(&a, &b)| pixel_difference(a, b) as f64)
        .sum()
}

fn print_image(image: &[f32], size: UVec2) {
    const GRADIENT: &str = "  `''.--__,~\"\"^!!;+++|\\/?7lt*z14[ny3e5ZkES96AdwODDR%%0QN$$$@@@MW";
    let gradient_len = GRADIENT.chars().count();

    let (width, height) = (size.x, size.y);
    let mut row = String::with_capacity(2 * width as usize);
    for y in 0..height {
        for x in 0..width {
            let pixel = image[(x + y * width) as usize];
            let index = (pixel * (gradient_len as f32)).max(0.) as usize;
            let index = index.clamp(0, gradient_len - 1);
            let character = GRADIENT.chars().nth(index).unwrap();
            row.push(character);
            row.push(character);
        }
        println!("{}", row);
        row.clear();
    }
}
