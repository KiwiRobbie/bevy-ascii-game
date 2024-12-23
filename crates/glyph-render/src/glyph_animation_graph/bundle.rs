use bevy::{asset::Handle, ecs::bundle::Bundle};

use super::{
    player::{GlyphAnimationGraphCurrent, GlyphAnimationGraphSettings, GlyphAnimationGraphTarget},
    GlyphAnimationGraph, GlyphAnimationGraphSource,
};

#[derive(Debug, Bundle, Clone)]
pub struct GlyphAnimationGraphBundle {
    pub graph: GlyphAnimationGraph,
    pub current: GlyphAnimationGraphCurrent,
    pub settings: GlyphAnimationGraphSettings,
    pub target: GlyphAnimationGraphTarget,
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
