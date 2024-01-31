use bevy::{
    ecs::{
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
        free::{FreeGrounded, FreeMarker},
        movement::{Movement, MovementObstructed},
        velocity::Velocity,
    },
    player::{
        input::{PlayerInputLunge, PlayerInputMarker},
        PlayerMarker,
    },
};

use super::{direction::PlayerDirection, MovementFilter};

#[derive(Debug, Component)]
pub struct PlayerLunging {
    pub timer: f32,
    pub direction: Vec2,
    pub speed: f32,
}

#[derive(Debug, Component)]
pub struct PlayerLungeCooldown {
    pub timer: f32,
}

#[derive(Debug, Component, Clone)]
pub struct PlayerLungeSettings {
    pub speed: f32,
    pub duration: f32,
    pub cooldown: f32,
    pub exit_velocity: f32,
}

impl Default for PlayerLungeSettings {
    fn default() -> Self {
        Self {
            speed: 80.0,
            duration: 0.25,
            cooldown: 0.5,
            exit_velocity: 50.0,
        }
    }
}

pub fn player_lunge_start_system(
    mut commands: Commands,
    q_player: Query<
        (Entity, &PlayerDirection, &PlayerLungeSettings),
        (
            MovementFilter,
            With<FreeMarker>,
            With<PlayerInputLunge>,
            Without<PlayerLungeCooldown>,
        ),
    >,
) {
    for (entity, direction, settings) in q_player.iter() {
        commands
            .entity(entity)
            .insert((PlayerLunging {
                direction: direction.get().as_vec2(),
                speed: settings.speed,
                timer: settings.duration,
            },))
            .remove::<PlayerInputMarker>()
            .remove::<FreeMarker>();
    }
}
pub fn player_lunge_update_system(
    mut commands: Commands,
    mut q_player_lunging: Query<
        (
            Entity,
            &mut PlayerLunging,
            &mut Movement,
            Option<&MovementObstructed>,
            &PlayerLungeSettings,
        ),
        With<PlayerMarker>,
    >,
    time: Res<Time>,
) {
    for (entity, mut lunging, mut movement, obstructed, settings) in q_player_lunging.iter_mut() {
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
                .insert((
                    FreeMarker,
                    PlayerInputMarker,
                    PlayerLungeCooldown {
                        timer: settings.cooldown,
                    },
                    Velocity {
                        velocity: Vec2 {
                            x: lunging.direction.x * settings.exit_velocity,
                            y: 0.0,
                        },
                    },
                ))
                .remove::<PlayerLunging>();
        }

        movement.add(lunging.direction * lunging.speed * time.delta_seconds());
    }
}

pub fn player_lunge_cooldown_update(
    mut commands: Commands,
    mut q_player_cooldown: Query<(Entity, &mut PlayerLungeCooldown, Option<&FreeGrounded>)>,
    time: Res<Time>,
) {
    for (entity, mut cooldown, grounded) in q_player_cooldown.iter_mut() {
        if cooldown.timer > 0.0 {
            cooldown.timer -= time.delta_seconds();
        }
        if cooldown.timer <= 0.0 {
            if grounded.is_some() {
                commands.entity(entity).remove::<PlayerLungeCooldown>();
            }
        }
    }
}
