use bevy::prelude::*;

use super::{
    setup::{setup_ui, DebugMenuMarker},
    state::DebugMenuState,
};
use crate::physics_grids::UiPhysicsGrid;
use ascii_ui::attachments::Root;
use glyph_render::glyph_buffer::GlyphBuffer;
use spatial_grid::grid::SpatialGrid;

pub fn toggle_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<DebugMenuState>,
    mut commands: Commands,
    gamepads: Query<&Gamepad>,
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
            setup_ui(&mut commands, &mut state);
        }
        _ => {}
    };
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
        root.position.x = buffer.size.x as i32 - root.size.x as i32;
        root.position.y = buffer.size.y as i32 - root.size.y as i32;
    }
}
