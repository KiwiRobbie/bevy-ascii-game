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
    math::{IVec2, Vec2},
    render::camera::Camera,
    time::Time,
    transform::components::GlobalTransform,
    window::{PrimaryWindow, Window},
};
use grid_physics::{
    actor::Actor,
    debug::{DebugCollisions, DebugPositions},
    position::GridSize,
    sets::EnablePhysicsSystems,
    solid::Solid,
};

use crate::player::PlayerMarker;

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
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_root: Query<&mut Root, With<DebugMenuMarker>>,
    grid_size: Res<GridSize>,
) {
    let Ok((camera, transform)) = q_camera.get_single() else {
        return;
    };
    let Some(rect) = camera.logical_viewport_rect() else {
        return;
    };

    let Some(ray) = camera.viewport_to_world(transform, rect.max) else {
        return;
    };

    for mut root in q_root.iter_mut() {
        root.position = (ray.origin.truncate() / grid_size.as_vec2()).as_ivec2()
            + IVec2::new(-1, 0) * root.size.as_ivec2()
            - IVec2::ZERO;
        root.size.y = (rect.height() / grid_size.y as f32) as u32 - 1;
    }
}

pub fn update_values(
    state: Res<DebugMenuState>,
    mut collisions: ResMut<DebugCollisions>,
    mut positions: ResMut<DebugPositions>,
    mut pause_physics: ResMut<EnablePhysicsSystems>,
    mut q_text: Query<&mut widgets::text::Text>,
    q_checkbox: Query<Option<&CheckboxEnabledMarker>, With<Checkbox>>,
    q_player: Query<(), With<PlayerMarker>>,
    q_solid: Query<(), With<Solid>>,
    q_actor: Query<(), With<Actor>>,
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
    if let Some(entity) = state.pause_physics_checkbox {
        let state = q_checkbox.get(entity).unwrap().is_some();
        **pause_physics = !state;
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
}
