use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::chunk::actor::id::structs::*;
use crate::entity::types::*;

use super::structs::ChunkActorRequest;

#[derive(Resource, Debug, Default)]
pub(in crate) struct ChunkActorRegistry {
    registered_chunk_actors: HashSet<ChunkActorID>,
    loaded_chunk_actors: HashMap<ChunkActorID, EntityReference>,
    currently_upgrading_to_chunk_actors: HashSet<ChunkActorID>,
    currently_downgrading_from_chunk_actors: HashSet<ChunkActorID>,
    currently_loading_chunk_actors: HashSet<ChunkActorID>,
    currently_saving_chunk_actors: HashSet<ChunkActorID>,
    next_chunk_actor_id: ChunkActorID,
    recycled_chunk_actor_ids: Vec<ChunkActorID>,
}

impl ChunkActorRegistry {
    pub(in crate) fn register_chunk_actor(&mut self) -> ChunkActorID {
        let chunk_actor_id = self.get_unused_chunk_actor_id();
        self.registered_chunk_actors.insert(chunk_actor_id);

        trace!("Registered chunk actor '{:?}'", chunk_actor_id);

        chunk_actor_id
    }

    pub(in crate) fn unregister_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.registered_chunk_actors.remove(&chunk_actor_id);

        trace!("Unregistered chunk actor '{:?}'", chunk_actor_id);
    }

    pub(in crate) fn load_chunk_actor(&mut self, chunk_actor_id: ChunkActorID, chunk_actor_entity_reference: EntityReference) {
        self.loaded_chunk_actors.insert(chunk_actor_id, chunk_actor_entity_reference);

        trace!("Loaded chunk actor '{:?}' | '{:?}'", chunk_actor_id, chunk_actor_entity_reference);
    }

    pub(in crate) fn save_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) -> Option<EntityReference> {
        let chunk_actor_entity_reference = self.loaded_chunk_actors.remove(&chunk_actor_id);

        trace!("Saved chunk actor '{:?}' | '{:?}'", chunk_actor_id, chunk_actor_entity_reference);

        chunk_actor_entity_reference
    }

    pub(in crate) fn start_upgrading_to_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.currently_upgrading_to_chunk_actors.insert(chunk_actor_id);

        trace!("Started upgrading to chunk actor '{:?}'", chunk_actor_id);
    }

    pub(in crate) fn stop_upgrading_to_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.currently_upgrading_to_chunk_actors.remove(&chunk_actor_id);

        trace!("Stopped upgrading to chunk actor '{:?}'", chunk_actor_id);
    }

    pub(in crate) fn start_downgrading_from_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.currently_downgrading_from_chunk_actors.insert(chunk_actor_id);

        trace!("Started downgrading from chunk actor '{:?}'", chunk_actor_id);
    }

    pub(in crate) fn stop_downgrading_from_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.currently_downgrading_from_chunk_actors.remove(&chunk_actor_id);

        trace!("Stopped downgrading from chunk actor '{:?}'", chunk_actor_id);
    }

    pub(in crate) fn start_loading_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.currently_loading_chunk_actors.insert(chunk_actor_id);

        trace!("Started loading chunk actor '{:?}'", chunk_actor_id);
    }

    pub(in crate) fn stop_loading_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.currently_loading_chunk_actors.remove(&chunk_actor_id);

        trace!("Stopped loading chunk actor '{:?}'", chunk_actor_id);
    }

    pub(in crate) fn start_saving_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.currently_saving_chunk_actors.insert(chunk_actor_id);

        trace!("Started saving chunk actor '{:?}'", chunk_actor_id);
    }

    pub(in crate) fn stop_saving_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.currently_saving_chunk_actors.remove(&chunk_actor_id);

        trace!("Stopped saving chunk actor '{:?}'", chunk_actor_id);
    }

    pub(in crate) fn is_chunk_actor_registered(&self, chunk_actor_id: ChunkActorID) -> bool {
        self.registered_chunk_actors.contains(&chunk_actor_id)
    }

    pub(in crate) fn is_chunk_actor_loaded(&self, chunk_actor_id: ChunkActorID) -> bool {
        self.loaded_chunk_actors.contains_key(&chunk_actor_id)
    }

    pub(in crate) fn is_chunk_actor_upgrading_to(&self, chunk_actor_id: ChunkActorID) -> bool {
        self.currently_upgrading_to_chunk_actors.contains(&chunk_actor_id)
    }

    pub(in crate) fn is_chunk_actor_downgrading_from(&self, chunk_actor_id: ChunkActorID) -> bool {
        self.currently_downgrading_from_chunk_actors.contains(&chunk_actor_id)
    }

    pub(in crate) fn is_chunk_actor_loading(&self, chunk_actor_id: ChunkActorID) -> bool {
        self.currently_loading_chunk_actors.contains(&chunk_actor_id)
    }

    pub(in crate) fn is_chunk_actor_saving(&self, chunk_actor_id: ChunkActorID) -> bool {
        self.currently_saving_chunk_actors.contains(&chunk_actor_id)
    }

    pub(in crate) fn registered_chunk_actors(&self) -> &HashSet<ChunkActorID> {
        &self.registered_chunk_actors
    }

    pub(in crate) fn get_loaded_chunk_actor_entity(&self, chunk_actor_id: ChunkActorID) -> Option<EntityReference> {
        self.loaded_chunk_actors.get(&chunk_actor_id).copied()
    }

    pub(in crate) fn get_loaded_chunk_actor_id(&self, chunk_actor_entity: EntityReference) -> Option<ChunkActorID> {
        self.loaded_chunk_actors.iter().find(|(_, e)| **e == chunk_actor_entity).map(|(id, _)| *id)
    }

    pub(in crate) fn loaded_chunk_actor_entity(&self, chunk_actor_id: ChunkActorID) -> EntityReference {
        self.loaded_chunk_actors[&chunk_actor_id]
    }

    pub(in crate) fn loaded_chunk_actor_id(&self, chunk_actor_entity: EntityReference) -> ChunkActorID {
        self.loaded_chunk_actors.iter().find(|(_, e)| **e == chunk_actor_entity).map(|(id, _)| *id).unwrap()
    }

    pub(in crate) fn loaded_chunk_actors(&self) -> &HashMap<ChunkActorID, EntityReference> {
        &self.loaded_chunk_actors
    }

    pub(in crate) fn upgrading_to_chunk_actors(&self) -> &HashSet<ChunkActorID> {
        &self.currently_upgrading_to_chunk_actors
    }

    pub(in crate) fn downgrading_from_chunk_actors(&self) -> &HashSet<ChunkActorID> {
        &self.currently_downgrading_from_chunk_actors
    }

    pub(in crate) fn loading_chunk_actors(&self) -> &HashSet<ChunkActorID> {
        &self.currently_loading_chunk_actors
    }

    pub(in crate) fn saving_chunk_actors(&self) -> &HashSet<ChunkActorID> {
        &self.currently_saving_chunk_actors
    }

    fn get_unused_chunk_actor_id(&mut self) -> ChunkActorID {
        if let Some(recycled_chunk_actor_id) = self.recycled_chunk_actor_ids.pop() {
            recycled_chunk_actor_id
        } else {
            let chunk_actor_id = self.next_chunk_actor_id;
            self.next_chunk_actor_id = ChunkActorID(chunk_actor_id.0 + 1);

            chunk_actor_id
        }
    }

    fn recycle_chunk_actor_id(&mut self, chunk_actor_id: ChunkActorID) {
        self.recycled_chunk_actor_ids.push(chunk_actor_id);
    }
}

#[derive(Resource, Debug, Default)]
pub(in crate) struct ChunkActorRequestRegistry {
    registered_chunk_actor_requests: HashSet<ChunkActorRequestID>,
    loaded_chunk_actor_requests: HashMap<ChunkActorRequestID, ChunkActorRequest>,
    next_chunk_actor_request_id: ChunkActorRequestID,
}

impl ChunkActorRequestRegistry {
    pub(in crate) fn register_chunk_actor_request(&mut self) -> ChunkActorRequestID {
        let chunk_actor_request_id = self.get_unused_chunk_actor_request_id();
        self.registered_chunk_actor_requests.insert(chunk_actor_request_id);

        trace!("Registered chunk actor request '{:?}'", chunk_actor_request_id);

        chunk_actor_request_id
    }

    pub(in crate) fn unregister_chunk_actor_request(&mut self, chunk_actor_request_id: ChunkActorRequestID) {
        self.registered_chunk_actor_requests.remove(&chunk_actor_request_id);

        trace!("Unregistered chunk actor request '{:?}'", chunk_actor_request_id);
    }

    pub(in crate) fn load_chunk_actor_request(&mut self, chunk_actor_request_id: ChunkActorRequestID, chunk_actor_request: ChunkActorRequest) {
        self.loaded_chunk_actor_requests.insert(chunk_actor_request_id, chunk_actor_request);

        trace!("Loaded chunk actor request '{:?}'", chunk_actor_request_id);
    }

    pub(in crate) fn unload_chunk_actor_request(&mut self, chunk_actor_request_id: ChunkActorRequestID) -> Option<ChunkActorRequest> {
        let removed_chunk_actor_request = self.loaded_chunk_actor_requests.remove(&chunk_actor_request_id);

        trace!("Unloaded chunk actor request '{:?}'", chunk_actor_request_id);

        removed_chunk_actor_request
    }

    pub(in crate) fn is_chunk_actor_request_registered(&self, chunk_actor_request_id: ChunkActorRequestID) -> bool {
        self.registered_chunk_actor_requests.contains(&chunk_actor_request_id)
    }

    pub(in crate) fn is_chunk_actor_request_loaded(&self, chunk_actor_request_id: ChunkActorRequestID) -> bool {
        self.loaded_chunk_actor_requests.contains_key(&chunk_actor_request_id)
    }

    pub(in crate) fn registered_chunk_actor_requests(&self) -> &HashSet<ChunkActorRequestID> {
        &self.registered_chunk_actor_requests
    }

    pub(in crate) fn get_loaded_chunk_actor_request(&self, chunk_actor_request_id: ChunkActorRequestID) -> Option<ChunkActorRequest> {
        self.loaded_chunk_actor_requests.get(&chunk_actor_request_id).copied()
    }

    pub(in crate) fn get_loaded_chunk_actor_request_id(&self, chunk_actor_request: &ChunkActorRequest) -> Option<ChunkActorRequestID> {
        self.loaded_chunk_actor_requests.iter().find(|(_, r)| **r == *chunk_actor_request).map(|(id, _)| *id)
    }

    pub(in crate) fn loaded_chunk_actor_request(&self, chunk_actor_request_id: ChunkActorRequestID) -> ChunkActorRequest {
        self.loaded_chunk_actor_requests[&chunk_actor_request_id]
    }

    pub(in crate) fn loaded_chunk_actor_request_id(&self, chunk_actor_request: ChunkActorRequest) -> ChunkActorRequestID {
        self.loaded_chunk_actor_requests.iter().find(|(_, r)| **r == chunk_actor_request).map(|(id, _)| *id).unwrap()
    }

    pub(in crate) fn loaded_chunk_actor_requests(&self) -> &HashMap<ChunkActorRequestID, ChunkActorRequest> {
        &self.loaded_chunk_actor_requests
    }

    fn get_unused_chunk_actor_request_id(&mut self) -> ChunkActorRequestID {
        let chunk_actor_request_id = self.next_chunk_actor_request_id;
        self.next_chunk_actor_request_id = ChunkActorRequestID(chunk_actor_request_id.0 + 1);

        chunk_actor_request_id
    }
}