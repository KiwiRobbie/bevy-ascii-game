use bevy::{
    asset::{Assets, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query, Res},
    },
    prelude::{Deref, DerefMut},
    time::Time,
};

use crate::glyph_animation::{GlyphAnimation, GlyphAnimationSource};

use super::{GlyphAnimationGraph, GlyphAnimationGraphSource};

#[derive(Debug, Component, Clone)]
pub struct GlyphAnimationGraphSettings {
    pub framerate: f32,
}

impl Default for GlyphAnimationGraphSettings {
    fn default() -> Self {
        Self { framerate: 15.0 }
    }
}

#[derive(Debug, Component, Deref, DerefMut, Clone, Default)]
pub struct GlyphAnimationGraphTarget(pub Option<String>);

#[derive(Debug, Component, Default, Clone)]
pub struct GlyphAnimationGraphCurrent {
    pub transitional_states: Vec<Handle<GlyphAnimationSource>>,
    pub current_state: usize,
    pub frame_timer: f32,
}

pub fn animation_graph_player(
    mut commands: Commands,
    mut q_players: Query<(
        Entity,
        &mut GlyphAnimationGraph,
        &mut GlyphAnimationGraphCurrent,
        Option<&mut GlyphAnimation>,
        &GlyphAnimationGraphSettings,
    )>,
    time: Res<Time>,
    glyph_animations: Res<Assets<GlyphAnimationSource>>,
    glyph_animation_graphs: Res<Assets<GlyphAnimationGraphSource>>,
) {
    for (entity, graph, mut current, animation, settings) in q_players.iter_mut() {
        if let Some(mut animation) = animation {
            current.frame_timer += time.delta_secs() * settings.framerate;
            let Some(animation_source) = glyph_animations.get(&animation.source) else {
                continue;
            };
            if current.frame_timer > 1.0 {
                animation.frame += current.frame_timer as u32;
                current.frame_timer -= current.frame_timer.floor();
            }
            if animation.frame < animation_source.frames.len() as u32 {
                continue;
            }
        }

        let Some(graph_source) = glyph_animation_graphs.get(&graph.source) else {
            continue;
        };
        let new_animation_component =
            if let Some(transition_animation) = current.transitional_states.pop() {
                GlyphAnimation {
                    frame: 0,
                    source: transition_animation,
                }
            } else {
                GlyphAnimation {
                    frame: 0,
                    source: graph_source
                        .states
                        .get(current.current_state)
                        .unwrap()
                        .animation
                        .clone(),
                }
            };
        commands.entity(entity).insert(new_animation_component);
    }
}

pub fn animation_graph_traverse(
    mut q_animation_graphs: Query<(
        &mut GlyphAnimationGraph,
        &mut GlyphAnimationGraphCurrent,
        &GlyphAnimationGraphTarget,
    )>,
    glyph_animation_graphs: Res<Assets<GlyphAnimationGraphSource>>,
) {
    for (graph, mut current, target) in q_animation_graphs.iter_mut() {
        let Some(target) = target.as_ref() else {
            continue;
        };

        let Some(graph_source) = glyph_animation_graphs.get(&graph.source) else {
            continue;
        };
        let target = *graph_source.state_names.get(target).unwrap();

        if current.current_state != target {
            let transition = graph_source.traverse(current.current_state, target);
            current.current_state = target;

            current.transitional_states = transition.transitions.unwrap_or(vec![]);
        }
    }
}
