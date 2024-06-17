use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use super::id::structs::*;
use crate::entity::types::*;

#[derive(Resource, Debug, Default)]
pub(in crate) struct ChunkLoaderRegistry {
    registered_chunk_loaders: HashSet<ChunkLoaderID>,
    loaded_chunk_loaders: HashMap<ChunkLoaderID, EntityReference>,
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
    next_chunk_loader_event_id: ChunkLoaderEventID,
}

impl ChunkLoaderEventRegistry {
    pub fn get_unused_chunk_loader_event_id(&mut self) -> ChunkLoaderEventID {
        let chunk_loader_event_id = self.next_chunk_loader_event_id;
        self.next_chunk_loader_event_id = ChunkLoaderEventID(chunk_loader_event_id.0 + 1);

        chunk_loader_event_id
    }
}