use ascii_ui::{
    attachments::Root,
    widgets::{self, button::ButtonJustPressedMarker},
};
use bevy::{
    asset::{AssetEvent, Assets, Handle},
    ecs::{
        component::Component,
        event::EventReader,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{
        gamepad::{GamepadButton, GamepadButtonType, Gamepads},
        keyboard::KeyCode,
        Input,
    },
};
use glyph_render::glyph_buffer::GlyphBuffer;

use spatial_grid::grid::SpatialGrid;

use bevy_ascii_game::{physics_grids::UiPhysicsGrid, tileset::asset::TilesetSource};

use crate::list_builder_widget::ListBuilderWidget;

use super::{
    setup::{DebugMenuMarker, ItemMutateButton},
    state::TilesetPanelState,
};

pub fn toggle_menu(
    keyboard: Res<Input<KeyCode>>,
    mut state: ResMut<TilesetPanelState>,
    gamepad_button: Res<Input<GamepadButton>>,
    gamepads: Res<Gamepads>,
    mut q_root: Query<&mut Root>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        state.enabled = !state.enabled;
    }
    for gamepad in gamepads.iter() {
        if gamepad_button.just_pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Select,
        }) {
            state.enabled = !state.enabled;
        }
    }

    if let Some(root) = state.root_widget {
        let mut root = q_root.get_mut(root).unwrap();
        root.enabled = state.enabled;
    }
}

pub fn update_position(
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

pub fn update_list_builder(
    mut commands: Commands,
    mut q_list_builder: Query<(&mut ListBuilderWidget<usize>, &mut widgets::Column)>,
    q_buttons: Query<&ItemMutateButton, (With<ButtonJustPressedMarker>, With<widgets::Button>)>,
) {
    for item in q_buttons.iter() {
        let (mut builder, mut column) = q_list_builder.get_mut(item.target).unwrap();
        match item.mode {
            super::setup::MutateMode::Add => {
                builder.push(&mut *column, 0, &mut commands);
            }
            super::setup::MutateMode::Remove => {
                builder.pop(&mut *column, &mut commands);
            }
        }
    }
}
#[derive(Debug, Component)]
pub struct TilesetHandles {
    pub handles: Vec<Handle<TilesetSource>>,
}

pub fn update_tilesets(
    mut commands: Commands,
    mut q_list_builder: Query<(
        &mut ListBuilderWidget<(TilesetSource, Handle<TilesetSource>)>,
        &mut widgets::Column,
        &TilesetHandles,
    )>,
    mut ev_tilesets: EventReader<AssetEvent<TilesetSource>>,
    tilesets: Res<Assets<TilesetSource>>,
) {
    for ev in ev_tilesets.read() {
        if let AssetEvent::LoadedWithDependencies { id } = ev {
            let tileset = tilesets.get(*id).unwrap().clone();
            for (mut builder, mut column, TilesetHandles { handles }) in q_list_builder.iter_mut() {
                if let Some(handle) = handles.iter().find(|handle| &handle.id() == id) {
                    builder.push::<widgets::Column>(
                        &mut column,
                        (tileset.clone(), handle.clone()),
                        &mut commands,
                    )
                }
            }
        };
    }
}
