use bevy::prelude::*;

use crate::player::PlayerMarker;
use glyph_render::glyph_render_plugin::GlyphSpriteMirrored;

use super::PlayerMovementMarker;

#[derive(Debug, Component, Clone)]
pub(crate) struct PlayerDirection(IVec2);

impl PlayerDirection {
    pub(crate) fn new(dir: IVec2) -> Self {
        let mut direction = Self(IVec2::X);
        direction.set(dir);
        direction
    }

    pub(crate) fn get(&self) -> IVec2 {
        self.0
    }
    pub(crate) fn set(&mut self, dir: IVec2) {
        if dir.x < -1 || dir.x > 1 || dir.y < -1 || dir.y > 1 {
            panic!();
        }
        self.0 = dir;
    }
    pub(crate) fn set_x(&mut self, x: i32) {
        if !(-1..=1).contains(&x) {
            panic!();
        }
        self.0.x = x;
    }
    pub(crate) fn _set_y(&mut self, y: i32) {
        if !(-1..=1).contains(&y) {
            panic!();
        }
        self.0.y = y;
    }
}

impl Default for PlayerDirection {
    fn default() -> Self {
        Self(IVec2::X)
    }
}

pub(crate) fn player_update_sprite_mirror(
    mut commands: Commands,
    q_player: Query<(Entity, &PlayerDirection), (With<PlayerMarker>, With<PlayerMovementMarker>)>,
) {
    for (player, direction) in q_player.iter() {
        if direction.get().x == -1 {
            commands.entity(player).insert(GlyphSpriteMirrored);
        } else {
            commands.entity(player).remove::<GlyphSpriteMirrored>();
        }
    }
}
