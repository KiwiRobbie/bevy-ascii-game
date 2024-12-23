use bevy_ecs::component::Component;

#[derive(Debug, Component, Clone, Copy)]
pub enum Direction {
    PosX,
    PosY,
    NegX,
    NegY,
}

pub(crate) enum DirectionCompare {
    Equal,
    Perpendicular,
    Opposite,
}
impl Direction {
    pub(crate) fn compare(&self, other: &Self) -> DirectionCompare {
        match (self, other) {
            (Direction::NegX, Direction::NegX) => DirectionCompare::Equal,
            (Direction::PosX, Direction::PosX) => DirectionCompare::Equal,
            (Direction::NegY, Direction::NegY) => DirectionCompare::Equal,
            (Direction::PosY, Direction::PosY) => DirectionCompare::Equal,

            (Direction::PosX, Direction::NegX) => DirectionCompare::Opposite,
            (Direction::NegX, Direction::PosX) => DirectionCompare::Opposite,
            (Direction::NegY, Direction::PosY) => DirectionCompare::Opposite,
            (Direction::PosY, Direction::NegY) => DirectionCompare::Opposite,

            (Direction::PosX, Direction::PosY) => DirectionCompare::Perpendicular,
            (Direction::PosX, Direction::NegY) => DirectionCompare::Perpendicular,
            (Direction::PosY, Direction::PosX) => DirectionCompare::Perpendicular,
            (Direction::PosY, Direction::NegX) => DirectionCompare::Perpendicular,
            (Direction::NegX, Direction::PosY) => DirectionCompare::Perpendicular,
            (Direction::NegX, Direction::NegY) => DirectionCompare::Perpendicular,
            (Direction::NegY, Direction::PosX) => DirectionCompare::Perpendicular,
            (Direction::NegY, Direction::NegX) => DirectionCompare::Perpendicular,
        }
    }
}
