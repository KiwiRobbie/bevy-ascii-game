use bevy::{
    ecs::{component::Component, entity::Entity},
    math::{IVec2, UVec2},
};

use super::{direction::Direction, solid::SolidCollisionCache};

#[derive(Component, Default, Clone)]
pub struct Collider {
    pub shape: CollisionShape,
}

#[derive(Debug, Clone)]
pub enum CollisionShape {
    Aabb(Aabb),
    Composite(Box<[CollisionShape]>),
}

impl CollisionShape {
    pub fn colliders<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Aabb> + 'a> {
        match self {
            Self::Aabb(aabb) => Box::new(std::iter::once(aabb)),
            Self::Composite(colliders) => {
                Box::new(colliders.iter().flat_map(|collider| collider.colliders()))
            }
        }
    }

    pub fn colliders_at<'a>(&'a self, offset: IVec2) -> Box<dyn Iterator<Item = Aabb> + 'a> {
        Box::new(self.colliders().map(move |aabb| Aabb {
            min: aabb.min + offset,
            size: aabb.size,
        }))
    }
}

impl Default for CollisionShape {
    fn default() -> Self {
        Self::Composite(Box::new([]))
    }
}

#[derive(Default, Debug, Clone)]
pub struct Aabb {
    pub min: IVec2,
    pub size: UVec2,
}

impl Aabb {
    pub fn contains(&self, point: IVec2) -> bool {
        self.min.cmple(point).all() && point.cmplt(self.min + self.size.as_ivec2()).all()
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        let other_ends_before = self.min.cmpge(other.min + other.size.as_ivec2());
        let other_starts_after = (self.min + self.size.as_ivec2()).cmple(other.min);
        let outside = other_ends_before | other_starts_after;
        !(outside.any())
    }

    pub fn overlap_distance(&self, other: &Self, direction: Direction) -> Option<i32> {
        if self.overlaps(other) {
            Some(match direction {
                Direction::X => other.min.x + other.size.x as i32 - self.min.x,
                Direction::Y => self.min.y + self.size.y as i32 - other.min.y,
                Direction::NegX => self.min.x + self.size.x as i32 - other.min.x,
                Direction::NegY => self.min.y - other.min.y - other.size.y as i32,
            })
        } else {
            None
        }
    }
}

impl Collider {
    pub fn overlaps(&self, self_pos: IVec2, other: &SolidCollisionCache) -> Option<Entity> {
        let self_colliders = self.shape.colliders_at(self_pos);

        for actor_aabb in self_colliders.into_iter() {
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
        other: &[Aabb],
        direction: Direction,
    ) -> Option<i32> {
        let mut overlap = None;

        let self_colliders = self.shape.colliders_at(self_pos);
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

pub struct OffsetCollision<'a> {
    pub shape: &'a CollisionShape,
    pub offset: IVec2,
}
