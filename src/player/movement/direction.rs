use crate::player::PlayerMarker;
use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    math::IVec2,
};
use glyph_render::glyph_render_plugin::GlyphSpriteMirrored;

use super::PlayerMovementMarker;

#[derive(Debug, Component, Clone)]
pub struct PlayerDirection(IVec2);

impl PlayerDirection {
    pub fn new(dir: IVec2) -> Self {
        let mut direction = Self(IVec2::X);
        direction.set(dir);
        direction
    }

    pub fn get(&self) -> IVec2 {
        self.0
    }
    pub fn set(&mut self, dir: IVec2) {
        if dir.x < -1 || dir.x > 1 || dir.y < -1 || dir.y > 1 {
            panic!();
        }
        self.0 = dir;
    }
    pub fn set_x(&mut self, x: i32) {
        if x < -1 || x > 1 {
            panic!();
        }
        self.0.x = x;
    }
    pub fn set_y(&mut self, y: i32) {
        if y < -1 || y > 1 {
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

pub fn player_update_sprite_mirror(
    mut commands: Commands,
    q_player: Query<(Entity, &PlayerDirection), (With<PlayerMarker>, With<PlayerMovementMarker>)>,
) {
    for (player, direction) in q_player.iter() {
        commands.entity(player).log_components();
        if direction.get().x == -1 {
            commands.entity(player).insert(GlyphSpriteMirrored);
        } else {
            commands.entity(player).remove::<GlyphSpriteMirrored>();
        }
    }
}
