use bevy::prelude::*;

use self::{controller::PlayerControllerInputPlugin, keyboard::PlayerKeyboardInputPlugin};

pub mod controller;
pub mod keyboard;

pub(crate) struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((PlayerControllerInputPlugin, PlayerKeyboardInputPlugin));
    }
}

#[derive(Debug, SystemSet, Hash, PartialEq, Eq, Clone)]
pub(crate) struct PlayerInputSet;

#[derive(Component, Default, Clone)]
pub(crate) struct PlayerInputMarker;

#[derive(Bundle, Default, Clone)]
pub(crate) struct PlayerInputBundle {
    marker: PlayerInputMarker,
    movement_input: player_inputs::Movement,
}

pub(crate) mod player_inputs {
    use bevy::prelude::*;

    #[derive(Bundle)]
    pub(crate) struct MarkerResetBundle {
        jump: JumpMarker,
        lunge: LungeMarker,
        interact: InteractMarker,
        reset: ResetMarker,
    }

    #[derive(Component, Default, Debug)]
    pub(crate) struct DefendMarker;

    #[derive(Component, Default, Debug)]
    pub(crate) struct AttackMaker;

    #[derive(Component, Default, Debug)]
    pub(crate) struct JumpMarker;

    #[derive(Component, Default, Debug)]
    pub(crate) struct LungeMarker;

    #[derive(Component, Default, Debug)]
    pub(crate) struct InteractMarker;

    #[derive(Debug, Default, Component, Clone)]
    pub(crate) struct ResetMarker;

    #[derive(Debug, Default, Component, Clone)]
    pub(crate) struct Movement {
        pub(crate) horizontal: f32,
        pub(crate) vertical: f32,
    }
}
