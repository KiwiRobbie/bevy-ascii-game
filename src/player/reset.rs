use bevy::{
    asset::AssetServer,
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    math::{IVec2, UVec2, Vec2},
};

use glyph_render::{
    glyph_animation_graph::bundle::GlyphAnimationGraphBundle, glyph_buffer::TargetGlyphBuffer,
};
use grid_physics::{
    actor::ActorPhysicsBundle,
    collision::{Aabb, Collider},
    free::FreeMarker,
    gravity::Gravity,
    velocity::Velocity,
};
use spatial_grid::{
    depth::Depth,
    position::{Position, SpatialBundle},
    remainder::Remainder,
};

use crate::physics_grids::GamePhysicsGridMarker;

use super::{
    input::{controller::PlayerInputController, player_inputs::ResetMarker},
    movement::{walk::PlayerWalkSpeed, PlayerMovementBundle},
    PlayerBundle,
};

pub(crate) fn player_reset_system(
    mut commands: Commands,
    q_player: Query<Entity, With<ResetMarker>>,
) {
    for player in q_player.iter() {
        commands.entity(player).insert((
            Position(IVec2::new(10, 10)),
            Remainder(Vec2::ZERO),
            Velocity(Vec2::ZERO),
        ));
    }
}

pub fn create_player_with_gamepad(
    commands: &mut Commands<'_, '_>,
    server: &Res<'_, AssetServer>,
    gamepad: Entity,
    glyph_buffer: Entity,
) {
    create_player(commands, server)
        .insert(PlayerInputController(gamepad))
        .insert(TargetGlyphBuffer(glyph_buffer))
        .insert(GamePhysicsGridMarker);
}

pub fn create_player<'a>(
    commands: &'a mut Commands,
    server: &Res<AssetServer>,
) -> bevy::ecs::system::EntityCommands<'a> {
    commands.spawn((
        GlyphAnimationGraphBundle::from_source(server.load("anim/player/player.agraph.ron")),
        PlayerBundle {
            actor: ActorPhysicsBundle {
                position: SpatialBundle {
                    position: IVec2::new(10, 10).into(),
                    ..Default::default()
                },
                collider: Collider {
                    shape: Aabb {
                        start: IVec2::ZERO,
                        size: UVec2 { x: 6, y: 5 },
                    }
                    .into(),
                },

                ..Default::default()
            },
            movement: PlayerMovementBundle {
                walk_speed: PlayerWalkSpeed { speed: 50.0 },
                ..Default::default()
            },
            ..Default::default()
        },
        FreeMarker,
        Gravity::default(),
        Velocity::default(),
        Depth(0.0),
    ))
}
