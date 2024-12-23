use bevy::{
    ecs::{query::With, system::Query},
    prelude::Has,
};

use glyph_render::glyph_animation_graph::player::GlyphAnimationGraphTarget;
use grid_physics::free::FreeGrounded;

use crate::mount::RiderMount;

use super::{
    input::player_inputs,
    movement::{lunge::PlayerLunging, PlayerMovementMarker},
    PlayerMarker,
};

pub(crate) fn set_animation_target(
    mut q_players: Query<
        (
            &mut GlyphAnimationGraphTarget,
            Has<PlayerMovementMarker>,
            &player_inputs::Movement,
            Has<FreeGrounded>,
            Has<PlayerLunging>,
            Has<RiderMount>,
        ),
        With<PlayerMarker>,
    >,
) {
    for (mut animation_target, movement_enabled, input_movement, grounded, lunging, mounted) in
        q_players.iter_mut()
    {
        let horizontal_movement = input_movement.horizontal.abs() > 0.5;
        let target = if movement_enabled {
            if lunging {
                "lunging"
            } else if grounded {
                if horizontal_movement {
                    "running"
                } else {
                    "idle"
                }
            } else if horizontal_movement {
                "air_strafe"
            } else {
                "air_idle"
            }
        } else if mounted {
            "idle"
        } else {
            "idle"
        };

        **animation_target = Some(target.into());
    }
}
