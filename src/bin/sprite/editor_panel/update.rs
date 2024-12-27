use ascii_ui::{attachments::Root, widgets::SingleChildWidget};
use bevy::prelude::*;

use glyph_render::{glyph_buffer::GlyphBuffer, glyph_render_plugin::GlyphTexture};

use spatial_grid::grid::SpatialGrid;

use bevy_ascii_game::physics_grids::UiPhysicsGrid;

use crate::tools::{
    text::spawn_type_tool, ExclusiveKeyboardEventHandler, FocusedTool, ToolUiEntity,
};

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
        root.position.x = buffer.size.x as i32 - root.size.x as i32;
        root.position.y = buffer.size.y as i32 - root.size.y as i32;
    }
}

pub(super) fn update_editor_ui(
    q_editor_state: Res<EditorPanelState>,
    q_focused: Query<&ToolUiEntity, (With<FocusedTool>, With<SingleChildWidget>)>,
    mut commands: Commands,
) {
    let focused_tool_ui = q_focused.get_single().ok();

    if let Some(container_entity) = q_editor_state.tool_container {
        let mut entity_commands = commands.entity(container_entity);
        entity_commands.despawn_descendants();
        if let Some(ToolUiEntity(entity)) = focused_tool_ui {
            entity_commands.add_child(*entity);
        }
    }
}

pub(super) fn update_editor_shortcuts(
    mut commands: Commands,
    q_focused: Query<Entity, With<FocusedTool>>,
    q_using_keyboard: Query<(), With<ExclusiveKeyboardEventHandler>>,
    input_keys: Res<ButtonInput<KeyCode>>,
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
) {
    let clear_focused = |commands: &mut Commands| {
        for entity in &q_focused {
            commands.entity(entity).despawn_recursive();
        }
    };

    if q_using_keyboard.iter().len() > 0 {
        return;
    }
    if input_keys.just_pressed(KeyCode::KeyT) {
        clear_focused(&mut commands);
        spawn_type_tool(&mut commands, &mut glyph_textures)
    }
}
