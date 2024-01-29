use bevy::{
    app::Plugin,
    ecs::{bundle::Bundle, component::Component},
};

use self::{controller::PlayerControllerInputPlugin, keyboard::PlayerKeyboardInputPlugin};

pub mod controller;
pub mod keyboard;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((PlayerControllerInputPlugin, PlayerKeyboardInputPlugin));
    }
}

#[derive(Component, Default, Clone)]
pub struct PlayerInputMarker;

#[derive(Bundle, Default, Clone)]
pub struct PlayerInputBundle {
    marker: PlayerInputMarker,
    movement_input: PlayerInputMovement,
}

#[derive(Component, Default, Debug)]
pub struct PlayerInputJump;

#[derive(Debug, Default, Component, Clone)]
pub struct PlayerInputMovement {
    pub horizontal: f32,
    pub vertical: f32,
}
