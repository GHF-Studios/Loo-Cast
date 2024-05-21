use bevy::prelude::*;
use crate::chunk::id::structs::*;

use super::structs::ChunkLoaderID;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChunkLoader {
    id: ChunkLoaderID,
    load_radius: u16,
    current_chunk_ids: Vec<ChunkID>,
}

impl ChunkLoader {
    pub fn new(id: ChunkLoaderID, load_radius: u16) -> Self {
        Self {
            id,
            load_radius,
            current_chunk_ids: Vec::new(),
        }
    }

    pub fn id(&self) -> ChunkLoaderID {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut ChunkLoaderID {
        &mut self.id
    }

    pub fn load_radius(&self) -> u16 {
        self.load_radius
    }

    pub(in crate) fn load_radius_mut(&mut self) -> &mut u16 {
        &mut self.load_radius
    }

    pub fn current_chunk_ids(&self) -> &Vec<ChunkID> {
        &self.current_chunk_ids
    }

    pub(in crate) fn current_chunk_ids_mut(&mut self) -> &mut Vec<ChunkID> {
        &mut self.current_chunk_ids
    }
}