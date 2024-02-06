use bevy::{
    app::{Plugin, PreUpdate, Startup, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    math::{IVec2, UVec2},
    prelude::{Deref, DerefMut},
    transform::components::{GlobalTransform, Transform},
    utils::HashSet,
};
use glyph_render::{
    atlas::{CharacterSet, FontAtlasUser},
    font::{CustomFont, FontSize},
    glyph_buffer::{GlyphBuffer, TargetGlyphBuffer},
};

use spatial_grid::{
    grid::{PhysicsGridMember, SpatialGrid},
    position::PositionBundle,
};

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

pub fn apply_physics_grid_markers(
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

pub fn create_physics_grids(
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
                    textures: HashSet::new(),
                    size: UVec2 { x: 100, y: 60 },
                },
                Transform::default(),
                GlobalTransform::default(),
                CustomFont(server.load("FiraCode-Regular.ttf")),
                CharacterSet(CHARSET.chars().collect()),
                FontSize(32),
                PositionBundle::from(IVec2::ZERO),
                SpatialGrid {
                    size: UVec2 { x: 19, y: 40 },
                },
                FontAtlasUser,
            ))
            .id(),
    );
    **ui_grid = Some(
        commands
            .spawn((
                GlyphBuffer {
                    textures: HashSet::new(),
                    size: UVec2 { x: 32, y: 40 },
                },
                Transform::default(),
                GlobalTransform::default(),
                CustomFont(server.load("FiraCode-Regular.ttf")),
                CharacterSet(CHARSET.chars().collect()),
                FontSize(32),
                SpatialGrid {
                    size: UVec2 { x: 19, y: 40 },
                },
                PositionBundle::from(IVec2::ZERO),
                FontAtlasUser,
            ))
            .id(),
    );
}

pub struct PhysicsGridPlugin;
impl Plugin for PhysicsGridPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreUpdate, apply_physics_grid_markers)
            .add_systems(Update, grid_resize_update)
            .add_systems(Startup, create_physics_grids)
            .init_resource::<GamePhysicsGrid>()
            .init_resource::<UiPhysicsGrid>();
    }
}
const CHARSET: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
#[derive(Debug, Component)]
pub struct PrimaryGlyphBufferMarker;
