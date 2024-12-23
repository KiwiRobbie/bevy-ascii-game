use std::collections::{self, BinaryHeap};

use bevy::{
    color::palettes::css,
    ecs::{query::QueryFilter, system::SystemParam},
    prelude::*,
};
use glyph_render::glyph_render_plugin::GlyphSolidColor;
use grid_physics::{
    actor::FilterActors,
    collision::{Collider, RayTest},
};
use spatial_grid::{position::Position, remainder::Remainder};

use crate::utils::clear_component;

use super::{input::player_inputs, movement::direction::PlayerDirection};

pub(crate) struct PlayerInteractionPlugin;
impl Plugin for PlayerInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    clear_component::<PlayerInteractFocused>,
                    clear_component::<PlayerInteractActive>,
                ),
                interaction_system,
                interaction_color_system,
            )
                .chain(),
        );
    }
}

#[derive(SystemParam)]
pub(crate) struct Raycast<'w, 's, F>
where
    F: QueryFilter + 'static,
{
    actors: Query<'w, 's, (Entity, &'static Position, &'static Collider), (FilterActors, F)>,
}

#[derive(Debug)]
pub(crate) struct RaycastConfig {
    origin: Position,
    dir_inv: Vec2,
    start: Option<f32>,
    end: Option<f32>,
}
impl RaycastConfig {
    pub(crate) fn cast<F>(self, raycast: &Raycast<F>) -> RaycastState
    where
        F: QueryFilter,
    {
        let mut heap = collections::BinaryHeap::new();
        for (actor, pos, col) in raycast.actors.iter() {
            if let Some((min, max)) = (pos, col).test_ray(*self.origin, self.dir_inv) {
                if let Some(start) = self.start {
                    if min < start {
                        continue;
                    }
                }
                if let Some(end) = self.end {
                    if min > end {
                        continue;
                    }
                }
                heap.push(RaycastIntersection { min, max, actor });
            };
        }

        RaycastState {
            position: self.origin,
            remainder: Vec2::ZERO.into(),
            heap,
            config: self,
        }
    }
}

pub(crate) struct RaycastState {
    position: Position,
    remainder: Remainder,
    config: RaycastConfig,
    heap: BinaryHeap<RaycastIntersection>,
}

#[derive(Debug, Clone)]
struct RaycastIntersection {
    min: f32,
    max: f32,
    actor: Entity,
}

impl PartialEq for RaycastIntersection {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min
    }
}
impl Eq for RaycastIntersection {}

// Reversed ordering to turn min-heap to max-heap
impl PartialOrd for RaycastIntersection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.min.partial_cmp(&self.min)
    }
}
impl Ord for RaycastIntersection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.min.total_cmp(&self.min)
    }
}

impl RaycastState {
    pub(crate) fn get_first(self) -> Option<RaycastIntersection> {
        self.heap.peek().cloned()
    }
    pub(crate) fn get_next(&mut self) -> Option<RaycastIntersection> {
        self.heap.pop()
    }
}

#[derive(Debug, Component, Clone, Default)]
pub(crate) struct InteractionSource {
    offset: IVec2,
}

#[derive(Debug, Component)]
pub struct PlayerInteractable;

#[derive(Debug, Component)]
pub struct PlayerInteractFocused {
    pub(crate) player: Entity,
}
#[derive(Debug, Component)]
pub(crate) struct PlayerInteractActive {
    pub(crate) player: Entity,
}

pub(crate) fn interaction_color_system(
    q_interactable: Query<
        (
            Entity,
            Has<PlayerInteractFocused>,
            Has<PlayerInteractActive>,
        ),
        With<PlayerInteractable>,
    >,
    mut commands: Commands,
) {
    for (entity, focused, active) in q_interactable.iter() {
        let color = match (focused, active) {
            (false, false) => css::WHITE,
            (true, false) => css::ORANGE_RED,
            (_, true) => css::GREEN,
        }
        .into();
        commands.entity(entity).insert(GlyphSolidColor { color });
    }
}

pub(crate) fn interaction_system(
    q_player: Query<(
        Entity,
        &Position,
        &InteractionSource,
        &PlayerDirection,
        Has<player_inputs::InteractMarker>,
    )>,
    raycast: Raycast<'_, '_, With<PlayerInteractable>>,
    mut commands: Commands,
) {
    for (player, pos, source, dir, interacting) in q_player.iter() {
        let dir_inv = dir.get().as_vec2().recip();
        let ray_config = RaycastConfig {
            origin: pos.offset(source.offset),
            dir_inv,
            start: Some(0.),
            end: Some(10.),
        };
        let result = ray_config.cast(&raycast);
        if let Some(intersection) = result.get_first() {
            let mut command = commands.entity(intersection.actor);

            command.insert(PlayerInteractFocused { player });
            if interacting {
                command.insert(PlayerInteractActive { player });
            }
        }
    }
}
