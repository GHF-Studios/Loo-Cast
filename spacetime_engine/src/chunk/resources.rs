use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::chunk::id::structs::*;
use crate::entity::types::*;

use super::structs::ChunkRequest;

#[derive(Resource, Debug, Default)]
pub(in crate) struct ChunkRegistry {
    registered_chunks: HashSet<ChunkID>,
    loaded_chunks: HashMap<ChunkID, EntityReference>,
    serialized_chunks: HashMap<ChunkID, String>,
    allocated_chunks: HashSet<ChunkID>,
    currently_creating_chunks: HashSet<ChunkID>,
    currently_destroying_chunks: HashSet<ChunkID>,
    currently_loading_chunks: HashSet<ChunkID>,
    currently_unloading_chunks: HashSet<ChunkID>,
}

impl ChunkRegistry {
    pub(in crate) fn register_chunk(&mut self, chunk_id: ChunkID) {
        self.registered_chunks.insert(chunk_id);
    }

    pub(in crate) fn unregister_chunk(&mut self, chunk_id: ChunkID) {
        self.registered_chunks.remove(&chunk_id);
    }

    pub(in crate) fn load_chunk(&mut self, chunk_id: ChunkID, chunk_entity_reference: EntityReference) {
        self.loaded_chunks.insert(chunk_id, chunk_entity_reference);
    }

    pub(in crate) fn unload_chunk(&mut self, chunk_id: ChunkID) -> Option<EntityReference> {
        self.loaded_chunks.remove(&chunk_id)
    }

    pub(in crate) fn serialize_chunk(&mut self, chunk_id: ChunkID, serialized_chunk: String) {
        self.serialized_chunks.insert(chunk_id, serialized_chunk);
    }

    pub(in crate) fn deserialize_chunk(&mut self, chunk_id: ChunkID) -> Option<String> {
        self.serialized_chunks.remove(&chunk_id)
    }

    pub(in crate) fn start_creating_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_creating_chunks.insert(chunk_id);
    }

    pub(in crate) fn stop_creating_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_creating_chunks.remove(&chunk_id);
    }

    pub(in crate) fn start_destroying_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_destroying_chunks.insert(chunk_id);
    }

    pub(in crate) fn stop_destroying_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_destroying_chunks.remove(&chunk_id);
    }

    pub(in crate) fn start_loading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_loading_chunks.insert(chunk_id);
    }

    pub(in crate) fn stop_loading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_loading_chunks.remove(&chunk_id);
    }

    pub(in crate) fn start_unloading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_unloading_chunks.insert(chunk_id);
    }

    pub(in crate) fn stop_unloading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_unloading_chunks.remove(&chunk_id);
    }

    pub(in crate) fn allocate_chunk(&mut self, chunk_id: ChunkID) {
        if !self.allocated_chunks.contains(&chunk_id) {
            self.allocated_chunks.insert(chunk_id);

            debug!("Allocated chunk '{:?}'", chunk_id);
        } else {
            panic!("Chunk with ID {:?} is already allocated", chunk_id);
        }
    }

    pub(in crate) fn try_allocate_chunk(&mut self, chunk_id: ChunkID) -> bool {
        if !self.allocated_chunks.contains(&chunk_id) {
            self.allocated_chunks.insert(chunk_id);
            
            debug!("Successfully allocated chunk '{:?}'", chunk_id);

            true
        } else {

            false
        }
    }

    pub(in crate) fn deallocate_chunk(&mut self, chunk_id: ChunkID) {
        if self.allocated_chunks.contains(&chunk_id) {
            self.allocated_chunks.remove(&chunk_id);

            debug!("Deallocated chunk '{:?}'", chunk_id);
        } else {
            panic!("Chunk with ID {:?} is not allocated", chunk_id);
        }
    }

    pub(in crate) fn try_deallocate_chunk(&mut self, chunk_id: ChunkID) -> bool {
        if self.allocated_chunks.contains(&chunk_id) {
            self.allocated_chunks.remove(&chunk_id);

            debug!("Successfully deallocated chunk '{:?}'", chunk_id);

            true
        } else {
            false
        }
    }

    pub(in crate) fn is_chunk_registered(&self, chunk_id: ChunkID) -> bool {
        self.registered_chunks.contains(&chunk_id)
    }

    pub(in crate) fn is_chunk_loaded(&self, chunk_id: ChunkID) -> bool {
        self.loaded_chunks.contains_key(&chunk_id)
    }

    pub(in crate) fn is_chunk_serialized(&self, chunk_id: ChunkID) -> bool {
        self.serialized_chunks.contains_key(&chunk_id)
    }

    pub(in crate) fn is_creating_chunk(&self, chunk_id: ChunkID) -> bool {
        self.currently_creating_chunks.contains(&chunk_id)
    }

    pub(in crate) fn is_destroying_chunk(&self, chunk_id: ChunkID) -> bool {
        self.currently_destroying_chunks.contains(&chunk_id)
    }

    pub(in crate) fn is_loading_chunk(&self, chunk_id: ChunkID) -> bool {
        self.currently_loading_chunks.contains(&chunk_id)
    }

    pub(in crate) fn is_unloading_chunk(&self, chunk_id: ChunkID) -> bool {
        self.currently_unloading_chunks.contains(&chunk_id)
    }

    pub(in crate) fn is_chunk_allocated(&self, chunk_id: ChunkID) -> bool {
        self.allocated_chunks.contains(&chunk_id)
    }

    pub(in crate) fn registered_chunks(&self) -> &HashSet<ChunkID> {
        &self.registered_chunks
    }

    pub(in crate) fn registered_chunks_mut(&mut self) -> &mut HashSet<ChunkID> {
        &mut self.registered_chunks
    }

    pub(in crate) fn get_loaded_chunk_entity(&self, chunk_id: ChunkID) -> Option<EntityReference> {
        self.loaded_chunks.get(&chunk_id).copied()
    }

    pub(in crate) fn loaded_chunk_entity(&self, chunk_id: ChunkID) -> EntityReference {
        self.loaded_chunks[&chunk_id]
    }

    pub(in crate) fn loaded_chunks(&self) -> &HashMap<ChunkID, EntityReference> {
        &self.loaded_chunks
    }

    pub(in crate) fn loaded_chunks_mut(&mut self) -> &mut HashMap<ChunkID, EntityReference> {
        &mut self.loaded_chunks
    }

    pub(in crate) fn loaded_chunk_ids(&self) -> HashSet<ChunkID> {
        self.loaded_chunks.keys().copied().collect()
    }

    pub(in crate) fn loaded_chunk_entities(&self) -> HashSet<EntityReference> {
        self.loaded_chunks.values().copied().collect()
    }

    pub(in crate) fn serialized_chunks(&self) -> &HashMap<ChunkID, String> {
        &self.serialized_chunks
    }

    pub(in crate) fn serialized_chunks_mut(&mut self) -> &mut HashMap<ChunkID, String> {
        &mut self.serialized_chunks
    }

    pub(in crate) fn creating_chunks(&self) -> &HashSet<ChunkID> {
        &self.currently_creating_chunks
    }

    pub(in crate) fn destroying_chunks(&self) -> &HashSet<ChunkID> {
        &self.currently_destroying_chunks
    }

    pub(in crate) fn loading_chunks(&self) -> &HashSet<ChunkID> {
        &self.currently_loading_chunks
    }

    pub(in crate) fn unloading_chunks(&self) -> &HashSet<ChunkID> {
        &self.currently_unloading_chunks
    }

    pub(in crate) fn allocated_chunks(&self) -> &HashSet<ChunkID> {
        &self.allocated_chunks
    }

    pub(in crate) fn allocated_chunks_mut(&mut self) -> &mut HashSet<ChunkID> {
        &mut self.allocated_chunks
    }
}

#[derive(Resource, Debug, Default)]
pub(in crate) struct ChunkRequestRegistry {
    registered_chunk_requests: HashSet<ChunkRequestID>,
    loaded_chunk_requests: HashMap<ChunkRequestID, ChunkRequest>,
    next_chunk_request_id: ChunkRequestID,
}

impl ChunkRequestRegistry {
    pub(in crate) fn register_chunk_request(&mut self, chunk_request_id: ChunkRequestID) {
        self.registered_chunk_requests.insert(chunk_request_id);

        trace!("Registered chunk request '{:?}'", chunk_request_id);
    }

    pub(in crate) fn unregister_chunk_request(&mut self, chunk_request_id: ChunkRequestID) {
        self.registered_chunk_requests.remove(&chunk_request_id);

        trace!("Unregistered chunk request '{:?}'", chunk_request_id);
    }

    pub(in crate) fn load_chunk_request(&mut self, chunk_request_id: ChunkRequestID, chunk_request: ChunkRequest) {
        self.loaded_chunk_requests.insert(chunk_request_id, chunk_request);

        trace!("Loaded chunk request '{:?}'", chunk_request_id);
    }

    pub(in crate) fn unload_chunk_request(&mut self, chunk_request_id: ChunkRequestID) -> Option<ChunkRequest> {
        let removed_chunk_request = self.loaded_chunk_requests.remove(&chunk_request_id);

        trace!("Unloaded chunk request '{:?}'", chunk_request_id);

        removed_chunk_request
    }

    pub(in crate) fn is_chunk_request_registered(&self, chunk_request_id: ChunkRequestID) -> bool {
        self.registered_chunk_requests.contains(&chunk_request_id)
    }

    pub(in crate) fn is_chunk_request_loaded(&self, chunk_request_id: ChunkRequestID) -> bool {
        self.loaded_chunk_requests.contains_key(&chunk_request_id)
    }

    pub(in crate) fn registered_chunk_requests(&self) -> &HashSet<ChunkRequestID> {
        &self.registered_chunk_requests
    }

    pub(in crate) fn registered_chunk_requests_mut(&mut self) -> &mut HashSet<ChunkRequestID> {
        &mut self.registered_chunk_requests
    }

    pub(in crate) fn get_loaded_chunk_request(&self, chunk_request_id: ChunkRequestID) -> Option<ChunkRequest> {
        self.loaded_chunk_requests.get(&chunk_request_id).copied()
    }

    pub(in crate) fn get_loaded_chunk_request_id(&self, chunk_request: &ChunkRequest) -> Option<ChunkRequestID> {
        self.loaded_chunk_requests.iter().find(|(_, r)| **r == *chunk_request).map(|(id, _)| *id)
    }

    pub(in crate) fn loaded_chunk_request(&self, chunk_request_id: ChunkRequestID) -> ChunkRequest {
        self.loaded_chunk_requests[&chunk_request_id]
    }

    pub(in crate) fn loaded_chunk_request_id(&self, chunk_request: ChunkRequest) -> ChunkRequestID {
        self.loaded_chunk_requests.iter().find(|(_, r)| **r == chunk_request).map(|(id, _)| *id).unwrap()
    }

    pub(in crate) fn loaded_chunk_requests(&self) -> &HashMap<ChunkRequestID, ChunkRequest> {
        &self.loaded_chunk_requests
    }

    pub(in crate) fn loaded_chunk_requests_mut(&mut self) -> &mut HashMap<ChunkRequestID, ChunkRequest> {
        &mut self.loaded_chunk_requests
    }

    pub fn get_unused_chunk_request_id(&mut self) -> ChunkRequestID {
        let chunk_request_id = self.next_chunk_request_id;
        self.next_chunk_request_id = ChunkRequestID(chunk_request_id.0 + 1);

        chunk_request_id
    }
}