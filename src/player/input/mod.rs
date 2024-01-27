use bevy::{
    app::Plugin,
    ecs::{bundle::Bundle, component::Component},
};

use self::keyboard::{PlayerInputMovement, PlayerKeyboardInputPlugin};

pub mod keyboard;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(PlayerKeyboardInputPlugin);
    }
}

#[derive(Component, Default)]
pub struct PlayerInputMarker;

#[derive(Bundle, Default)]
pub struct PlayerInputBundle {
    marker: PlayerInputMarker,
    movement_input: PlayerInputMovement,
}
