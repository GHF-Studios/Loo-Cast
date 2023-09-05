use super::{LocalChunkPosition, CHUNK_SIZE};

use bevy::prelude::*;
use image::*;
use serde::*;
use std::collections::HashSet;

#[derive(Component, Serialize, Deserialize)]
pub struct Chunk {
    pub pos: LocalChunkPosition,
    pub scale_level: i8,
    pub stored_entities: Vec<Entity>,
}

#[derive(Component, Serialize, Deserialize)]
pub struct UniverseObserver {
    pub observing_distance: i16,
    pub old_proximal_chunk_coordinates: HashSet<LocalChunkPosition>,
}

impl UniverseObserver {
    pub fn new(observing_distance: i16) -> Self {
        Self {
            observing_distance,
            old_proximal_chunk_coordinates: HashSet::new(),
        }
    }

    pub fn get_proximal_chunk_coordinates(&self, x: f32, y: f32) -> HashSet<LocalChunkPosition> {
        let mut proximal_chunk_coordinates = HashSet::new();

        let chunk_x = (x / CHUNK_SIZE as f32).floor() as i16;
        let chunk_y = (y / CHUNK_SIZE as f32).floor() as i16;

        for x_offset in -self.observing_distance..=self.observing_distance {
            for y_offset in -self.observing_distance..=self.observing_distance {
                proximal_chunk_coordinates.insert(LocalChunkPosition {
                    x: chunk_x + x_offset,
                    y: chunk_y + y_offset,
                });
            }
        }

        proximal_chunk_coordinates
    }
}
