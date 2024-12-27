use bevy::prelude::*;

use super::{
    player::{GlyphAnimationGraphCurrent, GlyphAnimationGraphSettings, GlyphAnimationGraphTarget},
    GlyphAnimationGraph, GlyphAnimationGraphSource,
};

#[derive(Debug, Bundle, Clone)]
pub struct GlyphAnimationGraphBundle {
    pub(crate) graph: GlyphAnimationGraph,
    pub(crate) current: GlyphAnimationGraphCurrent,
    pub(crate) settings: GlyphAnimationGraphSettings,
    pub(crate) target: GlyphAnimationGraphTarget,
}
impl GlyphAnimationGraphBundle {
    pub fn from_source(source: Handle<GlyphAnimationGraphSource>) -> Self {
        Self {
            graph: GlyphAnimationGraph { source },
            current: Default::default(),
            settings: Default::default(),
            target: Default::default(),
        }
    }
}
