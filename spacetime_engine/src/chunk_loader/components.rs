use bevy::prelude::*;
use crate::chunk::components::*;
use crate::operations::structs::*;
use crate::chunk_loader::structs::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChunkLoader {
    id: InstanceID<ChunkLoader>,
    load_radius: u16,
    registered_chunks: Vec<RegisteredChunkInfo>,
}
impl ChunkLoader {
    pub fn new(load_radius: u16) -> Self {
        Self {
            id: InstanceID::default(),
            load_radius,
            registered_chunks: Vec::new(),
        }
    }

    pub fn id(&self) -> InstanceID<ChunkLoader> {
        self.id
    }

    pub(in crate) fn id_mut(&mut self) -> &mut InstanceID<ChunkLoader> {
        &mut self.id
    }

    pub fn load_radius(&self) -> u16 {
        self.load_radius
    }

    pub fn load_radius_mut(&mut self) -> &mut u16 {
        &mut self.load_radius
    }

    pub fn registered_chunks(&self) -> &Vec<RegisteredChunkInfo> {
        &self.registered_chunks
    }

    pub(in crate) fn registered_chunks_mut(&mut self) -> &mut Vec<RegisteredChunkInfo> {
        &mut self.registered_chunks
    }

    pub fn register_unmanaged_chunk(&mut self, chunk_id: InstanceID<Chunk>) {
        self.registered_chunks.push(RegisteredChunkInfo::Unmanaged(chunk_id));
    }
    
    pub fn register_managed_chunk(&mut self, chunk_id: InstanceID<Chunk>) {
        self.registered_chunks.push(RegisteredChunkInfo::Managed(chunk_id));
    }

    pub fn unregister_chunk(&mut self, chunk_id: InstanceID<Chunk>) {
        self.registered_chunks.retain(|info| {
            match info {
                RegisteredChunkInfo::Unmanaged(id) => id != &chunk_id,
                RegisteredChunkInfo::Managed(id) => id != &chunk_id,
            }
        });
    }

    pub fn manage_chunk(&mut self, chunk_id: InstanceID<Chunk>) {
        if !self.is_chunk_registered(chunk_id) {
            panic!("Chunk '{:?}' is not registered!", chunk_id);
        }
        if self.is_chunk_managed(chunk_id) {
            panic!("Chunk '{:?}' is already managed!", chunk_id);
        }

        self.unregister_chunk(chunk_id);
        self.register_managed_chunk(chunk_id);
    }

    pub fn unmanage_chunk(&mut self, chunk_id: InstanceID<Chunk>) {
        if !self.is_chunk_registered(chunk_id) {
            panic!("Chunk '{:?}' is not registered!", chunk_id);
        }
        if !self.is_chunk_managed(chunk_id) {
            panic!("Chunk '{:?}' is already unmanaged!", chunk_id);
        }

        self.unregister_chunk(chunk_id);
        self.register_unmanaged_chunk(chunk_id);
    }

    pub fn is_chunk_registered(&self, chunk_id: InstanceID<Chunk>) -> bool {
        self.registered_chunks.iter().any(|info| {
            match info {
                RegisteredChunkInfo::Unmanaged(id) => id == &chunk_id,
                RegisteredChunkInfo::Managed(id) => id == &chunk_id,
            }
        })
    }

    pub fn is_chunk_managed(&self, chunk_id: InstanceID<Chunk>) -> bool {
        self.registered_chunks.iter().any(|info| {
            match info {
                RegisteredChunkInfo::Managed(id) => id == &chunk_id,
                _ => false,
            }
        })
    }
}