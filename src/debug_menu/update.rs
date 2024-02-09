use ascii_ui::{
    attachments::Root,
    widgets::{
        self,
        checkbox::{Checkbox, CheckboxEnabledMarker},
    },
};
use bevy::{
    ecs::{
        query::With,
        system::{Query, Res, ResMut},
    },
    input::{
        gamepad::{GamepadButton, GamepadButtonType, Gamepads},
        keyboard::KeyCode,
        Input,
    },
    time::Time,
};
use glyph_render::glyph_buffer::GlyphBuffer;
use grid_physics::{actor::Actor, sets::EnablePhysicsSystems, solid::Solid};

use spatial_grid::grid::SpatialGrid;

use crate::{
    debug::{DebugCollisions, DebugPositions, DebugUi},
    physics_grids::UiPhysicsGrid,
    player::PlayerMarker,
};

use super::{setup::DebugMenuMarker, state::DebugMenuState};

pub fn toggle_menu(
    keyboard: Res<Input<KeyCode>>,
    mut state: ResMut<DebugMenuState>,
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

pub fn update_values(
    state: Res<DebugMenuState>,
    mut collisions: ResMut<DebugCollisions>,
    mut positions: ResMut<DebugPositions>,
    mut pause_physics: ResMut<EnablePhysicsSystems>,
    mut ui: ResMut<DebugUi>,
    mut q_text: Query<&mut widgets::text::Text>,
    q_checkbox: Query<Option<&CheckboxEnabledMarker>, With<Checkbox>>,
    q_player: Query<(), With<PlayerMarker>>,
    q_solid: Query<(), With<Solid>>,
    q_actor: Query<(), With<Actor>>,
    q_entity: Query<()>,
    time: Res<Time>,
) {
    if !state.enabled {
        return;
    }

    if let Some(entity) = state.colliders_checkbox {
        let state = q_checkbox.get(entity).unwrap().is_some();
        **collisions = state;
    }
    if let Some(entity) = state.position_checkbox {
        let state = q_checkbox.get(entity).unwrap().is_some();
        **positions = state;
    }
    if let Some(entity) = state.pause_checkbox {
        let state = q_checkbox.get(entity).unwrap().is_some();
        **pause_physics = !state;
    }
    if let Some(entity) = state.ui_checkbox {
        let state = q_checkbox.get(entity).unwrap().is_some();
        **ui = !state;
    }

    if let Some(entity) = state.fps_text {
        q_text.get_mut(entity).unwrap().text = format!("FPS: {:0.2}", 1.0 / time.delta_seconds());
    }
    if let Some(entity) = state.player_count_text {
        q_text.get_mut(entity).unwrap().text = format!("Player Count: {}", q_player.iter().count());
    }
    if let Some(entity) = state.solid_count_text {
        q_text.get_mut(entity).unwrap().text = format!("Solid  Count: {}", q_solid.iter().count());
    }
    if let Some(entity) = state.actor_count_text {
        q_text.get_mut(entity).unwrap().text = format!("Actor  Count: {}", q_actor.iter().count());
    }
    if let Some(entity) = state.entity_count {
        q_text.get_mut(entity).unwrap().text = format!("Entity Count: {}", q_entity.iter().count());
    }
}
