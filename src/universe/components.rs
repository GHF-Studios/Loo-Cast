use std::collections::HashSet;

use super::CHUNK_SIZE;

use bevy::prelude::*;
use serde::*;

#[derive(Component, Serialize, Deserialize)]
pub struct Chunk {
    pub chunk_x: i16,
    pub chunk_y: i16,
    pub scale_level: i8,
    pub stored_entities: Vec<Entity>,
}

#[derive(Component, Serialize, Deserialize)]
pub struct UniverseObserver {
    pub observing_distance: i16,
    pub old_proximal_chunk_coordinates: HashSet<(i16, i16)>,
}

impl UniverseObserver {
    pub fn new(observing_distance: i16) -> Self {
        Self {
            observing_distance,
            old_proximal_chunk_coordinates: HashSet::new(),
            // Initialize other fields here
        }
    }

    pub fn get_proximal_chunk_coordinates(&self, x: f32, y: f32) -> HashSet<(i16, i16)> {
        let mut proximal_chunk_coordinates = HashSet::new();
    
        let chunk_x = (x / CHUNK_SIZE as f32).floor() as i16;
        let chunk_y = (y / CHUNK_SIZE as f32).floor() as i16;
    
        for x_offset in -self.observing_distance..=self.observing_distance {
            for y_offset in -self.observing_distance..=self.observing_distance {
                proximal_chunk_coordinates.insert((chunk_x + x_offset, chunk_y + y_offset));
            }
        }
    
        proximal_chunk_coordinates
    }
    
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalEntityPosition {
    pub chunk_position: GlobalChunkPosition,
    pub offset_x: f32,
    pub offset_y: f32,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalChunkPosition {
    pub scale_level: i8,
    pub parent_x: i16,
    pub parent_y: i16,
    pub x: i16,
    pub y: i16,
}