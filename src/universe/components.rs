use bevy::prelude::*;
use serde::*;

#[derive(Component, Serialize, Deserialize)]
pub struct Scale {
    pub scale_level: i8,
    pub loaded_chunks: Vec<Chunk>,
}

#[derive(Component, Serialize, Deserialize)]
pub struct Chunk {
    pub scale_level: i8,
    pub universe_object_ids: Vec<i32>,
}

#[derive(Component)]
pub struct UniverseObject {}
