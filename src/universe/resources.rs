use super::components::*;

use bevy::prelude::*;

#[derive(Resource)]
pub struct UniverseManager {
    pub current_scale_level: i8,
    pub current_chunk_offset_x: i16,
    pub current_chunk_offset_y: i16,
}
