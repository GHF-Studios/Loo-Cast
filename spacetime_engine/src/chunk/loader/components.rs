use bevy::prelude::*;
use crate::chunk::id::structs::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChunkLoader {
    pub(in crate) load_radius: u16,
    pub(in crate) current_chunk_ids: Vec<ChunkID>,
}