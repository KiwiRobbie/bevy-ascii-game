use bevy::{asset::Handle, ecs::bundle::Bundle};

use super::{
    player::{GlyphAnimationGraphCurrent, GlyphAnimationGraphSettings},
    GlyphAnimationGraph, GlyphAnimationGraphSource,
};

#[derive(Debug, Bundle, Clone)]
pub struct GlyphAnimationGraphBundle {
    pub graph: GlyphAnimationGraph,
    pub current: GlyphAnimationGraphCurrent,
    pub settings: GlyphAnimationGraphSettings,
}
impl GlyphAnimationGraphBundle {
    pub fn from_source(source: Handle<GlyphAnimationGraphSource>) -> Self {
        Self {
            graph: GlyphAnimationGraph { source },
            current: Default::default(),
            settings: Default::default(),
        }
    }
}
