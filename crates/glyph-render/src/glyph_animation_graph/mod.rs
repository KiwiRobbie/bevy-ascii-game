use std::collections::VecDeque;

use anyhow::Context;
use bevy::{
    asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, Handle},
    ecs::component::Component,
    reflect::TypePath,
    utils::hashbrown::{HashMap, HashSet},
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
                    // name: state.name,
                    animation: load_context.load(state.animation),
                });
            }

            for transition in meta.transitions {
                let from = state_names.get(&transition.from).context(format!(
                    "Invalid state name {} in transition {} -> {}",
                    &transition.from, &transition.from, &transition.to
                ))?;
                let to = state_names.get(&transition.to).context(format!(
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
        let mut visited: HashSet<usize> = HashSet::new();
        let mut queue = VecDeque::<(usize, Vec<Handle<GlyphAnimationSource>>)>::new();

        queue.push_back((src, vec![]));
        while let Some((node, path)) = queue.pop_front() {
            if node == dest {
                return GlyphAnimationTransition {
                    transitions: Some(path),
                    destination: self.states[dest].animation.clone(),
                };
            }
            visited.insert(node);
            for transition in &self.transitions[node] {
                let child = transition.to;
                if !visited.contains(&transition.to) {
                    let mut path = path.clone();
                    if let Some(animation) = &transition.animation {
                        path.push(animation.clone());
                    }

                    queue.push_back((child, path));
                }
            }
        }

        GlyphAnimationTransition {
            transitions: None,
            destination: self.states[dest].animation.clone(),
        }
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
    #[serde(default, deserialize_with = "wrap_some")]
    animation: Option<String>,
}

fn wrap_some<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    Ok(Some(s))
}

#[derive(Asset, TypePath)]
pub struct GlyphAnimationGraphSource {
    state_names: HashMap<String, usize>,
    states: Vec<GlyphAnimationGraphState>,
    transitions: Vec<Vec<GlyphAnimationGraphTransition>>,
}

struct GlyphAnimationGraphState {
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
