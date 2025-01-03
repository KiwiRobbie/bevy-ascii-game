use bevy::{ecs::entity::EntityHashSet, prelude::*, render::sync_world::SyncToRenderWorld};

use self::resize::grid_resize_update;
use crate::player::PlayerMarker;
use glyph_render::{
    atlas::{CharacterSet, FontAtlasUser},
    font::{CustomFont, FontSize},
    glyph_buffer::{GlyphBuffer, TargetGlyphBuffer},
};
use grid_physics::{collision::Collider, plugin::PhysicsUpdateSet, velocity::Velocity};
use parallax::parallax_system;
use spatial_grid::{
    grid::{PhysicsGridMember, SpatialGrid},
    position::{Position, SpatialBundle},
};

pub mod parallax;
pub mod resize;

#[derive(Component)]
pub struct GamePhysicsGridMarker;
#[derive(Component)]
pub struct UiPhysicsGridMarker;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct GamePhysicsGrid(pub Option<Entity>);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct UiPhysicsGrid(pub Option<Entity>);

fn apply_physics_grid_markers(
    mut commands: Commands,
    game_grid: Res<GamePhysicsGrid>,
    ui_grid: Res<UiPhysicsGrid>,
    q_game: Query<Entity, With<GamePhysicsGridMarker>>,
    q_ui: Query<Entity, With<UiPhysicsGridMarker>>,
) {
    let (Some(game_grid), Some(ui_grid)) = (**game_grid, **ui_grid) else {
        return;
    };
    for entity in q_game.iter() {
        commands
            .entity(entity)
            .remove::<GamePhysicsGridMarker>()
            .insert(TargetGlyphBuffer(game_grid))
            .insert(PhysicsGridMember { grid: game_grid });
    }
    for entity in q_ui.iter() {
        commands
            .entity(entity)
            .remove::<UiPhysicsGridMarker>()
            .insert(TargetGlyphBuffer(ui_grid))
            .insert(PhysicsGridMember { grid: ui_grid });
    }
}

fn create_physics_grids(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut game_grid: ResMut<GamePhysicsGrid>,
    mut ui_grid: ResMut<UiPhysicsGrid>,
) {
    **game_grid = Some(
        commands
            .spawn((
                PrimaryGlyphBufferMarker,
                GlyphBuffer {
                    textures: EntityHashSet::default(),
                    size: UVec2 { x: 100, y: 60 },
                },
                Transform::default(),
                GlobalTransform::default(),
                CustomFont(server.load("FiraCode-Regular.ttf")),
                CharacterSet(CHARSET.chars().collect()),
                FontSize(32),
                SpatialBundle::from(IVec2::ZERO),
                SpatialGrid {
                    step: UVec2::new(19, 40),
                },
                FontAtlasUser,
                SyncToRenderWorld,
            ))
            .id(),
    );
    **ui_grid = Some(
        commands
            .spawn((
                GlyphBuffer {
                    textures: EntityHashSet::default(),
                    size: UVec2 { x: 32, y: 40 },
                },
                Transform::default(),
                GlobalTransform::default(),
                CustomFont(server.load("FiraCode-Regular.ttf")),
                CharacterSet(CHARSET.chars().collect()),
                FontSize(32),
                SpatialGrid {
                    step: UVec2 { x: 19, y: 40 },
                },
                SpatialBundle::from(IVec2::ZERO),
                FontAtlasUser,
                SyncToRenderWorld,
            ))
            .id(),
    );
}

pub(crate) fn grid_translate(
    q_player: Query<
        (&Position, &Velocity, &Collider),
        (With<PlayerMarker>, Without<PrimaryGlyphBufferMarker>),
    >,
    mut q_primary_buffer: Query<
        (&mut Position, &GlyphBuffer),
        (Without<PlayerMarker>, With<PrimaryGlyphBufferMarker>),
    >,
    mut prediction_offset_current: Local<IVec2>,
    mut prediction_offset_target: Local<Vec2>,
    time: Res<Time>,
) {
    let Ok((mut position, &GlyphBuffer { textures: _, size })) = q_primary_buffer.get_single_mut()
    else {
        return;
    };
    let Ok((player_pos, player_velocity, collider)) = q_player.get_single() else {
        return;
    };
    let Some(aabb) = collider.aabb() else { return };

    let x_padding = size.x as i32 / 4;
    let y_target = size.y as i32 / 3;

    let clamped_velocity =
        (1.0 * **player_velocity).abs().min(0.5 * size.as_vec2()) * player_velocity.signum();

    *prediction_offset_target = damp(
        *prediction_offset_target,
        clamped_velocity,
        2.0,
        4.0,
        time.delta_secs(),
    );

    let difference = *prediction_offset_target - prediction_offset_current.as_vec2();
    if difference.x.abs() >= 1.0 {
        prediction_offset_current.x += difference.x.round() as i32;
    }
    if difference.y.abs() >= 1.0 {
        prediction_offset_current.y += difference.y.round() as i32;
    }

    let predicted_position = **player_pos - **position + *prediction_offset_current;
    let predicted_center = predicted_position + aabb.start + aabb.size.as_ivec2();

    let lower_screen_pos = predicted_position;
    let upper_screen_pos = size.as_ivec2() - lower_screen_pos - aabb.size.as_ivec2();

    let delta = IVec2::new(
        (lower_screen_pos.x - x_padding).min(0) - (upper_screen_pos.x - x_padding).min(0),
        predicted_center.y - y_target,
    );
    **position += delta;
}

fn damp(a: Vec2, b: Vec2, lambda: f32, base_speed: f32, dt: f32) -> Vec2 {
    let distance = a.distance(b);
    let step = dt * base_speed / distance;
    let t = (1.0 - (-lambda * dt).exp()).max(step);
    Vec2::lerp(a, b, t.clamp(0., 1.))
}

pub struct PhysicsGridPlugin;
impl Plugin for PhysicsGridPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreUpdate, apply_physics_grid_markers)
            .add_systems(Update, grid_resize_update)
            .add_systems(
                PostUpdate,
                (grid_translate, parallax_system)
                    .chain()
                    .after(PhysicsUpdateSet::PostUpdate)
                    .before(TransformSystem::TransformPropagate),
            )
            .add_systems(Startup, create_physics_grids)
            .init_resource::<GamePhysicsGrid>()
            .init_resource::<UiPhysicsGrid>();
    }
}

const CHARSET: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`´abcdefghijklmnopqrstuvwxyz{|}~┌┐┘└─│";
#[derive(Debug, Component)]
pub struct PrimaryGlyphBufferMarker;
