use bevy::prelude::*;
use spatial_grid::position::Position;

#[derive(Debug, Component)]
#[require(Position)]
pub struct EditorCursor;

// fn update_cursor(q_cursor: Quer) {}
