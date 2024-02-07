use std::ops::Div;

use ascii_ui::mouse::TriggeredMarker;
use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{mouse::MouseButton, Input},
    math::{IVec2, UVec2, Vec4Swizzles},
    prelude::Deref,
    render::{camera::Camera, color::Color},
    transform::components::GlobalTransform,
    window::{PrimaryWindow, Window},
};
use bevy_ascii_game::{
    physics_grids::GamePhysicsGridMarker, tilemap::component::Tilemap,
    tileset::asset::TilesetSource,
};
use glyph_render::glyph_render_plugin::{GlyphSolidColor, GlyphSprite, GlyphTextureSource};
use spatial_grid::{
    grid::{PhysicsGridMember, SpatialGrid},
    position::{Position, PositionBundle},
};

use crate::tileset_panel::setup::TilesetTileId;

pub struct BrushPlugin;
impl Plugin for BrushPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (set_brush, update_brush));
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Brush,
        PositionBundle::default(),
        GamePhysicsGridMarker,
        GlyphSolidColor { color: Color::GRAY },
    ));
}

#[derive(Component, Deref)]
pub struct BrushTileSize(pub UVec2);

pub fn set_brush(
    q_select: Query<&TilesetTileId, With<TriggeredMarker>>,
    q_brush: Query<Entity, With<Brush>>,
    mut commands: Commands,

    tilesets: Res<Assets<TilesetSource>>,
    mut glyph_textures: ResMut<Assets<GlyphTextureSource>>,
) {
    let Ok(entity) = q_brush.get_single() else {
        return;
    };
    for tile in q_select.iter() {
        let tileset = tilesets.get(tile.tileset.id()).unwrap();
        let tile = &tileset.tiles[tile.tile];
        commands.entity(entity).insert((
            GlyphSprite {
                offset: IVec2::ZERO,
                texture: glyph_textures.add(GlyphTextureSource { data: tile.clone() }),
            },
            BrushTileSize(tileset.tile_size),
        ));
    }
}

pub fn update_brush(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_mouse_buttons: Res<Input<MouseButton>>,
    q_physics_grid: Query<(&SpatialGrid, &GlobalTransform)>,
    mut q_brush: Query<(&mut Position, &PhysicsGridMember, Option<&BrushTileSize>), With<Brush>>,
    q_tilemap: Query<(&Tilemap, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();
    if let Some(cursor_position) = q_windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin)
    {
        let Ok((mut brush_position, grid_member, tile_size)) = q_brush.get_single_mut() else {
            return;
        };
        let Ok((grid, transform)) = q_physics_grid.get(grid_member.grid) else {
            return;
        };

        let cursor_position = (transform.compute_matrix().inverse() * cursor_position.extend(1.0))
            .xy()
            / grid.size.as_vec2();
        let cursor_position = (cursor_position + 0.5).as_ivec2();
        **brush_position = cursor_position
            - tile_size
                .map(|s| s.div(2))
                .unwrap_or(UVec2::ZERO)
                .as_ivec2();
    }
}

#[derive(Component)]
pub struct Brush;
