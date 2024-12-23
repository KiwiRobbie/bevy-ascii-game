use core::f32;

use bevy_ecs::{component::Component, entity::Entity};
use bevy_math::{IVec2, UVec2, Vec2};
use spatial_grid::{direction::Direction, position::Position};

use super::solid::SolidCollisionCache;

#[derive(Component, Default, Clone)]
pub struct Collider {
    pub shape: CompositeCollisionShape,
}

impl RayTest for (&Position, &Collider) {
    fn test_ray(&self, origin: IVec2, direction_inv: Vec2) -> Option<(f32, f32)> {
        let mut min: Option<f32> = None;
        let mut max: Option<f32> = None;

        for (ray_min, ray_max) in self
            .1
            .shape
            .iter_at(self.0 .0)
            .filter_map(|aabb| RayTest::test_ray(&aabb, origin, direction_inv))
        {
            if ray_min.is_finite() {
                if let Some(t_min) = min {
                    min = Some(t_min.min(ray_min));
                } else {
                    min = Some(ray_min)
                };
            }

            if ray_max.is_finite() {
                if let Some(t_max) = max {
                    max = Some(t_max.max(ray_max));
                } else {
                    max = Some(ray_max)
                };
            }
        }

        if let (Some(min), Some(max)) = (min, max) {
            Some((min, max))
        } else {
            None
        }
    }
}

pub trait RayTest {
    fn test_ray(&self, origin: IVec2, direction_inv: Vec2) -> Option<(f32, f32)>;
}

impl RayTest for Aabb {
    fn test_ray(&self, origin: IVec2, direction_inv: Vec2) -> Option<(f32, f32)> {
        let tmin = 0f32;
        let tmax = f32::INFINITY;

        let tx1 = (self.start.x - origin.x) as f32 * direction_inv.x;
        let tx2 = (self.start.x + self.size.x as i32 - origin.x) as f32 * direction_inv.x;
        let ty1 = (self.start.y - origin.y) as f32 * direction_inv.y;
        let ty2 = (self.start.y + self.size.y as i32 - origin.y) as f32 * direction_inv.y;

        let tmin = f32::min(tx1.max(tmin), tx2.max(tmin));
        let tmax = f32::max(tx1.min(tmax), tx2.min(tmax));

        let tmin = f32::min(ty1.max(tmin), ty2.max(tmin));
        let tmax = f32::max(ty1.min(tmax), ty2.min(tmax));

        if tmax >= tmin {
            Some((tmin, tmax))
        } else {
            None
        }
    }
}

pub(crate) trait Overlaps<T> {
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

#[derive(Debug, Clone)]
pub struct CompositeCollisionShape {
    pub shapes: Box<[Aabb]>,
}

impl CompositeCollisionShape {
    pub fn iter_at<'a>(&'a self, offset: IVec2) -> impl Iterator<Item = Aabb> + '_ {
        self.shapes.iter().map(move |shape| shape.translate(offset))
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

impl Into<Collider> for Aabb {
    fn into(self) -> Collider {
        Collider {
            shape: CompositeCollisionShape {
                shapes: Box::new([self]),
            },
        }
    }
}

impl Collider {
    pub(crate) fn overlaps(&self, self_pos: IVec2, other: &SolidCollisionCache) -> Option<Entity> {
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

    pub(crate) fn overlap_distance(
        &self,
        self_pos: IVec2,
        other: &[Aabb],
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

impl From<Aabb> for CompositeCollisionShape {
    fn from(value: Aabb) -> Self {
        Self {
            shapes: Box::new([value]),
        }
    }
}
