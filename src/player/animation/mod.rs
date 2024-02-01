use bevy::ecs::{
    entity::Entity,
    query::With,
    system::{Commands, Query},
};

use glyph_render::glyph_animation_graph::player::GlyphAnimationGraphTarget;
use grid_physics::free::FreeGrounded;

use super::{input::PlayerInputMovement, movement::lunge::PlayerLunging, PlayerMarker};

pub fn set_animation_target(
    mut commands: Commands,
    q_players: Query<
        (
            Entity,
            &PlayerInputMovement,
            Option<&FreeGrounded>,
            Option<&PlayerLunging>,
        ),
        With<PlayerMarker>,
    >,
) {
    for (player, movement, grounded, lunging) in q_players.iter() {
        let movement = movement.horizontal.abs() > 0.5;
        let grounded = grounded.is_some();
        let lunging = lunging.is_some();

        let target = if lunging {
            "lunging"
        } else if grounded {
            if movement {
                "running"
            } else {
                "idle"
            }
        } else if movement {
            "air_strafe"
        } else {
            "air_idle"
        };

        commands
            .entity(player)
            .insert(GlyphAnimationGraphTarget(target.into()));
    }
}
