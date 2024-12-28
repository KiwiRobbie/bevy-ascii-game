use ascii_ui::attachments::Root;
use bevy::prelude::*;

use glyph_render::{glyph_buffer::GlyphBuffer, glyph_render_plugin::GlyphTexture};

use spatial_grid::grid::SpatialGrid;

use bevy_ascii_game::physics_grids::UiPhysicsGrid;

use crate::{
    layers::{EditorLayer, SelectedEditorLayer},
    tools::{
        text::spawn_type_tool, translate::spawn_translate_tool, BuildToolUi,
        ExclusiveKeyboardEventHandler, FocusedTool,
    },
};

use super::{
    setup::{setup_ui, DebugMenuMarker},
    state::EditorPanelState,
};

pub(super) fn toggle_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<EditorPanelState>,
    gamepads: Query<&Gamepad>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        state.enabled = !state.enabled;
    }
    for gamepad in gamepads.iter() {
        if gamepad.just_pressed(GamepadButton::Select) {
            state.enabled = !state.enabled;
        }
    }

    match state.root_widget {
        Some(root) if !state.enabled => {
            state.root_widget.take();
            commands.entity(root).despawn_recursive();
        }
        None if state.enabled => {
            state.root_widget = Some(setup_ui(&mut commands));
        }
        _ => {}
    };
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
#[derive(Debug, Component)]
pub struct ToolUiContainer;

pub(super) fn update_editor_ui(
    q_tool_container: Query<(Entity, Option<&Children>), With<ToolUiContainer>>,
    q_build_tool: Query<&BuildToolUi, With<FocusedTool>>,
    mut commands: Commands,
) {
    if let Some((container_entity, children)) = q_tool_container.get_single().ok() {
        if let Some(BuildToolUi(builder)) = q_build_tool.get_single().ok() {
            if children.map(|children| children.is_empty()).unwrap_or(true) {
                let entity = builder(&mut commands);
                commands.entity(container_entity).add_child(entity);
            }
        } else {
            commands.entity(container_entity).despawn_descendants();
        }
    }
}
pub(super) fn isolate_layers_update(
    state: Res<EditorPanelState>,
    mut q_layers: Query<(&mut EditorLayer, Has<SelectedEditorLayer>)>,
) {
    for (mut layer, is_selected) in &mut q_layers {
        layer.visible = layer.enabled & (!state.isolate_selected || is_selected);
    }
}
pub(super) fn update_editor_shortcuts(
    mut commands: Commands,
    q_focused: Query<Entity, With<FocusedTool>>,
    q_using_keyboard: Query<(), With<ExclusiveKeyboardEventHandler>>,
    input_keys: Res<ButtonInput<KeyCode>>,
    mut glyph_textures: ResMut<Assets<GlyphTexture>>,
    mut state: ResMut<EditorPanelState>,
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
    if input_keys.just_pressed(KeyCode::KeyG) {
        clear_focused(&mut commands);
        spawn_translate_tool(&mut commands)
    }
    if input_keys.just_pressed(KeyCode::Slash) {
        state.isolate_selected = !state.isolate_selected;
    }
}
