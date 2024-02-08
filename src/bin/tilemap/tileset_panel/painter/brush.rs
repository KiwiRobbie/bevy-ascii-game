use std::ops::Div;

use ascii_ui::mouse::TriggeredMarker;
use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
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
    physics_grids::GamePhysicsGridMarker,
    tilemap::{asset::TilemapSource, component::Tilemap},
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
    for tile_id in q_select.iter() {
        let tileset = tilesets.get(tile_id.tileset.id()).unwrap();
        let tile = &tileset.tiles[tile_id.tile as usize];
        commands.entity(entity).insert((
            GlyphSprite {
                offset: IVec2::ZERO,
                texture: glyph_textures.add(GlyphTextureSource { data: tile.clone() }),
            },
            BrushTileSize(tileset.tile_size),
            tile_id.clone(),
        ));
    }
}

pub fn update_brush(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    input_mouse_buttons: Res<Input<MouseButton>>,
    q_physics_grid: Query<(&SpatialGrid, &GlobalTransform)>,
    mut q_brush: Query<
        (
            &mut Position,
            &PhysicsGridMember,
            Option<&BrushTileSize>,
            Option<&TilesetTileId>,
        ),
        With<Brush>,
    >,
    q_tilemap: Query<(&Tilemap, &Position), Without<Brush>>,
    mut tilemaps: ResMut<Assets<TilemapSource>>,
    tilesets: Res<Assets<TilesetSource>>,
) {
    let (camera, camera_transform) = q_camera.single();
    if let Some(world_cursor_position) = q_windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin)
    {
        let Ok((mut brush_position, grid_member, tile_size, brush_tile)) = q_brush.get_single_mut()
        else {
            return;
        };
        let Ok((grid, transform)) = q_physics_grid.get(grid_member.grid) else {
            return;
        };

        let grid_cursor_position =
            ((transform.compute_matrix().inverse() * world_cursor_position.extend(1.0)).xy()
                / grid.size.as_vec2()
                + 0.5)
                .as_ivec2();

        **brush_position = if let Some((target_tilemap, tilemap_local, cursor_position)) = q_tilemap
            .iter()
            .filter_map(|(tilemap, tilemap_position)| {
                if let Some(tilemap_source) = tilemaps.get(tilemap.id()) {
                    let tile_size = tilemap_source.tile_size.as_ivec2();

                    let local = (grid_cursor_position - **tilemap_position).div_euclid(tile_size);
                    Some((
                        (*tilemap).clone(),
                        local,
                        local * tile_size + **tilemap_position,
                    ))
                } else {
                    None
                }
            })
            .next()
        {
            if input_mouse_buttons.pressed(MouseButton::Left) {
                if let Some(tile) = brush_tile {
                    if let (Some(tilemap), Some(tileset)) = (
                        tilemaps.get_mut(target_tilemap.id()),
                        tilesets.get(tile.tileset.id()),
                    ) {
                        tilemap.insert_tile(
                            tilemap_local,
                            tileset.id.clone(),
                            tile.tile,
                            tile.tileset.clone(),
                        );
                    }
                }
            } else if input_mouse_buttons.pressed(MouseButton::Right) {
                if let Some(tilemap) = tilemaps.get_mut(target_tilemap.id()) {
                    tilemap.clear_tile(tilemap_local);
                }
            }

            cursor_position
        } else {
            grid_cursor_position
                - tile_size
                    .map(|s| s.div(2))
                    .unwrap_or(UVec2::ZERO)
                    .as_ivec2()
        };
    }
}

#[derive(Component)]
pub struct Brush;