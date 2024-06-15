use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use super::id::structs::*;
use super::structs::*;

#[derive(Resource, Debug, Default)]
pub(in crate) struct ChunkLoaderRegistry {
    registered_chunk_loaders: HashSet<ChunkLoaderID>,
    loaded_chunk_loaders: HashMap<ChunkLoaderID, Entity>,
    next_chunk_loader_id: ChunkLoaderID,
    recycled_chunk_loader_ids: Vec<ChunkLoaderID>,
    create_chunk_loader_entity_requests: HashMap<ChunkLoaderID, CreateChunkLoaderEntityRequest>,
    upgrade_to_chunk_loader_entity_requests: HashMap<ChunkLoaderID, UpgradeToChunkLoaderEntityRequest>,
}

impl ChunkLoaderRegistry {
    pub fn register_chunk_loader(&mut self) -> ChunkLoaderID {
        let chunk_loader_id = self.get_unused_chunk_loader_id();

        self.registered_chunk_loaders.insert(chunk_loader_id);

        chunk_loader_id
    }

    pub fn register_chunk_loaders(&mut self, batch_size: usize) -> HashSet<ChunkLoaderID> {
        let mut chunk_loader_ids = HashSet::new();

        for _ in 0..batch_size {
            let chunk_loader_id = self.get_unused_chunk_loader_id();
            self.registered_chunk_loaders.insert(chunk_loader_id);
            chunk_loader_ids.insert(chunk_loader_id);
        }

        chunk_loader_ids
    }

    pub fn unregister_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.registered_chunk_loaders.remove(&chunk_loader_id);

        self.recycle_chunk_loader_id(chunk_loader_id);
    }

    pub fn unregister_chunk_loaders(&mut self, chunk_loader_ids: HashSet<ChunkLoaderID>) {
        self.registered_chunk_loaders.retain(|&chunk_loader_id| !chunk_loader_ids.contains(&chunk_loader_id));

        for chunk_loader_id in chunk_loader_ids {
            self.recycle_chunk_loader_id(chunk_loader_id);
        }
    }

    pub fn load_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID, chunk_loader_entity: Entity) {
        self.loaded_chunk_loaders.insert(chunk_loader_id, chunk_loader_entity);
    }

    pub fn load_chunk_loaders(&mut self, chunk_loader_entities: HashMap<ChunkLoaderID, Entity>) {
        self.loaded_chunk_loaders.extend(chunk_loader_entities);
    }

    pub fn unload_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) -> Option<Entity> {
        self.loaded_chunk_loaders.remove(&chunk_loader_id)
    }

    pub fn unload_chunk_loaders(&mut self, chunk_loader_ids: HashSet<ChunkLoaderID>) {
        self.loaded_chunk_loaders.retain(|&chunk_loader_id, _| !chunk_loader_ids.contains(&chunk_loader_id));
    }

    pub fn start_creating_chunk_loader_entity(&mut self, request: CreateChunkLoaderEntityRequest) {
        self.create_chunk_loader_entity_requests.insert(request.chunk_loader_id, request);
    }

    pub fn start_creating_chunk_loader_entities(&mut self, requests: HashMap<ChunkLoaderID, CreateChunkLoaderEntityRequest>) {
        self.create_chunk_loader_entity_requests.extend(requests);
    }

    pub fn stop_creating_chunk_loader_entity(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.create_chunk_loader_entity_requests.remove(&chunk_loader_id);
    }

    pub fn stop_creating_chunk_loader_entities(&mut self, chunk_loader_ids: HashSet<ChunkLoaderID>) {
        self.create_chunk_loader_entity_requests.retain(|chunk_loader_id, _| {
            !chunk_loader_ids.contains(chunk_loader_id)
        });
    }

    pub fn start_upgrading_to_chunk_loader_entity(&mut self, request: UpgradeToChunkLoaderEntityRequest) {
        self.upgrade_to_chunk_loader_entity_requests.insert(request.chunk_loader_id, request);
    }

    pub fn start_upgrading_to_chunk_loader_entities(&mut self, requests: HashMap<ChunkLoaderID, UpgradeToChunkLoaderEntityRequest>) {
        self.upgrade_to_chunk_loader_entity_requests.extend(requests);
    }

    pub fn stop_upgrading_to_chunk_loader_entity(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.upgrade_to_chunk_loader_entity_requests.remove(&chunk_loader_id);
    }

    pub fn stop_upgrading_to_chunk_loader_entities(&mut self, chunk_loader_ids: HashSet<ChunkLoaderID>) {
        self.upgrade_to_chunk_loader_entity_requests.retain(|chunk_loader_id, _| {
            !chunk_loader_ids.contains(chunk_loader_id)
        });
    }

    pub fn is_chunk_loader_registered(&self, chunk_loader_id: ChunkLoaderID) -> bool {
        self.registered_chunk_loaders.contains(&chunk_loader_id)
    }

    pub fn are_chunk_loaders_registered(&self, chunk_loader_ids: HashSet<ChunkLoaderID>) -> bool {
        for chunk_loader_id in chunk_loader_ids {
            if !self.registered_chunk_loaders.contains(&chunk_loader_id) {
                return false;
            }
        }

        true
    }

    pub fn is_chunk_loader_loaded(&self, chunk_loader_id: ChunkLoaderID) -> bool {
        self.loaded_chunk_loaders.contains_key(&chunk_loader_id)
    }

    pub fn are_chunk_loaders_loaded(&self, chunk_loader_ids: HashSet<ChunkLoaderID>) -> bool {
        for chunk_loader_id in chunk_loader_ids {
            if !self.loaded_chunk_loaders.contains_key(&chunk_loader_id) {
                return false;
            }
        }

        true
    }

    pub fn is_chunk_loader_entity_creating(&self, chunk_loader_id: ChunkLoaderID) -> bool {
        self.create_chunk_loader_entity_requests.contains_key(&chunk_loader_id)
    }

    pub fn are_chunk_loader_entities_creating(&self, chunk_loader_ids: HashSet<ChunkLoaderID>) -> bool {
        for chunk_loader_id in chunk_loader_ids {
            if !self.create_chunk_loader_entity_requests.contains_key(&chunk_loader_id) {
                return false;
            }
        }

        true
    }

    pub fn is_chunk_loader_entity_being_upgraded_to(&self, chunk_loader_id: ChunkLoaderID) -> bool {
        self.upgrade_to_chunk_loader_entity_requests.contains_key(&chunk_loader_id)
    }

    pub fn are_chunk_loader_entities_being_upgraded_to(&self, chunk_loader_ids: HashSet<ChunkLoaderID>) -> bool {
        for chunk_loader_id in chunk_loader_ids {
            if !self.upgrade_to_chunk_loader_entity_requests.contains_key(&chunk_loader_id) {
                return false;
            }
        }

        true
    }

    pub fn registered_chunk_loaders(&self) -> &HashSet<ChunkLoaderID> {
        &self.registered_chunk_loaders
    }

    pub fn registered_chunk_loaders_mut(&mut self) -> &mut HashSet<ChunkLoaderID> {
        &mut self.registered_chunk_loaders
    }

    pub fn get_loaded_chunk_loader(&self, chunk_loader_id: ChunkLoaderID) -> Option<Entity> {
        self.loaded_chunk_loaders.get(&chunk_loader_id).copied()
    }

    pub fn loaded_chunk_loader(&self, chunk_loader_id: ChunkLoaderID) -> Entity {
        self.loaded_chunk_loaders[&chunk_loader_id]
    }

    pub fn loaded_chunk_loaders(&self) -> &HashMap<ChunkLoaderID, Entity> {
        &self.loaded_chunk_loaders
    }

    pub fn loaded_chunk_loaders_mut(&mut self) -> &mut HashMap<ChunkLoaderID, Entity> {
        &mut self.loaded_chunk_loaders
    }

    pub fn create_chunk_loader_entity_request(&self, chunk_loader_id: ChunkLoaderID) -> Option<&CreateChunkLoaderEntityRequest> {
        self.create_chunk_loader_entity_requests.get(&chunk_loader_id)
    }

    pub fn create_chunk_loader_entity_requests(&self) -> &HashMap<ChunkLoaderID, CreateChunkLoaderEntityRequest> {
        &self.create_chunk_loader_entity_requests
    }

    pub fn upgrade_to_chunk_loader_entity_request(&self, chunk_loader_id: ChunkLoaderID) -> Option<&UpgradeToChunkLoaderEntityRequest> {
        self.upgrade_to_chunk_loader_entity_requests.get(&chunk_loader_id)
    }

    pub fn upgrade_to_chunk_loader_entity_requests(&self) -> &HashMap<ChunkLoaderID, UpgradeToChunkLoaderEntityRequest> {
        &self.upgrade_to_chunk_loader_entity_requests
    }

    fn get_unused_chunk_loader_id(&mut self) -> ChunkLoaderID {
        if let Some(recycled_chunk_loader_id) = self.recycled_chunk_loader_ids.pop() {
            recycled_chunk_loader_id
        } else {
            let new_chunk_loader_id = self.next_chunk_loader_id;
            self.next_chunk_loader_id = ChunkLoaderID(new_chunk_loader_id.0 + 1);

            new_chunk_loader_id
        }
    }

    fn recycle_chunk_loader_id(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.recycled_chunk_loader_ids.push(chunk_loader_id);
    }
}