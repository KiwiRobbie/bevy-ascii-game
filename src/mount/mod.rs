use bevy::{color::palettes::css, prelude::*};
use glyph_render::{
    glyph_animation_graph::player::GlyphAnimationGraphTarget,
    glyph_render_plugin::{GlyphSolidColor, GlyphSpriteMirrored},
};
use grid_physics::{
    free::FreeMarker, gravity::Gravity, movement::Movement, plugin::PhysicsUpdateSet,
    velocity::Velocity,
};
use spatial_grid::position::Position;

use crate::player::{
    input::{player_inputs, PlayerInputSet},
    interaction::{PlayerInteractActive, PlayerInteractable},
    movement::{direction::PlayerDirection, PlayerMovementMarker},
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
                horse_movement_system,
                mount_update_sprite_mirror_system,
                horse_animation_system,
                dismount_system,
            ),
        )
        .add_systems(
            PostUpdate,
            update_rider_system
                .in_set(PhysicsUpdateSet::PostUpdate)
                .after(PhysicsUpdateSet::Update),
        )
        .add_systems(
            PreUpdate,
            transfer_player_inputs_system.after(PlayerInputSet),
        );
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

fn mount_interaction_system(
    q_mounts: Query<
        (Entity, &PlayerInteractActive, Has<GlyphSpriteMirrored>),
        With<MountableMarker>,
    >,
    mut commands: Commands,
) {
    for (mount, &PlayerInteractActive { player }, mirrored) in q_mounts.iter() {
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
        player_command.remove::<(PlayerMovementMarker, FreeMarker, Movement)>();
        player_command.insert(RiderMount { mount });
        if mirrored {
            player_command.insert(GlyphSpriteMirrored);
        } else {
            player_command.remove::<GlyphSpriteMirrored>();
        }
    }
}

fn dismount_system(
    q_mount: Query<
        (
            Entity,
            &mount_inputs::Movement,
            &MountRider,
            Has<GlyphSpriteMirrored>,
        ),
        With<MountableMarker>,
    >,
    mut commands: Commands,
) {
    for (mount, movement, &MountRider { rider }, mirrored) in q_mount.iter() {
        if movement.vertical > -0.5 {
            continue;
        }

        let mut mount_command: EntityCommands<'_> = commands.entity(mount);
        mount_command.remove::<(PlayerInteractable, MountRider)>();
        mount_command.insert((PlayerInteractable, mount_inputs::Movement::default()));

        let mut player_command: EntityCommands<'_> = commands.entity(rider);
        player_command.remove::<RiderMount>();
        player_command.insert((
            PlayerMovementMarker,
            FreeMarker,
            Movement::default(),
            PlayerDirection::new(if mirrored { IVec2::NEG_X } else { IVec2::X }),
        ));
    }
}

fn update_rider_system(
    q_mount: Query<
        (&Position, &Velocity, &MountOrigin, &MountRider),
        (With<MountMarker>, Without<PlayerMarker>),
    >,
    mut q_player: Query<(&mut Position, &mut Velocity), (Without<MountMarker>, With<PlayerMarker>)>,
) {
    for (mount_pos, mount_vel, mount_origin, &MountRider { rider }) in q_mount.iter() {
        let Ok((mut rider_pos, mut rider_vel)) = q_player.get_mut(rider) else {
            continue;
        };
        **rider_pos = mount_pos.0 + mount_origin.origin;
        **rider_vel = **mount_vel;
    }
}

fn mount_update_sprite_mirror_system(
    mut commands: Commands,
    q_mount: Query<(Entity, &mount_inputs::Movement, &MountRider), With<MountMarker>>,
) {
    for (player, movement, &MountRider { rider }) in q_mount.iter() {
        if movement.horizontal < -0.5 {
            commands.entity(player).insert(GlyphSpriteMirrored);
            commands.entity(rider).insert(GlyphSpriteMirrored);
        } else if movement.horizontal > 0.5 {
            commands.entity(player).remove::<GlyphSpriteMirrored>();
            commands.entity(rider).remove::<GlyphSpriteMirrored>();
        }
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

fn transfer_player_inputs_system(
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
    for (mut target, _, movement_input) in q_horse.iter_mut() {
        let target_str = if movement_input.horizontal != 0. {
            "gallop"
        } else {
            "idle"
        };
        **target = Some(target_str.into());
    }
}
