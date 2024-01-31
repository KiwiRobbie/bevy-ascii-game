use bevy::{
    asset::AssetServer,
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    input::gamepad::Gamepad,
    math::{IVec2, UVec2, Vec2},
    render::color::Color,
};

use glyph_render::{
    atlas::{CharacterSet, FontAtlasUser},
    font::{CustomFont, FontSize},
    glyph_animation_graph::bundle::GlyphAnimationGraphBundle,
    glyph_render_plugin::{GlyphSolidColor, GlyphSpriteMirrored},
};
use grid_physics::{
    actor::ActorPhysicsBundle,
    collision::{Aabb, Collider, CollisionShape},
    free::FreeMarker,
    gravity::Gravity,
    position::{Position, PositionBundle},
    velocity::Velocity,
};

use super::{
    input::{controller::PlayerInputController, PlayerInputReset},
    movement::{walk::PlayerWalkSpeed, PlayerMovementBundle},
    PlayerBundle,
};

pub fn player_reset_system(
    mut commands: Commands,
    q_player: Query<Entity, With<PlayerInputReset>>,
) {
    for player in q_player.iter() {
        commands.entity(player).insert((
            Position {
                position: IVec2::ZERO,
                remainder: Vec2::ZERO,
            },
            Velocity {
                ..Default::default()
            },
        ));
    }
}

pub fn create_player_with_gamepad(
    commands: &mut Commands<'_, '_>,
    server: &Res<'_, AssetServer>,
    gamepad: Gamepad,
) {
    create_player(commands, server)
        .insert(PlayerInputController(gamepad))
        .insert(GlyphSolidColor {
            color: Color::hsl(360.0 * (1.0 + gamepad.id as f32) / 6.0, 1.0, 0.6).as_rgba_linear()
                * 10.0,
        });
}

pub fn create_player<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    server: &Res<AssetServer>,
) -> bevy::ecs::system::EntityCommands<'w, 's, 'a> {
    commands.spawn((
        GlyphAnimationGraphBundle::from_source(server.load("anim/player/player.agraph.ron")),
        FontAtlasUser,
        CustomFont(server.load("FiraCode-Regular.ttf")),
        CharacterSet(CHARSET.chars().collect()),
        FontSize(32),
        GlyphSpriteMirrored,
        PlayerBundle {
            actor: ActorPhysicsBundle {
                position: PositionBundle {
                    position: Position {
                        position: IVec2 { x: -20, y: 0 },
                        ..Default::default()
                    },
                    ..Default::default()
                },
                collider: Collider {
                    shape: CollisionShape::Aabb(Aabb {
                        min: IVec2::ZERO,
                        size: UVec2 { x: 6, y: 5 },
                    }),
                },

                ..Default::default()
            },
            movement: PlayerMovementBundle {
                walk_speed: PlayerWalkSpeed { speed: 1.0 },
                ..Default::default()
            },
            ..Default::default()
        },
        FreeMarker,
        Gravity::default(),
        Velocity::default(),
    ))
}
const CHARSET: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
