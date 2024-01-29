use bevy::ecs::{
    entity::Entity,
    query::With,
    system::{Commands, Query},
};

use crate::{
    glyph_animation_graph::player::GlyphAnimationGraphTarget, physics::free::FreeGrounded,
};

use super::{input::PlayerInputMovement, PlayerMarker};

pub fn set_animation_target(
    mut commands: Commands,
    q_players: Query<(Entity, &PlayerInputMovement, Option<&FreeGrounded>), With<PlayerMarker>>,
) {
    for (player, movement, grounded) in q_players.iter() {
        let movement = movement.horizontal.abs() > 0.5;
        let grounded = grounded.is_some();

        let target = if grounded {
            if movement {
                "running"
            } else {
                "idle"
            }
        } else {
            if movement {
                "air_strafe"
            } else {
                "air_idle"
            }
        };

        dbg!(target);
        commands
            .entity(player)
            .insert(GlyphAnimationGraphTarget(target.into()));
    }
}
