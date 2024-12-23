use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        system::{Query, Res},
    },
    time::Time,
};

use super::{GlyphAnimation, GlyphAnimationSource};

#[derive(Debug, Component, Clone)]
pub struct GlyphAnimationPlayer {
    pub framerate: f32,
    pub repeat: bool,
    pub frame_timer: f32,
}

pub(crate) fn loop_animation_player(
    mut q_players: Query<(&mut GlyphAnimation, &mut GlyphAnimationPlayer)>,
    time: Res<Time>,
    glyph_animations: Res<Assets<GlyphAnimationSource>>,
) {
    for (mut animation, mut player) in q_players.iter_mut() {
        player.frame_timer += time.delta_secs() * player.framerate;
        let Some(animation_source) = glyph_animations.get(&animation.source) else {
            continue;
        };
        if player.frame_timer > 1.0 {
            animation.frame = (animation.frame + player.frame_timer as u32)
                .rem_euclid(animation_source.frames.len() as u32);
            player.frame_timer -= player.frame_timer.floor();
        }
    }
}
