use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::chunk::id::structs::*;
use crate::entity::id::structs::EntityRequestID;
use crate::entity::types::*;

use super::structs::ChunkRequest;

#[derive(Resource, Debug, Default)]
pub(in crate) struct ChunkRegistry {
    registered_chunks: HashSet<ChunkID>,
    loaded_chunks: HashMap<ChunkID, EntityReference>,
    serialized_chunks: HashMap<ChunkID, String>,
    currently_upgrading_to_chunks: HashSet<ChunkID>,
    currently_downgrading_from_chunks: HashSet<ChunkID>,
    currently_loading_chunks: HashSet<ChunkID>,
    currently_saving_chunks: HashSet<ChunkID>,
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

    pub(in crate) fn start_upgrading_to_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_upgrading_to_chunks.insert(chunk_id);
    }

    pub(in crate) fn stop_upgrading_to_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_upgrading_to_chunks.remove(&chunk_id);
    }

    pub(in crate) fn start_downgrading_from_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_downgrading_from_chunks.insert(chunk_id);
    }

    pub(in crate) fn stop_downgrading_from_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_downgrading_from_chunks.remove(&chunk_id);
    }

    pub(in crate) fn start_loading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_loading_chunks.insert(chunk_id);
    }

    pub(in crate) fn stop_loading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_loading_chunks.remove(&chunk_id);
    }

    pub(in crate) fn start_saving_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_saving_chunks.insert(chunk_id);
    }

    pub(in crate) fn stop_saving_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_saving_chunks.remove(&chunk_id);
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

    pub(in crate) fn is_chunk_upgrading_to(&self, chunk_id: ChunkID) -> bool {
        self.currently_upgrading_to_chunks.contains(&chunk_id)
    }

    pub(in crate) fn is_chunk_downgrading_from(&self, chunk_id: ChunkID) -> bool {
        self.currently_downgrading_from_chunks.contains(&chunk_id)
    }

    pub(in crate) fn is_chunk_loading(&self, chunk_id: ChunkID) -> bool {
        self.currently_loading_chunks.contains(&chunk_id)
    }

    pub(in crate) fn is_chunk_saving(&self, chunk_id: ChunkID) -> bool {
        self.currently_saving_chunks.contains(&chunk_id)
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

    pub(in crate) fn upgrading_to_chunks(&self) -> &HashSet<ChunkID> {
        &self.currently_upgrading_to_chunks
    }

    pub(in crate) fn downgrading_from_chunks(&self) -> &HashSet<ChunkID> {
        &self.currently_downgrading_from_chunks
    }

    pub(in crate) fn loading_chunks(&self) -> &HashSet<ChunkID> {
        &self.currently_loading_chunks
    }

    pub(in crate) fn saving_chunks(&self) -> &HashSet<ChunkID> {
        &self.currently_saving_chunks
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