use std::ops::Div;

use ascii_ui::mouse::{input::MouseInput, TriggeredMarker};
use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    color::palettes::css::GRAY,
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res, ResMut},
    },
    input::mouse::MouseButton,
    math::{IVec2, UVec2, Vec4Swizzles},
    prelude::Deref,
    transform::components::GlobalTransform,
};
use bevy_ascii_game::{
    physics_grids::GamePhysicsGridMarker,
    tilemap::{asset::TilemapSource, chunk::TilemapChunk, component::Tilemap},
    tileset::asset::TilesetSource,
};
use glyph_render::{
    glyph_render_plugin::{GlyphSolidColor, GlyphTexture},
    glyph_sprite::GlyphSprite,
};
use spatial_grid::{
    grid::{PhysicsGridMember, SpatialGrid},
    position::{Position, SpatialBundle},
};

use crate::tileset_panel::setup::TilesetTileId;

pub(crate) struct BrushPlugin;
impl Plugin for BrushPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (set_brush, update_brush));
    }
}

pub(crate) fn setup(mut commands: Commands) {
    commands.spawn((
        Brush,
        SpatialBundle::default(),
        GamePhysicsGridMarker,
        GlyphSolidColor { color: GRAY.into() },
    ));
}

#[derive(Component, Deref)]
pub(crate) struct BrushTileSize(pub(crate) UVec2);

pub(crate) fn set_brush(
    q_select: Query<&TilesetTileId, With<TriggeredMarker>>,
    q_brush: Query<Entity, With<Brush>>,
    mut commands: Commands,

    tilesets: Res<Assets<TilesetSource>>,
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
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
                texture: glyph_textures.add(GlyphTexture {
                    source: tile.clone(),
                }),
            },
            BrushTileSize(tileset.tile_size),
            tile_id.clone(),
        ));
    }
}

pub(crate) fn update_brush(
    mut commands: Commands,
    q_physics_grid: Query<(&SpatialGrid, &Position, &GlobalTransform)>,
    mut q_brush: Query<
        (
            Entity,
            &PhysicsGridMember,
            Option<&BrushTileSize>,
            Option<&TilesetTileId>,
        ),
        With<Brush>,
    >,
    q_tilemap: Query<(&Tilemap, &Position), Without<Brush>>,
    mut tilemaps: ResMut<Assets<TilemapSource>>,
    mut chunks: ResMut<Assets<TilemapChunk>>,
    tilesets: Res<Assets<TilesetSource>>,
    mouse_input: Res<MouseInput>,
) {
    let Ok((entity, grid_member, tile_size, brush_tile)) = q_brush.get_single_mut() else {
        return;
    };

    if let Some(world_cursor_position) = mouse_input.world_position() {
        let Ok((grid, buffer_position, transform)) = q_physics_grid.get(grid_member.grid) else {
            return;
        };

        let grid_cursor_position =
            ((transform.compute_matrix().inverse() * world_cursor_position.extend(1.0)).xy()
                / grid.size.as_vec2()
                + 0.5)
                .as_ivec2()
                + **buffer_position;

        let mut cursor_position = grid_cursor_position
            - tile_size
                .map(|s| s.div(2))
                .unwrap_or(UVec2::ZERO)
                .as_ivec2()
            + **buffer_position;

        for (tilemap, tilemap_position) in q_tilemap.iter() {
            let Some(tilemap_source) = tilemaps.get(tilemap.id()) else {
                continue;
            };

            let tile_size = tilemap_source.tile_size.as_ivec2();
            let tilemap_local = (grid_cursor_position - **tilemap_position).div_euclid(tile_size);
            cursor_position = tilemap_local * tile_size + **tilemap_position - **buffer_position;

            if mouse_input.pressed(MouseButton::Left) {
                if let Some(tile) = brush_tile {
                    if let (Some(tilemap), Some(tileset)) = (
                        tilemaps.get_mut(tilemap.id()),
                        tilesets.get(tile.tileset.id()),
                    ) {
                        tilemap.insert_tile(
                            &mut chunks,
                            tilemap_local,
                            tileset.id.clone(),
                            tile.tile,
                            tile.tileset.clone(),
                        );
                    }
                }
            } else if mouse_input.pressed(MouseButton::Right) {
                if let Some(tilemap) = tilemaps.get_mut(tilemap.id()) {
                    tilemap.clear_tile(&mut chunks, tilemap_local);
                }
            }

            break;
        }
        commands.entity(entity).insert(Position(cursor_position));
    } else if let Ok(entity) = q_brush.get_single().map(|brush| brush.0) {
        commands.entity(entity).remove::<Position>();
    }
}

#[derive(Component)]
pub(crate) struct Brush;
