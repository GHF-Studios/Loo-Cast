use bevy::prelude::*;
use serde::*;

#[derive(Component, Serialize, Deserialize)]
pub struct Chunk {
    pub chunk_x: i16,
    pub chunk_y: i16,
    pub scale_level: i8,
    pub stored_entities: Vec<Entity>,
}
