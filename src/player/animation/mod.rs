use bevy::{
    ecs::{query::With, system::Query},
    prelude::{Has, Without},
};

use glyph_render::glyph_animation_graph::player::GlyphAnimationGraphTarget;
use grid_physics::free::FreeGrounded;

use crate::mount::{mount_inputs, MountMarker, RiderMount};

use super::{
    input::player_inputs,
    movement::{lunge::PlayerLunging, PlayerMovementMarker},
    PlayerMarker,
};

pub(super) fn set_animation_target(
    mut q_players: Query<
        (
            &mut GlyphAnimationGraphTarget,
            Has<PlayerMovementMarker>,
            &player_inputs::Movement,
            Has<FreeGrounded>,
            Has<PlayerLunging>,
            Option<&RiderMount>,
        ),
        With<PlayerMarker>,
    >,
    q_mount: Query<&mount_inputs::Movement, (With<MountMarker>, Without<PlayerMarker>)>,
) {
    for (mut animation_target, movement_enabled, input_movement, grounded, lunging, mount) in
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
        } else if let Some(&RiderMount { mount }) = mount {
            if q_mount
                .get(mount)
                .map(|movement| movement.horizontal != 0.)
                .unwrap_or(false)
            {
                "mounted_gallop"
            } else {
                "mounted_idle"
            }
        } else {
            "idle"
        };

        **animation_target = Some(target.into());
    }
}
