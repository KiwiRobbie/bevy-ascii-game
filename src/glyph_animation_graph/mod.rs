use anyhow::Context;
use bevy::{
    asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, Handle},
    ecs::component::Component,
    reflect::TypePath,
    utils::hashbrown::HashMap,
};
use serde::Deserialize;

use crate::glyph_animation::GlyphAnimationSource;

pub mod bundle;
pub mod player;
pub mod plugin;

#[derive(Default)]
pub struct GlyphAnimationGraphAssetLoader {}

impl AssetLoader for GlyphAnimationGraphAssetLoader {
    type Asset = GlyphAnimationGraphSource;
    type Error = anyhow::Error;
    type Settings = ();

    fn extensions(&self) -> &[&str] {
        &["agraph.ron"]
    }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let meta = ron::de::from_bytes::<GlyphAnimationGraphMeta>(&bytes)?;

            let mut states = Vec::with_capacity(meta.states.len());
            let mut transitions = vec![vec![]; meta.transitions.len()];

            let mut state_names: HashMap<String, usize> = HashMap::with_capacity(meta.states.len());

            for state in meta.states {
                if state_names
                    .insert(state.name.clone(), states.len())
                    .is_some()
                {
                    anyhow::bail!("States must have unique names!");
                }

                states.push(GlyphAnimationGraphState {
                    name: state.name,
                    animation: load_context.load(state.animation),
                });
            }

            for transition in meta.transitions {
                let from = state_names.get(&transition.from).context(format!(
                    "Invalid state name {} in transition {} -> {}",
                    &transition.from, &transition.from, &transition.to
                ))?;
                let to = state_names.get(&transition.from).context(format!(
                    "Invalid state name {} in transition {} -> {}",
                    &transition.to, &transition.from, &transition.to
                ))?;
                transitions[*from].push(GlyphAnimationGraphTransition {
                    to: *to,
                    animation: transition
                        .animation
                        .map(|animation| load_context.load(animation)),
                });
            }

            Ok(GlyphAnimationGraphSource {
                state_names,
                states,
                transitions,
            })
        })
    }
}

pub struct GlyphAnimationTransition {
    pub transitions: Option<Vec<Handle<GlyphAnimationSource>>>,
    pub destination: Handle<GlyphAnimationSource>,
}

impl GlyphAnimationGraphSource {
    pub fn traverse_named(
        &self,
        src: String,
        dest: String,
    ) -> Result<GlyphAnimationTransition, anyhow::Error> {
        let (src, dest) = (
            *self
                .state_names
                .get(&src)
                .context("Invalid state name {} in transition from {} -> {}")?,
            *self
                .state_names
                .get(&dest)
                .context("Invalid state name {} in transition from {} -> {}")?,
        );

        Ok(self.traverse(src, dest))
    }

    pub fn traverse(&self, src: usize, dest: usize) -> GlyphAnimationTransition {
        let mut path = Vec::with_capacity(2);
        path.push(src);

        GlyphAnimationTransition {
            transitions: self.recursive_traversal(dest, &path, &vec![]),
            destination: self.states[src].animation.clone(),
        }
    }

    fn recursive_traversal(
        &self,
        dest: usize,
        path: &Vec<usize>,
        transition_animations: &Vec<Handle<GlyphAnimationSource>>,
    ) -> Option<Vec<Handle<GlyphAnimationSource>>> {
        let node = *path.last().unwrap();

        if node == dest {
            return Some(transition_animations.clone());
        }

        for child in self.transitions[node].iter() {
            if path.contains(&child.to) {
                continue;
            }
            let mut path = path.clone();
            path.push(child.to);

            let mut transition_animations = transition_animations.clone();
            if let Some(animation) = child.animation.as_ref() {
                transition_animations.push(animation.clone());
            }

            if let Some(path) = self.recursive_traversal(dest, &path, &transition_animations) {
                return Some(path);
            };
        }
        None
    }
}

#[derive(Deserialize)]
struct GlyphAnimationGraphMeta {
    states: Vec<GlyphAnimationGraphStateMeta>,
    transitions: Vec<GlyphAnimationGraphTransitionMeta>,
}

#[derive(Deserialize)]
struct GlyphAnimationGraphStateMeta {
    name: String,
    animation: String,
}

#[derive(Deserialize)]
struct GlyphAnimationGraphTransitionMeta {
    from: String,
    to: String,
    animation: Option<String>,
}

#[derive(Asset, TypePath)]
pub struct GlyphAnimationGraphSource {
    state_names: HashMap<String, usize>,
    states: Vec<GlyphAnimationGraphState>,
    transitions: Vec<Vec<GlyphAnimationGraphTransition>>,
}

struct GlyphAnimationGraphState {
    name: String,
    animation: Handle<GlyphAnimationSource>,
}

#[derive(Clone, Debug)]
pub struct GlyphAnimationGraphTransition {
    to: usize,
    animation: Option<Handle<GlyphAnimationSource>>,
}

#[derive(Debug, Component, Clone)]
pub struct GlyphAnimationGraph {
    pub source: Handle<GlyphAnimationGraphSource>,
}

impl GlyphAnimationGraph {
    pub fn new(graph: Handle<GlyphAnimationGraphSource>) -> Self {
        Self { source: graph }
    }
}
