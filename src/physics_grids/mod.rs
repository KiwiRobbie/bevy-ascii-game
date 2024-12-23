use bevy::{
    app::{Plugin, PostUpdate, PreUpdate, Startup, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::{Entity, EntityHashSet},
        query::With,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    math::{IVec2, UVec2, Vec2},
    prelude::{Deref, DerefMut, IntoSystemConfigs, Local, TransformSystem, Without},
    render::sync_world::SyncToRenderWorld,
    time::Time,
    transform::components::{GlobalTransform, Transform},
};
use glyph_render::{
    atlas::{CharacterSet, FontAtlasUser},
    font::{CustomFont, FontSize},
    glyph_buffer::{GlyphBuffer, TargetGlyphBuffer},
};

use grid_physics::{collision::Collider, plugin::PhysicsUpdateSet, velocity::Velocity};
use spatial_grid::{
    grid::{PhysicsGridMember, SpatialGrid},
    position::{Position, SpatialBundle},
};

use crate::player::PlayerMarker;

use self::resize::grid_resize_update;

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
    mut prediction_offset: Local<Vec2>,
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

    // let padding = IVec2::new(size.x as i32 / 4, size.y as i32 / 2);

    let x_padding = size.x as i32 / 4;
    let y_target = size.y as i32 / 3;

    let clamped_velocity =
        (1.0 * **player_velocity).abs().min(0.5 * size.as_vec2()) * player_velocity.signum();

    *prediction_offset = damp(*prediction_offset, clamped_velocity, 2.0, time.delta_secs());

    let predicted_position = **player_pos + prediction_offset.as_ivec2() - **position;
    let predicted_center = predicted_position + aabb.start + aabb.size.as_ivec2() / 2;

    let lower_screen_pos = predicted_position;
    let upper_screen_pos = size.as_ivec2() - lower_screen_pos - aabb.size.as_ivec2();

    let y_delta = predicted_center.y - y_target;
    let x_delta = (lower_screen_pos.x - x_padding).min(0) - (upper_screen_pos.x - x_padding).min(0);

    **position += IVec2::new(x_delta, y_delta);
}

fn damp(a: Vec2, b: Vec2, lambda: f32, dt: f32) -> Vec2 {
    Vec2::lerp(a, b, 1.0 - (-lambda * dt).exp())
}

pub struct PhysicsGridPlugin;
impl Plugin for PhysicsGridPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreUpdate, apply_physics_grid_markers)
            .add_systems(Update, grid_resize_update)
            .add_systems(
                PostUpdate,
                grid_translate
                    .after(PhysicsUpdateSet::PostUpdate)
                    .before(TransformSystem::TransformPropagate),
            )
            .add_systems(Startup, create_physics_grids)
            .init_resource::<GamePhysicsGrid>()
            .init_resource::<UiPhysicsGrid>();
    }
}
const CHARSET: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
#[derive(Debug, Component)]
pub struct PrimaryGlyphBufferMarker;
