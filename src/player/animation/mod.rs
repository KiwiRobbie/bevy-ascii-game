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
    q_players: Query<(Entity, &PlayerInputMovement), (With<PlayerMarker>, With<FreeGrounded>)>,
) {
    for (player, movement) in q_players.iter() {
        if movement.horizontal == 0.0 {
            commands
                .entity(player)
                .insert(GlyphAnimationGraphTarget("idle".into()));
        } else {
            commands
                .entity(player)
                .insert(GlyphAnimationGraphTarget("running".into()));
        }
    }
}
