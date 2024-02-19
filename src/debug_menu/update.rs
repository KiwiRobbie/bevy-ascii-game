use super::{setup::DebugMenuMarker, state::DebugMenuState};
use crate::physics_grids::UiPhysicsGrid;
use ascii_ui::attachments::Root;
use bevy::{
    ecs::{
        query::With,
        system::{Query, Res, ResMut},
    },
    input::{
        gamepad::{GamepadButton, GamepadButtonType, Gamepads},
        keyboard::KeyCode,
        ButtonInput,
    },
};
use glyph_render::glyph_buffer::GlyphBuffer;
use spatial_grid::grid::SpatialGrid;

pub fn toggle_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<DebugMenuState>,
    gamepad_button: Res<ButtonInput<GamepadButton>>,
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
