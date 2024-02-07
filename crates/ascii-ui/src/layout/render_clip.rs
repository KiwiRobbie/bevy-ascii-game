use bevy::{
    ecs::component::Component,
    math::{IVec2, UVec2},
};

#[derive(Debug, Component, Clone)]
pub struct ClipRegion {
    pub start: IVec2,
    pub size: UVec2,
}
impl ClipRegion {
    pub fn intersection(&self, other: &Self) -> Self {
        let start = self.start.max(other.start);

        let self_end = self.start + self.size.as_ivec2();
        let other_end = other.start + other.size.as_ivec2();
        let end = self_end.min(other_end);

        let size = (end - start).min(IVec2::ZERO).as_uvec2();

        Self { start, size }
    }
    pub fn to_world_coord(&self) -> Self {
        return Self {
            start: IVec2::new(1, -1) * self.start - IVec2::Y * self.size.y as i32,
            size: self.size,
        };
    }
    pub fn contains(&self, pos: IVec2) -> bool {
        self.start.cmple(pos).all() && pos.cmplt(self.start + self.size.as_ivec2()).all()
    }
}
