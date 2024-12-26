use std::{ffi::OsStr, path::Path};

use ascii_ui::{
    attachments::Root,
    widgets::{self, button::ButtonJustPressedMarker, MultiChildWidget},
};
use bevy::{
    asset::{
        io::AssetSourceId,
        saver::{AssetSaver, SavedAsset},
        AssetEvent, AssetServer, Assets, ErasedLoadedAsset, Handle, LoadedAsset,
    },
    ecs::{
        component::Component,
        event::EventReader,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{gamepad::GamepadButton, keyboard::KeyCode, ButtonInput},
    prelude::{Children, Entity, Gamepad},
    tasks::IoTaskPool,
};
use glyph_render::glyph_buffer::GlyphBuffer;

use spatial_grid::grid::SpatialGrid;

use bevy_ascii_game::{
    physics_grids::UiPhysicsGrid,
    tilemap::{
        asset::TilemapSource,
        chunk::TilemapChunk,
        component::Tilemap,
        loader::ChunkSettings,
        saver::{ChunkSaver, TilemapSaver},
    },
    tileset::asset::TilesetSource,
};

use crate::list_builder_widget::ListBuilderWidget;

use super::{
    setup::{DebugMenuMarker, ItemMutateButton, SaveTilemapButton},
    state::TilesetPanelState,
};

pub(super) fn toggle_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<TilesetPanelState>,
    gamepads: Query<&Gamepad>,
    mut q_root: Query<&mut Root>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        state.enabled = !state.enabled;
    }
    for gamepad in gamepads.iter() {
        if gamepad.just_pressed(GamepadButton::Select) {
            state.enabled = !state.enabled;
        }
    }

    if let Some(root) = state.root_widget {
        let mut root = q_root.get_mut(root).unwrap();
        root.enabled = state.enabled;
    }
}

pub(super) fn update_position(
    mut q_root: Query<&mut Root, With<DebugMenuMarker>>,
    ui_grid: Res<UiPhysicsGrid>,
    q_ui_grid: Query<(&SpatialGrid, &GlyphBuffer)>,
) {
    let Some(grid) = **ui_grid else {
        return;
    };
    let Ok((_grid, buffer)) = q_ui_grid.get(grid) else {
        return;
    };

    for mut root in q_root.iter_mut() {
        root.position.y = -(buffer.size.y as i32);
        root.position.x = buffer.size.x as i32 - root.size.x as i32;
    }
}

pub(super) fn update_list_builder(
    mut commands: Commands,
    mut q_list_builder: Query<
        (&mut ListBuilderWidget<usize>, &Children),
        With<widgets::MultiChildWidget>,
    >,
    q_buttons: Query<&ItemMutateButton, (With<ButtonJustPressedMarker>, With<widgets::Button>)>,
) {
    for item in q_buttons.iter() {
        let (mut builder, children) = q_list_builder.get_mut(item.target).unwrap();
        match item.mode {
            super::setup::MutateMode::Add => {
                builder.push(item.target, 0, &mut commands);
            }
            super::setup::MutateMode::Remove => {
                builder.pop(children, &mut commands);
            }
        }
    }
}
#[derive(Debug, Component)]
pub(crate) struct TilesetHandles {
    pub(crate) handles: Vec<Handle<TilesetSource>>,
}

pub(super) fn update_tilesets_system(
    mut commands: Commands,
    mut q_list_builder: Query<
        (
            Entity,
            &mut ListBuilderWidget<(TilesetSource, Handle<TilesetSource>)>,
            &TilesetHandles,
        ),
        With<MultiChildWidget>,
    >,
    mut ev_tilesets: EventReader<AssetEvent<TilesetSource>>,
    tilesets: Res<Assets<TilesetSource>>,
) {
    for ev in ev_tilesets.read() {
        if let AssetEvent::LoadedWithDependencies { id } = ev {
            let tileset = tilesets.get(*id).unwrap().clone();
            for (list_widget_entity, mut builder, TilesetHandles { handles }) in
                q_list_builder.iter_mut()
            {
                if let Some(handle) = handles.iter().find(|handle| &handle.id() == id) {
                    builder.push(
                        list_widget_entity,
                        (tileset.clone(), handle.clone()),
                        &mut commands,
                    )
                }
            }
        };
    }
}

pub(super) fn save_tilemap_system(
    q_buttons: Query<
        (),
        (
            With<SaveTilemapButton>,
            With<ButtonJustPressedMarker>,
            With<widgets::Button>,
        ),
    >,
    q_tilemap: Query<&Tilemap>,
    tilemaps: Res<Assets<TilemapSource>>,
    chunks: Res<Assets<TilemapChunk>>,
    server: Res<AssetServer>,
) {
    if q_buttons.iter().next().is_none() {
        return;
    }
    let Some(tilemap) = q_tilemap
        .get_single()
        .ok()
        .and_then(|h| tilemaps.get(h.id()).cloned())
    else {
        return;
    };

    let server = server.clone();

    for (chunk_id, chunk) in tilemap.chunk_handles.iter() {
        let (chunk_id, chunk) = (*chunk_id, chunks.get(chunk.id()).unwrap().clone());
        let loaded: LoadedAsset<_> = chunk.into();
        let erased: ErasedLoadedAsset = loaded.into();
        let server = server.clone();
        IoTaskPool::get()
            .spawn(async move {
                let asset_source = server.get_source(AssetSourceId::default()).unwrap();
                let chunk_label =
                    format!("tilemaps/output/{}_{}.chunk.bin", chunk_id.x, chunk_id.y);

                let output = asset_source
                    .writer()
                    .unwrap()
                    .write(Path::new(OsStr::new(&chunk_label)));
                let mut output = output.await.unwrap();

                ChunkSaver
                    .save(
                        &mut output,
                        SavedAsset::from_loaded(&erased).unwrap(),
                        &ChunkSettings::default(),
                    )
                    .await
                    .unwrap();
            })
            .detach();
    }

    IoTaskPool::get()
        .spawn(async move {
            let output = server
                .get_source(AssetSourceId::default())
                .unwrap()
                .writer()
                .unwrap()
                .write(Path::new("tilemaps/output.tilemap.ron"));
            let loaded: LoadedAsset<_> = tilemap.clone().into();
            let erased: ErasedLoadedAsset = loaded.into();

            let mut output = output.await.unwrap();

            dbg!(
                TilemapSaver
                    .save(
                        &mut output,
                        SavedAsset::from_loaded(&erased).unwrap(),
                        &Default::default(),
                    )
                    .await
            )
            .unwrap();
        })
        .detach();
}
