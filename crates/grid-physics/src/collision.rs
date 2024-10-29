use std::collections::linked_list::Iter;

use bevy_ecs::{component::Component, entity::Entity};
use bevy_math::{IVec2, UVec2};
use spatial_grid::direction::Direction;

use super::solid::SolidCollisionCache;

#[derive(Component, Default, Clone)]
pub struct Collider {
    pub shape: CompositeCollisionShape,
}

#[derive(Debug, Clone)]
pub enum CollisionShape {
    Aabb(Aabb),
    HalfPlane(HalfPlane),
}

impl Overlaps<&CollisionShape> for CollisionShape {
    fn overlaps(&self, other: &CollisionShape) -> bool {
        match (self, other) {
            (CollisionShape::Aabb(a), CollisionShape::Aabb(b)) => a.overlaps(b),
            (CollisionShape::Aabb(a), CollisionShape::HalfPlane(b)) => a.overlaps(b),
            (CollisionShape::HalfPlane(a), CollisionShape::Aabb(b)) => a.overlaps(b),
            (CollisionShape::HalfPlane(a), CollisionShape::HalfPlane(b)) => a.overlaps(b),
        }
    }

    fn overlap_distance(&self, other: &CollisionShape, direction: Direction) -> Option<i32> {
        match (self, other) {
            (CollisionShape::Aabb(a), CollisionShape::Aabb(b)) => a.overlap_distance(b, direction),
            (CollisionShape::Aabb(a), CollisionShape::HalfPlane(b)) => {
                a.overlap_distance(b, direction)
            }
            (CollisionShape::HalfPlane(a), CollisionShape::Aabb(b)) => {
                a.overlap_distance(b, direction)
            }
            (CollisionShape::HalfPlane(a), CollisionShape::HalfPlane(b)) => {
                a.overlap_distance(b, direction)
            }
        }
    }
}

pub trait Overlaps<T> {
    fn overlaps(&self, other: T) -> bool;
    fn overlap_distance(&self, other: T, direction: Direction) -> Option<i32>;
}

impl Overlaps<&Aabb> for Aabb {
    fn overlaps(&self, other: &Aabb) -> bool {
        let other_ends_before = self.start.cmpge(other.start + other.size.as_ivec2());
        let other_starts_after = (self.start + self.size.as_ivec2()).cmple(other.start);
        let outside = other_ends_before | other_starts_after;
        !(outside.any())
    }

    fn overlap_distance(&self, other: &Self, direction: Direction) -> Option<i32> {
        if self.overlaps(other) {
            Some(match direction {
                Direction::PosX => other.start.x + other.size.x as i32 - self.start.x,
                Direction::PosY => self.start.y + self.size.y as i32 - other.start.y,
                Direction::NegX => self.start.x + self.size.x as i32 - other.start.x,
                Direction::NegY => self.start.y - other.start.y - other.size.y as i32,
            })
        } else {
            None
        }
    }
}

impl Overlaps<&HalfPlane> for Aabb {
    fn overlaps(&self, other: &HalfPlane) -> bool {
        let end = self.start + self.size.as_ivec2();
        match other {
            HalfPlane::NegX { x } => self.start.x <= *x,
            HalfPlane::NegY { y } => self.start.y <= *y,
            HalfPlane::PosX { x } => end.x > *x,
            HalfPlane::PosY { y } => end.y > *y,
        }
    }
    fn overlap_distance(&self, other: &HalfPlane, direction: Direction) -> Option<i32> {
        match other.normal().compare(&direction) {
            spatial_grid::direction::DirectionCompare::Equal => {}
            spatial_grid::direction::DirectionCompare::Perpendicular => return None,
            spatial_grid::direction::DirectionCompare::Opposite => {}
        }

        return None;
        // match other {
        //     HalfPlane::NegX { x } => self.start.x <= *x,
        //     HalfPlane::NegY { y } => self.start.y <= *y,
        //     HalfPlane::PosX { x } => end.x > *x,
        //     HalfPlane::PosY { y } => end.y > *y,
        // }
    }
}

impl Overlaps<&Aabb> for HalfPlane {
    fn overlaps(&self, other: &Aabb) -> bool {
        other.overlaps(self)
    }
    fn overlap_distance(&self, other: &Aabb, direction: Direction) -> Option<i32> {
        Aabb::overlap_distance(&other, self, direction)
    }
}

impl Overlaps<&HalfPlane> for HalfPlane {
    fn overlaps(&self, other: &HalfPlane) -> bool {
        match (self, other) {
            // Opposite directions might overlap
            (HalfPlane::NegX { x: a }, HalfPlane::PosX { x: b }) => a >= b,
            (HalfPlane::PosX { x: a }, HalfPlane::NegX { x: b }) => a <= b,
            (HalfPlane::NegY { y: a }, HalfPlane::PosY { y: b }) => a >= b,
            (HalfPlane::PosY { y: a }, HalfPlane::NegY { y: b }) => a <= b,

            // Same or perpendicular directions will overlap
            _ => true,
        }
    }

    fn overlap_distance(&self, other: &Self, direction: Direction) -> Option<i32> {
        None
    }
}

#[derive(Debug, Clone)]
pub enum HalfPlane {
    NegX { x: i32 },
    PosX { x: i32 },
    NegY { y: i32 },
    PosY { y: i32 },
}

impl HalfPlane {
    pub fn translate(&self, offset: IVec2) -> Self {
        match self {
            Self::NegX { x } => Self::NegX { x: x + offset.x },
            Self::PosX { x } => Self::PosX { x: x + offset.x },
            Self::NegY { y } => Self::NegY { y: y + offset.y },
            Self::PosY { y } => Self::PosY { y: y + offset.y },
        }
    }
    pub fn normal(&self) -> Direction {
        match self {
            HalfPlane::NegX { x } => Direction::NegX,
            HalfPlane::PosX { x } => Direction::PosX,
            HalfPlane::NegY { y } => Direction::NegY,
            HalfPlane::PosY { y } => Direction::PosY,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompositeCollisionShape {
    pub shapes: Box<[CollisionShape]>,
}

impl CompositeCollisionShape {
    pub fn iter_at<'a>(&'a self, offset: IVec2) -> Box<dyn Iterator<Item = CollisionShape> + 'a> {
        Box::new(self.shapes.iter().map(move |shape| match shape {
            CollisionShape::Aabb(aabb) => CollisionShape::Aabb(aabb.translate(offset)),
            CollisionShape::HalfPlane(half_plane) => {
                CollisionShape::HalfPlane(half_plane.translate(offset))
            }
        }))
    }
}

impl Default for CompositeCollisionShape {
    fn default() -> Self {
        Self {
            shapes: Box::new([]),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Aabb {
    pub start: IVec2,
    pub size: UVec2,
}

impl Aabb {
    pub fn contains(&self, point: IVec2) -> bool {
        self.start.cmple(point).all() && point.cmplt(self.start + self.size.as_ivec2()).all()
    }

    pub fn translate(&self, offset: IVec2) -> Self {
        Self {
            start: self.start + offset,
            size: self.size,
        }
    }
}

impl Collider {
    pub fn overlaps(&self, self_pos: IVec2, other: &SolidCollisionCache) -> Option<Entity> {
        let self_colliders = self.shape.iter_at(self_pos);

        for actor_aabb in self_colliders {
            for (solid, solid_aabb) in other
                .collisions
                .iter()
                .flat_map(|(solid, collider)| collider.iter().map(|aabb| (*solid, aabb)))
            {
                if actor_aabb.overlaps(solid_aabb) {
                    return Some(solid);
                }
            }
        }
        None
    }

    pub fn overlap_distance(
        &self,
        self_pos: IVec2,
        other: &[CollisionShape],
        direction: Direction,
    ) -> Option<i32> {
        let mut overlap = None;

        let self_colliders = self.shape.iter_at(self_pos);
        for a in self_colliders.into_iter() {
            for b in other.iter() {
                if let Some(new_distance) = a.overlap_distance(b, direction) {
                    if let Some(distance) = overlap {
                        if distance < new_distance {
                            overlap = Some(new_distance);
                        }
                    } else {
                        overlap = Some(new_distance);
                    }
                }
            }
        }
        overlap
    }
}

impl From<CollisionShape> for CompositeCollisionShape {
    fn from(value: CollisionShape) -> Self {
        Self {
            shapes: Box::new([value]),
        }
    }
}
impl From<Aabb> for CompositeCollisionShape {
    fn from(value: Aabb) -> Self {
        Self {
            shapes: Box::new([CollisionShape::Aabb(value)]),
        }
    }
}
