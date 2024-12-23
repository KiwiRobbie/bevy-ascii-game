use bevy::{color::palettes::css, prelude::*};
use glyph_render::{
    glyph_animation_graph::player::GlyphAnimationGraphTarget, glyph_render_plugin::GlyphSolidColor,
};
use grid_physics::{gravity::Gravity, movement::Movement, velocity::Velocity};
use spatial_grid::position::Position;

use crate::player::{
    input::{player_inputs, PlayerInputSet},
    interaction::{PlayerInteractActive, PlayerInteractable},
    movement::PlayerMovementMarker,
    PlayerMarker,
};

pub mod horse;

pub struct HorsePlugin;
impl Plugin for HorsePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                mount_interaction_system,
                (horse_movement_system, update_rider_position).chain_ignore_deferred(),
                horse_animation_system,
            ),
        )
        .add_systems(PreUpdate, transfer_player_inputs.after(PlayerInputSet));
    }
}

#[derive(Debug, Component)]
pub struct MountMarker;

#[derive(Debug, Component)]
pub struct MountOrigin {
    pub origin: IVec2,
}

#[derive(Debug, Component)]
pub struct MountableMarker;

#[derive(Debug, Component)]
pub(crate) struct RiderMount {
    pub(crate) mount: Entity,
}

#[derive(Debug, Component)]
pub(crate) struct MountRider {
    rider: Entity,
}

pub(crate) fn mount_interaction_system(
    q_mounts: Query<(Entity, &PlayerInteractActive), With<MountableMarker>>,
    mut commands: Commands,
) {
    for (mount, &PlayerInteractActive { player }) in q_mounts.iter() {
        let mut mount_command: EntityCommands<'_> = commands.entity(mount);
        mount_command.remove::<PlayerInteractable>();
        mount_command.insert((
            MountRider { rider: player },
            mount_inputs::Movement::default(),
            Velocity::default(),
            Gravity::default(),
            Movement::default(),
            GlyphSolidColor {
                color: css::WHITE.into(),
            },
        ));

        let mut player_command: EntityCommands<'_> = commands.entity(player);
        player_command.remove::<(PlayerMovementMarker, Velocity, Gravity, Movement)>();
        player_command.insert(RiderMount { mount });
    }
}

pub(crate) fn update_rider_position(
    q_mount: Query<
        (&Position, &MountOrigin, &MountRider),
        (With<MountMarker>, Without<PlayerMarker>),
    >,
    mut q_player: Query<&mut Position, (Without<MountMarker>, With<PlayerMarker>)>,
) {
    for (mount_pos, mount_origin, &MountRider { rider }) in q_mount.iter() {
        let Ok(mut rider_pos) = q_player.get_mut(rider) else {
            continue;
        };
        **rider_pos = mount_pos.0 + mount_origin.origin;
    }
}

pub mod mount_inputs {
    use bevy::prelude::Component;

    #[derive(Debug, Component, Default)]
    pub struct Movement {
        pub horizontal: f32,
        pub vertical: f32,
    }

    #[derive(Debug, Component)]
    pub struct JumpMarker;
}

pub(crate) fn transfer_player_inputs(
    mut q_mount: Query<
        (Entity, &MountRider, &mut mount_inputs::Movement),
        (With<MountMarker>, Without<PlayerMarker>),
    >,
    q_player: Query<
        (&player_inputs::Movement, Has<player_inputs::JumpMarker>),
        (Without<MountMarker>, With<PlayerMarker>),
    >,
    mut commands: Commands,
) {
    for (mount, &MountRider { rider }, mut mount_movement) in q_mount.iter_mut() {
        let Ok((player_movement, player_jump)) = q_player.get(rider) else {
            continue;
        };
        mount_movement.horizontal = player_movement.horizontal;
        mount_movement.vertical = player_movement.vertical;

        let mut mount_commands = commands.entity(mount);
        if player_jump {
            mount_commands.insert(mount_inputs::JumpMarker);
        } else {
            mount_commands.remove::<mount_inputs::JumpMarker>();
        }
    }
}

fn horse_movement_system(mut q_horse_movement: Query<(&mut Velocity, &mount_inputs::Movement)>) {
    for (mut velocity, input) in q_horse_movement.iter_mut() {
        velocity.x = input.horizontal * 100.0;
    }
}

fn horse_animation_system(
    mut q_horse: Query<(
        &mut GlyphAnimationGraphTarget,
        Has<MountRider>,
        &mount_inputs::Movement,
    )>,
) {
    for (mut target, has_rider, movement_input) in q_horse.iter_mut() {
        let target_str = if movement_input.horizontal != 0. {
            "gallop"
        } else {
            "idle"
        };
        **target = Some(target_str.into());
    }
}
