use bevy::prelude::*;

pub(in crate) fn calculate_chunks_in_range(position: Vec2, range: u32) -> Vec<(i32, i32)> {
    unimplemented!("Returns all chunk coordinates within a given range from the position");
}

pub(in crate) fn grid_position(position: Vec2) -> (i32, i32) {
    unimplemented!("Converts world space position to a grid-based chunk coordinate");
}