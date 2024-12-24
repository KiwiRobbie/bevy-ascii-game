use ascii_ui::{
    attachments::Root,
    widgets::{self, SingleChildWidget},
};
use bevy::prelude::*;

use glyph_render::glyph_buffer::GlyphBuffer;

use spatial_grid::grid::SpatialGrid;

use bevy_ascii_game::{physics_grids::UiPhysicsGrid, tileset::asset::TilesetSource};

use crate::tools::FocusedToolUi;

use super::{setup::DebugMenuMarker, state::EditorPanelState};

pub(super) fn toggle_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<EditorPanelState>,
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

#[derive(Debug, Component)]
pub(crate) struct TilesetHandles {
    pub(crate) handles: Vec<Handle<TilesetSource>>,
}

pub(super) fn update_tilesets_system(
    mut commands: Commands,
    mut q_list_builder: Query<(
        &mut widgets::ListBuilderWidget<(TilesetSource, Handle<TilesetSource>)>,
        &mut widgets::FlexWidget,
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
                    builder.push::<widgets::FlexWidget>(
                        &mut column,
                        (tileset.clone(), handle.clone()),
                        &mut commands,
                    )
                }
            }
        };
    }
}

pub(super) fn update_editor_ui(
    q_editor_state: Res<EditorPanelState>,
    q_focused: Query<Entity, With<FocusedToolUi>>,
    mut q_container: Query<&mut SingleChildWidget>,
) {
    if let Ok(focused_tool_entity) = q_focused.get_single() {
        if let Some(container_entity) = q_editor_state.tool_container {
            if let Ok(mut container) = q_container.get_mut(container_entity) {
                container.child = Some(focused_tool_entity);
            }
        }
    };
}
