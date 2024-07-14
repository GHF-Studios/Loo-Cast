use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use super::{id::structs::*, structs::*};
use crate::entity::types::*;

#[derive(Resource, Debug, Default)]
pub(in crate) struct ChunkActorRegistry {
    registered_chunk_actors: HashSet<ChunkActorID>,
    loaded_chunk_actors: HashMap<ChunkActorID, EntityReference>,
    currently_creating_chunk_actors: HashSet<ChunkActorID>,
    currently_destroying_chunk_actors: HashSet<ChunkActorID>,
    currently_upgrading_to_chunk_actors: HashSet<ChunkActorID>,
    create_chunk_actor_requests: HashMap<ChunkActorRequestID, ChunkActorCreateRequest>,
    promote__chunk_actor_requests: HashMap<ChunkActorRequestID, ChunkActorUpgradeRequest>,
    next_chunk_actor_id: ChunkActorID,
    recycled_chunk_actor_ids: Vec<ChunkActorID>,
}

impl ChunkActorRegistry {
    pub fn register_chunk_actor(&mut self) -> ChunkActorID {
        let chunk_actor_id = self.get_unused_chunk_actor_id();

        self.registered_chunk_actors.insert(chunk_actor_id);

        chunk_actor_id
    }

    pub fn register_chunk_actors(&mut self, batch_size: usize) -> HashSet<ChunkActorID> {
        let mut chunk_actor_ids = HashSet::new();

        for _ in 0..batch_size {
            let chunk_actor_id = self.get_unused_chunk_actor_id();
            self.registered_chunk_actors.insert(chunk_actor_id);
            chunk_actor_ids.insert(chunk_actor_id);
        }

        chunk_actor_ids
    }

    pub fn unregister_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.registered_chunk_actors.remove(&chunk_actor_id);

        self.recycle_chunk_actor_id(chunk_actor_id);
    }

    pub fn unregister_chunk_actors(&mut self, chunk_actor_ids: HashSet<ChunkActorID>) {
        self.registered_chunk_actors.retain(|&chunk_actor_id| !chunk_actor_ids.contains(&chunk_actor_id));

        for chunk_actor_id in chunk_actor_ids {
            self.recycle_chunk_actor_id(chunk_actor_id);
        }
    }

    pub fn load_chunk_actor(&mut self, chunk_actor_id: ChunkActorID, chunk_actor_entity_reference: EntityReference) {
        self.loaded_chunk_actors.insert(chunk_actor_id, chunk_actor_entity_reference);
    }

    pub fn load_chunk_actors(&mut self, chunk_actor_entities: HashMap<ChunkActorID, EntityReference>) {
        self.loaded_chunk_actors.extend(chunk_actor_entities);
    }

    pub fn unload_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) -> Option<EntityReference> {
        self.loaded_chunk_actors.remove(&chunk_actor_id)
    }

    pub fn unload_chunk_actors(&mut self, chunk_actor_ids: HashSet<ChunkActorID>) {
        self.loaded_chunk_actors.retain(|&chunk_actor_id, _| !chunk_actor_ids.contains(&chunk_actor_id));
    }

    pub fn start_creating_chunk_actor(&mut self, request: ChunkActorCreateRequest) {
        self.currently_creating_chunk_actors.insert(request.chunk_actor_id);
        self.create_chunk_actor_requests.insert(request.chunk_actor_request_id, request);
    }

    pub fn start_creating_chunk_actors(&mut self, requests: HashSet<ChunkActorCreateRequest>) {
        for request in requests {
            self.currently_creating_chunk_actors.insert(request.chunk_actor_id);
            self.create_chunk_actor_requests.insert(request.chunk_actor_request_id, request.clone());
        }
    }

    pub fn stop_creating_chunk_actor(&mut self, chunk_actor_id: ChunkActorID, request_id: ChunkActorRequestID) {
        let removed_request = match self.create_chunk_actor_requests.remove(&request_id) {
            Some(request) => request,
            None => {
                panic!("Request '{:?}' could not be found!", request_id);
            },
        };

        if removed_request.chunk_actor_id != chunk_actor_id {
            panic!("Request '{:?}' does not belong to chunk actor '{:?}'!", request_id, chunk_actor_id);
        }

        self.currently_creating_chunk_actors.remove(&chunk_actor_id);
    }

    pub fn stop_creating_chunk_actors(&mut self, chunk_actor_ids: HashSet<ChunkActorID>, request_ids: HashSet<ChunkActorRequestID>) {
        if chunk_actor_ids.len() != request_ids.len() {
            panic!("Mismatched number of chunk actor IDs and request IDs!");
        }

        let mut remaining_chunk_actor_ids = chunk_actor_ids.clone();

        for request_id in request_ids {
            let removed_request = match self.create_chunk_actor_requests.remove(&request_id) {
                Some(request) => request,
                None => {
                    panic!("Request '{:?}' could not be found!", request_id);
                },
            };

            if !remaining_chunk_actor_ids.remove(&removed_request.chunk_actor_id) {
                panic!("Request '{:?}' does not belong to any of the provided chunk actor IDs or has already been matched!", request_id);
            }
        }

        for chunk_actor_id in chunk_actor_ids {
            self.currently_creating_chunk_actors.remove(&chunk_actor_id);
        }
    }

    pub fn start_destroying_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.currently_destroying_chunk_actors.insert(chunk_actor_id);
    }

    pub fn start_destroying_chunk_actors(&mut self, chunk_actor_ids: HashSet<ChunkActorID>) {
        self.currently_destroying_chunk_actors.extend(chunk_actor_ids);
    }

    pub fn stop_destroying_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.currently_destroying_chunk_actors.remove(&chunk_actor_id);
    }

    pub fn stop_destroying_chunk_actors(&mut self, chunk_actor_ids: HashSet<ChunkActorID>) {
        self.currently_destroying_chunk_actors.retain(|&chunk_actor_id| !chunk_actor_ids.contains(&chunk_actor_id));
    }

    pub fn start_upgrading_to_chunk_actor(&mut self, request: ChunkActorUpgradeRequest) {
        self.currently_upgrading_to_chunk_actors.insert(request.chunk_actor_id);
        self.promote__chunk_actor_requests.insert(request.chunk_actor_request_id, request);
    }

    pub fn start_upgrading_to_chunk_actors(&mut self, requests: HashSet<ChunkActorUpgradeRequest>) {
        for request in requests {
            self.currently_upgrading_to_chunk_actors.insert(request.chunk_actor_id);
            self.promote__chunk_actor_requests.insert(request.chunk_actor_request_id, request.clone());
        }
    }

    pub fn stop_upgrading_to_chunk_actor(&mut self, chunk_actor_id: ChunkActorID, request_id: ChunkActorRequestID) {
        let removed_request = match self.promote__chunk_actor_requests.remove(&request_id) {
            Some(request) => request,
            None => {
                panic!("Request '{:?}' could not be found!", request_id);
            },
        };

        if removed_request.chunk_actor_id != chunk_actor_id {
            panic!("Request '{:?}' does not belong to chunk actor '{:?}'!", request_id, chunk_actor_id);
        }

        self.currently_upgrading_to_chunk_actors.remove(&chunk_actor_id);
    }

    pub fn stop_upgrading_to_chunk_actors(&mut self, chunk_actor_ids: HashSet<ChunkActorID>, request_ids: HashSet<ChunkActorRequestID>) {
        if chunk_actor_ids.len() != request_ids.len() {
            panic!("Mismatched number of chunk actor IDs and request IDs!");
        }

        let mut remaining_chunk_actor_ids = chunk_actor_ids.clone();

        for request_id in request_ids {
            let removed_request = match self.promote__chunk_actor_requests.remove(&request_id) {
                Some(request) => request,
                None => {
                    panic!("Request '{:?}' could not be found!", request_id);
                },
            };

            if !remaining_chunk_actor_ids.remove(&removed_request.chunk_actor_id) {
                panic!("Request '{:?}' does not belong to any of the provided chunk actor IDs or has already been matched!", request_id);
            }
        }

        for chunk_actor_id in chunk_actor_ids {
            self.currently_upgrading_to_chunk_actors.remove(&chunk_actor_id);
        }
    }

    pub fn is_chunk_actor_registered(&self, chunk_actor_id: ChunkActorID) -> bool {
        self.registered_chunk_actors.contains(&chunk_actor_id)
    }

    pub fn are_chunk_actors_registered(&self, chunk_actor_ids: HashSet<ChunkActorID>) -> bool {
        for chunk_actor_id in chunk_actor_ids {
            if !self.registered_chunk_actors.contains(&chunk_actor_id) {
                return false;
            }
        }

        true
    }

    pub fn is_chunk_actor_loaded(&self, chunk_actor_id: ChunkActorID) -> bool {
        self.loaded_chunk_actors.contains_key(&chunk_actor_id)
    }

    pub fn are_chunk_actors_loaded(&self, chunk_actor_ids: HashSet<ChunkActorID>) -> bool {
        for chunk_actor_id in chunk_actor_ids {
            if !self.loaded_chunk_actors.contains_key(&chunk_actor_id) {
                return false;
            }
        }

        true
    }

    pub fn is_chunk_actor_creating(&self, chunk_actor_id: ChunkActorID) -> bool {
        self.currently_creating_chunk_actors.contains(&chunk_actor_id)
    }

    pub fn are_chunk_actors_creating(&self, chunk_actor_ids: HashSet<ChunkActorID>) -> bool {
        for chunk_actor_id in chunk_actor_ids {
            if !self.currently_creating_chunk_actors.contains(&chunk_actor_id) {
                return false;
            }
        }

        true
    }

    pub fn is_chunk_actor_destroying(&self, chunk_actor_id: ChunkActorID) -> bool {
        self.currently_destroying_chunk_actors.contains(&chunk_actor_id)
    }

    pub fn are_chunk_actors_destroying(&self, chunk_actor_ids: HashSet<ChunkActorID>) -> bool {
        for chunk_actor_id in chunk_actor_ids {
            if !self.currently_destroying_chunk_actors.contains(&chunk_actor_id) {
                return false;
            }
        }

        true
    }

    pub fn is_chunk_actor_upgrading_to(&self, chunk_actor_id: ChunkActorID) -> bool {
        self.currently_upgrading_to_chunk_actors.contains(&chunk_actor_id)
    }

    pub fn are_chunk_actors_upgrading_to(&self, chunk_actor_ids: HashSet<ChunkActorID>) -> bool {
        for chunk_actor_id in chunk_actor_ids {
            if !self.currently_upgrading_to_chunk_actors.contains(&chunk_actor_id) {
                return false;
            }
        }

        true
    }

    pub fn registered_chunk_actors(&self) -> &HashSet<ChunkActorID> {
        &self.registered_chunk_actors
    }

    pub fn registered_chunk_actors_mut(&mut self) -> &mut HashSet<ChunkActorID> {
        &mut self.registered_chunk_actors
    }

    pub fn get_loaded_chunk_actor(&self, chunk_actor_id: ChunkActorID) -> Option<EntityReference> {
        self.loaded_chunk_actors.get(&chunk_actor_id).copied()
    }

    pub fn loaded_chunk_actor(&self, chunk_actor_id: ChunkActorID) -> EntityReference {
        self.loaded_chunk_actors[&chunk_actor_id]
    }

    pub fn loaded_chunk_actors(&self) -> &HashMap<ChunkActorID, EntityReference> {
        &self.loaded_chunk_actors
    }

    pub fn loaded_chunk_actors_mut(&mut self) -> &mut HashMap<ChunkActorID, EntityReference> {
        &mut self.loaded_chunk_actors
    }

    pub fn creating_chunk_actors(&self) -> &HashSet<ChunkActorID> {
        &self.currently_creating_chunk_actors
    }

    pub fn creating_chunk_actors_mut(&mut self) -> &mut HashSet<ChunkActorID> {
        &mut self.currently_creating_chunk_actors
    }

    pub fn destroying_chunk_actors(&self) -> &HashSet<ChunkActorID> {
        &self.currently_destroying_chunk_actors
    }

    pub fn destroying_chunk_actors_mut(&mut self) -> &mut HashSet<ChunkActorID> {
        &mut self.currently_destroying_chunk_actors
    }

    pub fn upgrading_to_chunk_actors(&self) -> &HashSet<ChunkActorID> {
        &self.currently_upgrading_to_chunk_actors
    }

    pub fn upgrading_to_chunk_actors_mut(&mut self) -> &mut HashSet<ChunkActorID> {
        &mut self.currently_upgrading_to_chunk_actors
    }

    pub fn get_create_chunk_actor_request(&self, request_id: ChunkActorRequestID) -> Option<&ChunkActorCreateRequest> {
        self.create_chunk_actor_requests.get(&request_id)
    }

    pub fn create_chunk_actor_request(&self, request_id: ChunkActorRequestID) -> &ChunkActorCreateRequest {
        &self.create_chunk_actor_requests[&request_id]
    }

    pub fn create_chunk_actor_requests(&self) -> &HashMap<ChunkActorRequestID, ChunkActorCreateRequest> {
        &self.create_chunk_actor_requests
    }

    pub fn create_chunk_actor_requests_mut(&mut self) -> &mut HashMap<ChunkActorRequestID, ChunkActorCreateRequest> {
        &mut self.create_chunk_actor_requests
    }

    pub fn get_promote__chunk_actor_request(&self, request_id: ChunkActorRequestID) -> Option<&ChunkActorUpgradeRequest> {
        self.promote__chunk_actor_requests.get(&request_id)
    }

    pub fn promote__chunk_actor_request(&self, request_id: ChunkActorRequestID) -> &ChunkActorUpgradeRequest {
        &self.promote__chunk_actor_requests[&request_id]
    }

    pub fn promote__chunk_actor_requests(&self) -> &HashMap<ChunkActorRequestID, ChunkActorUpgradeRequest> {
        &self.promote__chunk_actor_requests
    }

    pub fn promote__chunk_actor_requests_mut(&mut self) -> &mut HashMap<ChunkActorRequestID, ChunkActorUpgradeRequest> {
        &mut self.promote__chunk_actor_requests
    }

    fn get_unused_chunk_actor_id(&mut self) -> ChunkActorID {
        if let Some(recycled_chunk_actor_id) = self.recycled_chunk_actor_ids.pop() {
            recycled_chunk_actor_id
        } else {
            let new_chunk_actor_id = self.next_chunk_actor_id;
            self.next_chunk_actor_id = ChunkActorID(new_chunk_actor_id.0 + 1);

            new_chunk_actor_id
        }
    }

    fn recycle_chunk_actor_id(&mut self, chunk_actor_id: ChunkActorID) {
        self.recycled_chunk_actor_ids.push(chunk_actor_id);
    }
}

#[derive(Resource, Debug, Default)]
pub(in crate) struct ChunkActorRequestRegistry {
    next_chunk_actor_request_id: ChunkActorRequestID,
}

impl ChunkActorRequestRegistry {
    pub fn get_unused_chunk_actor_request_id(&mut self) -> ChunkActorRequestID {
        let chunk_actor_request_id = self.next_chunk_actor_request_id;

        self.next_chunk_actor_request_id = ChunkActorRequestID(chunk_actor_request_id.0 + 1);

        chunk_actor_request_id
    }
}