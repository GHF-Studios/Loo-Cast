use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::chunk::loader::id::structs::*;
use crate::entity::types::*;

use super::structs::ChunkLoaderRequest;

#[derive(Resource, Debug, Default)]
pub(in crate) struct ChunkLoaderRegistry {
    registered_chunk_loaders: HashSet<ChunkLoaderID>,
    loaded_chunk_loaders: HashMap<ChunkLoaderID, EntityReference>,
    currently_upgrading_to_chunk_loaders: HashSet<ChunkLoaderID>,
    currently_downgrading_from_chunk_loaders: HashSet<ChunkLoaderID>,
    currently_loading_chunk_loaders: HashSet<ChunkLoaderID>,
    currently_saving_chunk_loaders: HashSet<ChunkLoaderID>,
    next_chunk_loader_id: ChunkLoaderID,
    recycled_chunk_loader_ids: Vec<ChunkLoaderID>,
}

impl ChunkLoaderRegistry {
    pub(in crate) fn register_chunk_loader(&mut self) -> ChunkLoaderID {
        let chunk_loader_id = self.get_unused_chunk_loader_id();
        self.registered_chunk_loaders.insert(chunk_loader_id);

        trace!("Registered chunk loader '{:?}'", chunk_loader_id);

        chunk_loader_id
    }

    pub(in crate) fn unregister_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.registered_chunk_loaders.remove(&chunk_loader_id);

        trace!("Unregistered chunk loader '{:?}'", chunk_loader_id);
    }

    pub(in crate) fn load_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID, chunk_loader_entity_reference: EntityReference) {
        self.loaded_chunk_loaders.insert(chunk_loader_id, chunk_loader_entity_reference);

        trace!("Loaded chunk loader '{:?}' | '{:?}'", chunk_loader_id, chunk_loader_entity_reference);
    }

    pub(in crate) fn save_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) -> Option<EntityReference> {
        let chunk_loader_entity_reference = self.loaded_chunk_loaders.remove(&chunk_loader_id);

        trace!("Saved chunk loader '{:?}' | '{:?}'", chunk_loader_id, chunk_loader_entity_reference);

        chunk_loader_entity_reference
    }

    pub(in crate) fn start_upgrading_to_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.currently_upgrading_to_chunk_loaders.insert(chunk_loader_id);

        trace!("Started upgrading to chunk loader '{:?}'", chunk_loader_id);
    }

    pub(in crate) fn stop_upgrading_to_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.currently_upgrading_to_chunk_loaders.remove(&chunk_loader_id);

        trace!("Stopped upgrading to chunk loader '{:?}'", chunk_loader_id);
    }

    pub(in crate) fn start_downgrading_from_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.currently_downgrading_from_chunk_loaders.insert(chunk_loader_id);

        trace!("Started downgrading from chunk loader '{:?}'", chunk_loader_id);
    }

    pub(in crate) fn stop_downgrading_from_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.currently_downgrading_from_chunk_loaders.remove(&chunk_loader_id);

        trace!("Stopped downgrading from chunk loader '{:?}'", chunk_loader_id);
    }

    pub(in crate) fn start_loading_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.currently_loading_chunk_loaders.insert(chunk_loader_id);

        trace!("Started loading chunk loader '{:?}'", chunk_loader_id);
    }

    pub(in crate) fn stop_loading_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.currently_loading_chunk_loaders.remove(&chunk_loader_id);

        trace!("Stopped loading chunk loader '{:?}'", chunk_loader_id);
    }

    pub(in crate) fn start_saving_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.currently_saving_chunk_loaders.insert(chunk_loader_id);

        trace!("Started saving chunk loader '{:?}'", chunk_loader_id);
    }

    pub(in crate) fn stop_saving_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.currently_saving_chunk_loaders.remove(&chunk_loader_id);

        trace!("Stopped saving chunk loader '{:?}'", chunk_loader_id);
    }

    pub(in crate) fn is_chunk_loader_registered(&self, chunk_loader_id: ChunkLoaderID) -> bool {
        self.registered_chunk_loaders.contains(&chunk_loader_id)
    }

    pub(in crate) fn is_chunk_loader_loaded(&self, chunk_loader_id: ChunkLoaderID) -> bool {
        self.loaded_chunk_loaders.contains_key(&chunk_loader_id)
    }

    pub(in crate) fn is_chunk_loader_upgrading_to(&self, chunk_loader_id: ChunkLoaderID) -> bool {
        self.currently_upgrading_to_chunk_loaders.contains(&chunk_loader_id)
    }

    pub(in crate) fn is_chunk_loader_downgrading_from(&self, chunk_loader_id: ChunkLoaderID) -> bool {
        self.currently_downgrading_from_chunk_loaders.contains(&chunk_loader_id)
    }

    pub(in crate) fn is_chunk_loader_loading(&self, chunk_loader_id: ChunkLoaderID) -> bool {
        self.currently_loading_chunk_loaders.contains(&chunk_loader_id)
    }

    pub(in crate) fn is_chunk_loader_saving(&self, chunk_loader_id: ChunkLoaderID) -> bool {
        self.currently_saving_chunk_loaders.contains(&chunk_loader_id)
    }

    pub(in crate) fn registered_chunk_loaders(&self) -> &HashSet<ChunkLoaderID> {
        &self.registered_chunk_loaders
    }

    pub(in crate) fn get_loaded_chunk_loader_entity(&self, chunk_loader_id: ChunkLoaderID) -> Option<EntityReference> {
        self.loaded_chunk_loaders.get(&chunk_loader_id).copied()
    }

    pub(in crate) fn get_loaded_chunk_loader_id(&self, chunk_loader_entity: EntityReference) -> Option<ChunkLoaderID> {
        self.loaded_chunk_loaders.iter().find(|(_, e)| **e == chunk_loader_entity).map(|(id, _)| *id)
    }

    pub(in crate) fn loaded_chunk_loader_entity(&self, chunk_loader_id: ChunkLoaderID) -> EntityReference {
        self.loaded_chunk_loaders[&chunk_loader_id]
    }

    pub(in crate) fn loaded_chunk_loader_id(&self, chunk_loader_entity: EntityReference) -> ChunkLoaderID {
        self.loaded_chunk_loaders.iter().find(|(_, e)| **e == chunk_loader_entity).map(|(id, _)| *id).unwrap()
    }

    pub(in crate) fn loaded_chunk_loaders(&self) -> &HashMap<ChunkLoaderID, EntityReference> {
        &self.loaded_chunk_loaders
    }

    pub(in crate) fn upgrading_to_chunk_loaders(&self) -> &HashSet<ChunkLoaderID> {
        &self.currently_upgrading_to_chunk_loaders
    }

    pub(in crate) fn downgrading_from_chunk_loaders(&self) -> &HashSet<ChunkLoaderID> {
        &self.currently_downgrading_from_chunk_loaders
    }

    pub(in crate) fn loading_chunk_loaders(&self) -> &HashSet<ChunkLoaderID> {
        &self.currently_loading_chunk_loaders
    }

    pub(in crate) fn saving_chunk_loaders(&self) -> &HashSet<ChunkLoaderID> {
        &self.currently_saving_chunk_loaders
    }

    fn get_unused_chunk_loader_id(&mut self) -> ChunkLoaderID {
        if let Some(recycled_chunk_loader_id) = self.recycled_chunk_loader_ids.pop() {
            recycled_chunk_loader_id
        } else {
            let chunk_loader_id = self.next_chunk_loader_id;
            self.next_chunk_loader_id = ChunkLoaderID(chunk_loader_id.0 + 1);

            chunk_loader_id
        }
    }

    fn recycle_chunk_loader_id(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.recycled_chunk_loader_ids.push(chunk_loader_id);
    }
}

#[derive(Resource, Debug, Default)]
pub(in crate) struct ChunkLoaderRequestRegistry {
    registered_chunk_loader_requests: HashSet<ChunkLoaderRequestID>,
    loaded_chunk_loader_requests: HashMap<ChunkLoaderRequestID, ChunkLoaderRequest>,
    next_chunk_loader_request_id: ChunkLoaderRequestID,
}

impl ChunkLoaderRequestRegistry {
    pub(in crate) fn register_chunk_loader_request(&mut self) -> ChunkLoaderRequestID {
        let chunk_loader_request_id = self.get_unused_chunk_loader_request_id();
        self.registered_chunk_loader_requests.insert(chunk_loader_request_id);

        trace!("Registered chunk loader request '{:?}'", chunk_loader_request_id);

        chunk_loader_request_id
    }

    pub(in crate) fn unregister_chunk_loader_request(&mut self, chunk_loader_request_id: ChunkLoaderRequestID) {
        self.registered_chunk_loader_requests.remove(&chunk_loader_request_id);

        trace!("Unregistered chunk loader request '{:?}'", chunk_loader_request_id);
    }

    pub(in crate) fn load_chunk_loader_request(&mut self, chunk_loader_request_id: ChunkLoaderRequestID, chunk_loader_request: ChunkLoaderRequest) {
        self.loaded_chunk_loader_requests.insert(chunk_loader_request_id, chunk_loader_request);

        trace!("Loaded chunk loader request '{:?}'", chunk_loader_request_id);
    }

    pub(in crate) fn unload_chunk_loader_request(&mut self, chunk_loader_request_id: ChunkLoaderRequestID) -> Option<ChunkLoaderRequest> {
        let removed_chunk_loader_request = self.loaded_chunk_loader_requests.remove(&chunk_loader_request_id);

        trace!("Unloaded chunk loader request '{:?}'", chunk_loader_request_id);

        removed_chunk_loader_request
    }

    pub(in crate) fn is_chunk_loader_request_registered(&self, chunk_loader_request_id: ChunkLoaderRequestID) -> bool {
        self.registered_chunk_loader_requests.contains(&chunk_loader_request_id)
    }

    pub(in crate) fn is_chunk_loader_request_loaded(&self, chunk_loader_request_id: ChunkLoaderRequestID) -> bool {
        self.loaded_chunk_loader_requests.contains_key(&chunk_loader_request_id)
    }

    pub(in crate) fn registered_chunk_loader_requests(&self) -> &HashSet<ChunkLoaderRequestID> {
        &self.registered_chunk_loader_requests
    }

    pub(in crate) fn get_loaded_chunk_loader_request(&self, chunk_loader_request_id: ChunkLoaderRequestID) -> Option<ChunkLoaderRequest> {
        self.loaded_chunk_loader_requests.get(&chunk_loader_request_id).copied()
    }

    pub(in crate) fn get_loaded_chunk_loader_request_id(&self, chunk_loader_request: &ChunkLoaderRequest) -> Option<ChunkLoaderRequestID> {
        self.loaded_chunk_loader_requests.iter().find(|(_, r)| **r == *chunk_loader_request).map(|(id, _)| *id)
    }

    pub(in crate) fn loaded_chunk_loader_request(&self, chunk_loader_request_id: ChunkLoaderRequestID) -> ChunkLoaderRequest {
        self.loaded_chunk_loader_requests[&chunk_loader_request_id]
    }

    pub(in crate) fn loaded_chunk_loader_request_id(&self, chunk_loader_request: ChunkLoaderRequest) -> ChunkLoaderRequestID {
        self.loaded_chunk_loader_requests.iter().find(|(_, r)| **r == chunk_loader_request).map(|(id, _)| *id).unwrap()
    }

    pub(in crate) fn loaded_chunk_loader_requests(&self) -> &HashMap<ChunkLoaderRequestID, ChunkLoaderRequest> {
        &self.loaded_chunk_loader_requests
    }

    fn get_unused_chunk_loader_request_id(&mut self) -> ChunkLoaderRequestID {
        let chunk_loader_request_id = self.next_chunk_loader_request_id;
        self.next_chunk_loader_request_id = ChunkLoaderRequestID(chunk_loader_request_id.0 + 1);

        chunk_loader_request_id
    }
}