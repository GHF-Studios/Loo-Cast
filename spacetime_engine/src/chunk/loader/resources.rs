use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use super::id::structs::*;
use crate::entity::types::*;

#[derive(Resource, Debug, Default)]
pub(in crate) struct ChunkLoaderRegistry {
    registered_chunk_loaders: HashSet<ChunkLoaderID>,
    loaded_chunk_loaders: HashMap<ChunkLoaderID, EntityReference>,
    started_chunk_loaders: HashMap<ChunkLoaderID, EntityReference>,
    currently_creating_chunk_loaders: HashSet<ChunkLoaderID>,
    currently_destroying_chunk_loaders: HashSet<ChunkLoaderID>,
    currently_upgrading_to_chunk_loaders: HashSet<ChunkLoaderID>,
    next_chunk_loader_id: ChunkLoaderID,
    recycled_chunk_loader_ids: Vec<ChunkLoaderID>,
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

    pub fn load_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID, chunk_loader_entity_reference: EntityReference) {
        self.loaded_chunk_loaders.insert(chunk_loader_id, chunk_loader_entity_reference);
    }

    pub fn load_chunk_loaders(&mut self, chunk_loader_entities: HashMap<ChunkLoaderID, EntityReference>) {
        self.loaded_chunk_loaders.extend(chunk_loader_entities);
    }

    pub fn unload_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) -> Option<EntityReference> {
        self.loaded_chunk_loaders.remove(&chunk_loader_id)
    }

    pub fn unload_chunk_loaders(&mut self, chunk_loader_ids: HashSet<ChunkLoaderID>) {
        self.loaded_chunk_loaders.retain(|&chunk_loader_id, _| !chunk_loader_ids.contains(&chunk_loader_id));
    }

    pub fn start_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.started_chunk_loaders.insert(chunk_loader_id, self.loaded_chunk_loaders[&chunk_loader_id]);
    }

    pub fn start_chunk_loaders(&mut self, chunk_loader_entities: HashSet<ChunkLoaderID>) {
        for chunk_loader_id in chunk_loader_entities {
            self.started_chunk_loaders.insert(chunk_loader_id, self.loaded_chunk_loaders[&chunk_loader_id]);
        }
    }

    pub fn stop_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.started_chunk_loaders.remove(&chunk_loader_id);
    }

    pub fn stop_chunk_loaders(&mut self, chunk_loader_ids: HashSet<ChunkLoaderID>) {
        for chunk_loader_id in chunk_loader_ids {
            self.started_chunk_loaders.remove(&chunk_loader_id);
        }
    }

    pub fn start_creating_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.currently_creating_chunk_loaders.insert(chunk_loader_id);
    }

    pub fn start_creating_chunk_loaders(&mut self, chunk_loader_ids: HashSet<ChunkLoaderID>) {
        self.currently_creating_chunk_loaders.extend(chunk_loader_ids);
    }

    pub fn stop_creating_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.currently_creating_chunk_loaders.remove(&chunk_loader_id);
    }

    pub fn stop_creating_chunk_loaders(&mut self, chunk_loader_ids: HashSet<ChunkLoaderID>) {
        self.currently_creating_chunk_loaders.retain(|&chunk_loader_id| !chunk_loader_ids.contains(&chunk_loader_id));
    }

    pub fn start_destroying_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.currently_destroying_chunk_loaders.insert(chunk_loader_id);
    }

    pub fn start_destroying_chunk_loaders(&mut self, chunk_loader_ids: HashSet<ChunkLoaderID>) {
        self.currently_destroying_chunk_loaders.extend(chunk_loader_ids);
    }

    pub fn stop_destroying_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.currently_destroying_chunk_loaders.remove(&chunk_loader_id);
    }

    pub fn stop_destroying_chunk_loaders(&mut self, chunk_loader_ids: HashSet<ChunkLoaderID>) {
        self.currently_destroying_chunk_loaders.retain(|&chunk_loader_id| !chunk_loader_ids.contains(&chunk_loader_id));
    }

    pub fn start_upgrading_to_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.currently_upgrading_to_chunk_loaders.insert(chunk_loader_id);
    }

    pub fn start_upgrading_to_chunk_loaders(&mut self, chunk_loader_ids: HashSet<ChunkLoaderID>) {
        self.currently_upgrading_to_chunk_loaders.extend(chunk_loader_ids);
    }

    pub fn stop_upgrading_to_chunk_loader(&mut self, chunk_loader_id: ChunkLoaderID) {
        self.currently_upgrading_to_chunk_loaders.remove(&chunk_loader_id);
    }

    pub fn stop_upgrading_to_chunk_loaders(&mut self, chunk_loader_ids: HashSet<ChunkLoaderID>) {
        self.currently_upgrading_to_chunk_loaders.retain(|&chunk_loader_id| !chunk_loader_ids.contains(&chunk_loader_id));
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

    pub fn is_chunk_loader_started(&self, chunk_loader_id: ChunkLoaderID) -> bool {
        self.started_chunk_loaders.contains_key(&chunk_loader_id)
    }

    pub fn are_chunk_loaders_started(&self, chunk_loader_ids: HashSet<ChunkLoaderID>) -> bool {
        for chunk_loader_id in chunk_loader_ids {
            if !self.started_chunk_loaders.contains_key(&chunk_loader_id) {
                return false;
            }
        }

        true
    }

    pub fn is_chunk_loader_creating(&self, chunk_loader_id: ChunkLoaderID) -> bool {
        self.currently_creating_chunk_loaders.contains(&chunk_loader_id)
    }

    pub fn are_chunk_loaders_creating(&self, chunk_loader_ids: HashSet<ChunkLoaderID>) -> bool {
        for chunk_loader_id in chunk_loader_ids {
            if !self.currently_creating_chunk_loaders.contains(&chunk_loader_id) {
                return false;
            }
        }

        true
    }

    pub fn is_chunk_loader_destroying(&self, chunk_loader_id: ChunkLoaderID) -> bool {
        self.currently_destroying_chunk_loaders.contains(&chunk_loader_id)
    }

    pub fn are_chunk_loaders_destroying(&self, chunk_loader_ids: HashSet<ChunkLoaderID>) -> bool {
        for chunk_loader_id in chunk_loader_ids {
            if !self.currently_destroying_chunk_loaders.contains(&chunk_loader_id) {
                return false;
            }
        }

        true
    }

    pub fn is_chunk_loader_upgrading_to(&self, chunk_loader_id: ChunkLoaderID) -> bool {
        self.currently_upgrading_to_chunk_loaders.contains(&chunk_loader_id)
    }

    pub fn are_chunk_loaders_upgrading_to(&self, chunk_loader_ids: HashSet<ChunkLoaderID>) -> bool {
        for chunk_loader_id in chunk_loader_ids {
            if !self.currently_upgrading_to_chunk_loaders.contains(&chunk_loader_id) {
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

    pub fn get_loaded_chunk_loader(&self, chunk_loader_id: ChunkLoaderID) -> Option<EntityReference> {
        self.loaded_chunk_loaders.get(&chunk_loader_id).copied()
    }

    pub fn loaded_chunk_loader(&self, chunk_loader_id: ChunkLoaderID) -> EntityReference {
        self.loaded_chunk_loaders[&chunk_loader_id]
    }

    pub fn loaded_chunk_loaders(&self) -> &HashMap<ChunkLoaderID, EntityReference> {
        &self.loaded_chunk_loaders
    }

    pub fn loaded_chunk_loaders_mut(&mut self) -> &mut HashMap<ChunkLoaderID, EntityReference> {
        &mut self.loaded_chunk_loaders
    }

    pub fn get_started_chunk_loader(&self, chunk_loader_id: ChunkLoaderID) -> Option<EntityReference> {
        self.started_chunk_loaders.get(&chunk_loader_id).copied()
    }

    pub fn started_chunk_loader(&self, chunk_loader_id: ChunkLoaderID) -> EntityReference {
        self.started_chunk_loaders[&chunk_loader_id]
    }

    pub fn started_chunk_loaders(&self) -> &HashMap<ChunkLoaderID, EntityReference> {
        &self.started_chunk_loaders
    }

    pub fn started_chunk_loaders_mut(&mut self) -> &mut HashMap<ChunkLoaderID, EntityReference> {
        &mut self.started_chunk_loaders
    }

    pub fn currently_creating_chunk_loaders(&self) -> &HashSet<ChunkLoaderID> {
        &self.currently_creating_chunk_loaders
    }

    pub fn currently_creating_chunk_loaders_mut(&mut self) -> &mut HashSet<ChunkLoaderID> {
        &mut self.currently_creating_chunk_loaders
    }

    pub fn currently_destroying_chunk_loaders(&self) -> &HashSet<ChunkLoaderID> {
        &self.currently_destroying_chunk_loaders
    }

    pub fn currently_destroying_chunk_loaders_mut(&mut self) -> &mut HashSet<ChunkLoaderID> {
        &mut self.currently_destroying_chunk_loaders
    }

    pub fn currently_upgrading_to_chunk_loaders(&self) -> &HashSet<ChunkLoaderID> {
        &self.currently_upgrading_to_chunk_loaders
    }

    pub fn currently_upgrading_to_chunk_loaders_mut(&mut self) -> &mut HashSet<ChunkLoaderID> {
        &mut self.currently_upgrading_to_chunk_loaders
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

#[derive(Resource, Debug, Default)]
pub(in crate) struct ChunkLoaderEventRegistry {
    next_chunk_loader_request_id: ChunkLoaderRequestID,
}

impl ChunkLoaderEventRegistry {
    pub fn get_unused_chunk_loader_request_id(&mut self) -> ChunkLoaderRequestID {
        let chunk_loader_request_id = self.next_chunk_loader_request_id;
        self.next_chunk_loader_request_id = ChunkLoaderRequestID(chunk_loader_request_id.0 + 1);

        chunk_loader_request_id
    }
}