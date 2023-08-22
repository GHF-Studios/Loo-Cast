use bevy::prelude::*;

#[derive(Component)]
pub struct Background {
    pub background_width: i32,
    pub background_height: i32,
    pub background_chunk_position_x: i32,
    pub background_chunk_position_y: i32,
}
