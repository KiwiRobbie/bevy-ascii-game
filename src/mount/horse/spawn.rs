use bevy::prelude::*;
use glyph_render::glyph_animation_graph::bundle::GlyphAnimationGraphBundle;
use grid_physics::{
    actor::ActorPhysicsBundle, collision::Aabb, free::FreeMarker, gravity::Gravity,
    velocity::Velocity,
};
use spatial_grid::depth::Depth;

use crate::{
    mount::{MountMarker, MountOrigin, MountableMarker},
    physics_grids::GamePhysicsGridMarker,
    player::interaction::PlayerInteractable,
};

pub fn create_horse<'a>(
    commands: &'a mut Commands,
    server: &Res<AssetServer>,
) -> EntityCommands<'a> {
    commands.spawn((
        GlyphAnimationGraphBundle::from_source(server.load("anim/horse/horse.agraph.ron")),
        ActorPhysicsBundle {
            collider: Aabb {
                start: IVec2::new(0, 0),
                size: UVec2 { x: 30, y: 10 },
            }
            .into(),

            position: IVec2::new(10, 10).into(),
            ..Default::default()
        },
        FreeMarker,
        Gravity::default(),
        Velocity::default(),
        GamePhysicsGridMarker,
        Depth(-1.0),
        PlayerInteractable,
        MountMarker,
        MountableMarker,
        MountOrigin {
            origin: IVec2::new(13, 4),
        },
    ))
}
