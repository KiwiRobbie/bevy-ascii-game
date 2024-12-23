use bevy::{
    ecs::component::Component,
    math::{IVec2, UVec2},
};

#[derive(Debug, Component, Clone)]
pub(crate) struct ClipRegion {
    pub(crate) start: IVec2,
    pub(crate) size: UVec2,
}
impl ClipRegion {
    pub(crate) fn intersection(&self, other: &Self) -> Self {
        let start = self.start.max(other.start);

        let self_end = self.start + self.size.as_ivec2();
        let other_end = other.start + other.size.as_ivec2();
        let end = self_end.min(other_end);

        let size = (end - start).min(IVec2::ZERO).as_uvec2();

        Self { start, size }
    }
    pub(crate) fn to_world_coord(&self) -> Self {
        return Self {
            start: IVec2::new(1, -1) * self.start - IVec2::Y * self.size.y as i32,
            size: self.size,
        };
    }
    pub(crate) fn contains(&self, pos: IVec2) -> bool {
        self.start.cmple(pos).all() && pos.cmplt(self.start + self.size.as_ivec2()).all()
    }
}
