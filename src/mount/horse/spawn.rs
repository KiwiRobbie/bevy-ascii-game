use bevy::{
    asset::AssetServer,
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    input::gamepad::Gamepad,
    math::{IVec2, UVec2, Vec2},
    prelude::Component,
};

use glyph_render::{
    glyph_animation::{player::GlyphAnimationPlayer, GlyphAnimation},
    glyph_buffer::TargetGlyphBuffer,
    glyph_render_plugin::GlyphSolidColor,
};
use grid_physics::{
    actor::ActorPhysicsBundle,
    collision::{Aabb, Collider, CompositeCollisionShape},
    free::FreeMarker,
    gravity::Gravity,
    velocity::Velocity,
};
use spatial_grid::{
    depth::Depth,
    position::{Position, SpatialBundle},
    remainder::Remainder,
};

use crate::{
    physics_grids::GamePhysicsGridMarker,
    player::{
        movement::{walk::PlayerWalkSpeed, PlayerMovementBundle},
        PlayerBundle,
    },
};

// #[derive(Component)]
// enum HorseMode {
//     Player,
//     Ai,
//     Idle,
// }

// pub fn spawn_mount<'w, 's, 'a>(
//     mode: Query<HorseMode::Player>,
//     commands: &'a mut Commands<'w, 's>,
//     server: &Res<AssetServer>,
// ) -> bevy::ecs::system::EntityCommands<'a> {
//     commands.spawn((
//         GlyphAnimation {
//             source: server.load("anim/horse/states/unmounted/idle.anim.ron"),
//             frame: 0,
//         },
//         GlyphAnimationPlayer {
//             framerate: 10.0,
//             repeat: true,
//             frame_timer: 0.0,
//         },
//         ActorPhysicsBundle {
//             collider: Collider {
//                 shape: CollisionShape::Aabb(Aabb {
//                     min: IVec2::new(0, 0),
//                     size: UVec2 { x: 30, y: 10 },
//                 }),
//             },
//             position: IVec2::new(60, 10).into(),
//             ..Default::default()
//         },
//         FreeMarker,
//         Gravity::default(),
//         Velocity::default(),
//         GamePhysicsGridMarker,
//         Depth(0.5),
//         PlayerBundle {
//             actor: ActorPhysicsBundle {
//                 position: SpatialBundle {
//                     position: Position(IVec2::new(10, 10)),
//                     ..Default::default()
//                 },
//                 collider: Collider {
//                     shape: CollisionShape::Aabb(Aabb {
//                         min: IVec2::ZERO,
//                         size: UVec2 { x: 6, y: 5 },
//                     }),
//                 },

//                 ..Default::default()
//             },
//             movement: PlayerMovementBundle {
//                 walk_speed: PlayerWalkSpeed { speed: 50.0 },
//                 ..Default::default()
//             },
//             ..Default::default()
//         },
//     ))
// }
