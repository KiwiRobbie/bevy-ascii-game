use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    math::Vec2,
    time::Time,
};

use crate::{
    physics::{
        free::FreeMarker,
        movement::{Movement, MovementObstructed},
        velocity::Velocity,
    },
    player::{
        input::{PlayerInputJump, PlayerInputLunge, PlayerInputMarker, PlayerInputMovement},
        PlayerMarker,
    },
};

use super::{jump::PlayerJumpVelocity, MovementFilter};

#[derive(Debug, Component)]
pub struct PlayerLunging {
    pub timer: f32,
    pub direction: Vec2,
    pub speed: f32,
}

#[derive(Debug, Component, Clone)]
pub struct PlayerLungeSettings {
    pub speed: f32,
    pub duration: f32,
}

impl Default for PlayerLungeSettings {
    fn default() -> Self {
        Self {
            speed: 100.0,
            duration: 0.4,
        }
    }
}

pub fn player_lunge_start_system(
    mut commands: Commands,
    q_player: Query<
        (Entity, &PlayerInputMovement, &PlayerLungeSettings),
        (MovementFilter, With<FreeMarker>, With<PlayerInputLunge>),
    >,
) {
    for (entity, movement_input, settings) in q_player.iter() {
        let direction = if movement_input.horizontal.is_sign_positive() {
            Vec2::X
        } else {
            Vec2::NEG_X
        };

        commands
            .entity(entity)
            .insert(PlayerLunging {
                direction,
                speed: settings.speed,
                timer: settings.duration,
            })
            .remove::<PlayerInputMarker>()
            .remove::<FreeMarker>();
    }
}
pub fn player_lunge_update_system(
    mut commands: Commands,
    mut q_player: Query<
        (
            Entity,
            &mut PlayerLunging,
            &mut Movement,
            Option<&MovementObstructed>,
        ),
        With<PlayerMarker>,
    >,
    time: Res<Time>,
) {
    for (entity, mut lunging, mut movement, obstructed) in q_player.iter_mut() {
        lunging.timer -= time.delta_seconds();
        let obstructed = if let Some(obstructed) = obstructed {
            lunging.direction.x > 0.0 && obstructed.x.is_some()
                || lunging.direction.y > 0.0 && obstructed.y.is_some()
                || lunging.direction.x < 0.0 && obstructed.neg_x.is_some()
                || lunging.direction.x < 0.0 && obstructed.neg_y.is_some()
        } else {
            false
        };

        if obstructed || lunging.timer <= 0.0 {
            commands
                .entity(entity)
                .insert((FreeMarker, PlayerInputMarker))
                .remove::<PlayerLunging>();
        }

        movement.add(lunging.direction * lunging.speed * time.delta_seconds());
    }
}
