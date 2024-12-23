use bevy::{
    app::Plugin,
    ecs::{bundle::Bundle, component::Component},
    prelude::SystemSet,
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

#[derive(Debug, SystemSet, Hash, PartialEq, Eq, Clone)]
pub struct PlayerInputSet;

#[derive(Component, Default, Clone)]
pub struct PlayerInputMarker;

#[derive(Bundle, Default, Clone)]
pub struct PlayerInputBundle {
    marker: PlayerInputMarker,
    movement_input: player_inputs::Movement,
}

pub mod player_inputs {
    use bevy::{ecs::bundle::Bundle, ecs::component::Component};

    #[derive(Bundle)]
    pub struct MarkerResetBundle {
        jump: JumpMarker,
        lunge: LungeMarker,
        interact: InteractMarker,
        reset: ResetMarker,
    }

    #[derive(Component, Default, Debug)]
    pub struct DefendMarker;

    #[derive(Component, Default, Debug)]
    pub struct AttackMaker;

    #[derive(Component, Default, Debug)]
    pub struct JumpMarker;

    #[derive(Component, Default, Debug)]
    pub struct LungeMarker;

    #[derive(Component, Default, Debug)]
    pub struct InteractMarker;

    #[derive(Debug, Default, Component, Clone)]
    pub struct ResetMarker;

    #[derive(Debug, Default, Component, Clone)]
    pub struct Movement {
        pub horizontal: f32,
        pub vertical: f32,
    }
}
