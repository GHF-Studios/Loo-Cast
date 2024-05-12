use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::chunk::id::structs::*;
use crate::chunk::actor::id::structs::*;

#[derive(Resource, Debug, Default)]
pub struct ChunkRegistry {
    registered_chunks: HashSet<ChunkID>,
    loaded_chunks: HashMap<ChunkID, Entity>,
    serialized_chunks: HashMap<ChunkID, String>,
    currently_creating_chunks: HashSet<ChunkID>,
    currently_destroying_chunks: HashSet<ChunkID>,
    currently_loading_chunks: HashSet<ChunkID>,
    currently_unloading_chunks: HashSet<ChunkID>,
    registered_chunk_actors: HashSet<ChunkActorID>,
    loaded_chunk_actors: HashMap<ChunkActorID, Entity>,
    next_chunk_actor_id: ChunkActorID,
    recycled_chunk_actor_ids: Vec<ChunkActorID>,
}

impl ChunkRegistry {
    pub fn register_chunk(&mut self, chunk_id: ChunkID) {
        self.registered_chunks.insert(chunk_id);
    }

    pub fn register_chunks(&mut self, chunk_ids: Vec<ChunkID>) {
        self.registered_chunks.extend(chunk_ids);
    }

    pub fn unregister_chunk(&mut self, chunk_id: ChunkID) {
        self.registered_chunks.remove(&chunk_id);
    }

    pub fn unregister_chunks(&mut self, chunk_ids: HashSet<ChunkID>) {
        self.registered_chunks.retain(|&chunk_id| !chunk_ids.contains(&chunk_id));
    }

    pub fn load_chunk(&mut self, chunk_id: ChunkID, entity: Entity) {
        self.loaded_chunks.insert(chunk_id, entity);
    }

    pub fn load_chunks(&mut self, chunk_entities: HashMap<ChunkID, Entity>) {
        self.loaded_chunks.extend(chunk_entities);
    }

    pub fn unload_chunk(&mut self, chunk_id: ChunkID) -> Option<Entity> {
        self.loaded_chunks.remove(&chunk_id)
    }

    pub fn unload_chunks(&mut self, chunk_ids: HashSet<ChunkID>) {
        self.loaded_chunks.retain(|&chunk_id, _| !chunk_ids.contains(&chunk_id));
    }

    pub fn serialize_chunk(&mut self, chunk_id: ChunkID, serialized_chunk: String) {
        self.serialized_chunks.insert(chunk_id, serialized_chunk);
    }

    pub fn serialize_chunks(&mut self, serialized_chunks: HashMap<ChunkID, String>) {
        self.serialized_chunks.extend(serialized_chunks);
    }

    pub fn deserialize_chunk(&mut self, chunk_id: ChunkID) -> Option<String> {
        self.serialized_chunks.remove(&chunk_id)
    }

    pub fn deserialize_chunks(&mut self, chunk_ids: HashSet<ChunkID>) -> HashMap<ChunkID, Option<String>> {
        let mut deserialized_chunks = HashMap::new();

        for chunk_id in chunk_ids {
            deserialized_chunks.insert(chunk_id, self.serialized_chunks.remove(&chunk_id));
        }

        deserialized_chunks
    }

    pub fn start_creating_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_creating_chunks.insert(chunk_id);
    }

    pub fn start_creating_chunks(&mut self, chunk_ids: HashSet<ChunkID>) {
        self.currently_creating_chunks.extend(chunk_ids);
    }

    pub fn stop_creating_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_creating_chunks.remove(&chunk_id);
    }

    pub fn stop_creating_chunks(&mut self, chunk_ids: HashSet<ChunkID>) {
        self.currently_creating_chunks.retain(|&chunk_id| !chunk_ids.contains(&chunk_id));
    }

    pub fn start_destroying_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_destroying_chunks.insert(chunk_id);
    }

    pub fn start_destroying_chunks(&mut self, chunk_ids: HashSet<ChunkID>) {
        self.currently_destroying_chunks.extend(chunk_ids);
    }

    pub fn stop_destroying_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_destroying_chunks.remove(&chunk_id);
    }

    pub fn stop_destroying_chunks(&mut self, chunk_ids: HashSet<ChunkID>) {
        self.currently_destroying_chunks.retain(|&chunk_id| !chunk_ids.contains(&chunk_id));
    }

    pub fn start_loading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_loading_chunks.insert(chunk_id);
    }

    pub fn start_loading_chunks(&mut self, chunk_ids: HashSet<ChunkID>) {
        self.currently_loading_chunks.extend(chunk_ids);
    }

    pub fn stop_loading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_loading_chunks.remove(&chunk_id);
    }

    pub fn stop_loading_chunks(&mut self, chunk_ids: HashSet<ChunkID>) {
        self.currently_loading_chunks.retain(|&chunk_id| !chunk_ids.contains(&chunk_id));
    }

    pub fn start_unloading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_unloading_chunks.insert(chunk_id);
    }

    pub fn start_unloading_chunks(&mut self, chunk_ids: HashSet<ChunkID>) {
        self.currently_unloading_chunks.extend(chunk_ids);
    }

    pub fn stop_unloading_chunk(&mut self, chunk_id: ChunkID) {
        self.currently_unloading_chunks.remove(&chunk_id);
    }

    pub fn stop_unloading_chunks(&mut self, chunk_ids: HashSet<ChunkID>) {
        self.currently_unloading_chunks.retain(|&chunk_id| !chunk_ids.contains(&chunk_id));
    }

    pub fn is_chunk_registered(&self, chunk_id: ChunkID) -> bool {
        self.registered_chunks.contains(&chunk_id)
    }

    pub fn are_chunks_registered(&self, chunk_ids: HashSet<ChunkID>) -> bool {
        for chunk_id in chunk_ids {
            if !self.registered_chunks.contains(&chunk_id) {
                return false;
            }
        }

        true
    }

    pub fn is_chunk_loaded(&self, chunk_id: ChunkID) -> bool {
        self.loaded_chunks.contains_key(&chunk_id)
    }

    pub fn are_chunks_loaded(&self, chunk_ids: HashSet<ChunkID>) -> bool {
        for chunk_id in chunk_ids {
            if !self.loaded_chunks.contains_key(&chunk_id) {
                return false;
            }
        }

        true
    }

    pub fn is_chunk_serialized(&self, chunk_id: ChunkID) -> bool {
        self.serialized_chunks.contains_key(&chunk_id)
    }

    pub fn are_chunks_serialized(&self, chunk_ids: HashSet<ChunkID>) -> bool {
        for chunk_id in chunk_ids {
            if !self.serialized_chunks.contains_key(&chunk_id) {
                return false;
            }
        }

        true
    }

    pub fn is_creating_chunk(&self, chunk_id: ChunkID) -> bool {
        self.currently_creating_chunks.contains(&chunk_id)
    }

    pub fn are_chunks_creating(&self, chunk_ids: HashSet<ChunkID>) -> bool {
        for chunk_id in chunk_ids {
            if !self.currently_creating_chunks.contains(&chunk_id) {
                return false;
            }
        }

        true
    }

    pub fn is_destroying_chunk(&self, chunk_id: ChunkID) -> bool {
        self.currently_destroying_chunks.contains(&chunk_id)
    }

    pub fn are_chunks_destroying(&self, chunk_ids: HashSet<ChunkID>) -> bool {
        for chunk_id in chunk_ids {
            if !self.currently_destroying_chunks.contains(&chunk_id) {
                return false;
            }
        }

        true
    }

    pub fn is_loading_chunk(&self, chunk_id: ChunkID) -> bool {
        self.currently_loading_chunks.contains(&chunk_id)
    }

    pub fn are_chunks_loading(&self, chunk_ids: HashSet<ChunkID>) -> bool {
        for chunk_id in chunk_ids {
            if !self.currently_loading_chunks.contains(&chunk_id) {
                return false;
            }
        }

        true
    }

    pub fn is_unloading_chunk(&self, chunk_id: ChunkID) -> bool {
        self.currently_unloading_chunks.contains(&chunk_id)
    }

    pub fn are_chunks_unloading(&self, chunk_ids: HashSet<ChunkID>) -> bool {
        for chunk_id in chunk_ids {
            if !self.currently_unloading_chunks.contains(&chunk_id) {
                return false;
            }
        }

        true
    }

    pub fn registered_chunks(&self) -> &HashSet<ChunkID> {
        &self.registered_chunks
    }

    pub fn registered_chunks_mut(&mut self) -> &mut HashSet<ChunkID> {
        &mut self.registered_chunks
    }

    pub fn get_loaded_chunk_entity(&self, chunk_id: ChunkID) -> Option<Entity> {
        self.loaded_chunks.get(&chunk_id).copied()
    }

    pub fn loaded_chunk_entity(&self, chunk_id: ChunkID) -> Entity {
        self.loaded_chunks[&chunk_id]
    }

    pub fn loaded_chunks(&self) -> &HashMap<ChunkID, Entity> {
        &self.loaded_chunks
    }

    pub fn loaded_chunks_mut(&mut self) -> &mut HashMap<ChunkID, Entity> {
        &mut self.loaded_chunks
    }

    pub fn loaded_chunk_ids(&self) -> HashSet<ChunkID> {
        self.loaded_chunks.keys().copied().collect()
    }

    pub fn loaded_chunk_entities(&self) -> HashSet<Entity> {
        self.loaded_chunks.values().copied().collect()
    }

    pub fn serialized_chunks(&self) -> &HashMap<ChunkID, String> {
        &self.serialized_chunks
    }

    pub fn serialized_chunks_mut(&mut self) -> &mut HashMap<ChunkID, String> {
        &mut self.serialized_chunks
    }

    pub fn creating_chunks(&self) -> &HashSet<ChunkID> {
        &self.currently_creating_chunks
    }

    pub fn destroying_chunks(&self) -> &HashSet<ChunkID> {
        &self.currently_destroying_chunks
    }

    pub fn loading_chunks(&self) -> &HashSet<ChunkID> {
        &self.currently_loading_chunks
    }

    pub fn unloading_chunks(&self) -> &HashSet<ChunkID> {
        &self.currently_unloading_chunks
    }

    pub fn register_chunk_actor(&mut self) -> ChunkActorID {
        let chunk_actor_id = self.get_unused_chunk_actor_id();
        self.registered_chunk_actors.insert(chunk_actor_id);

        chunk_actor_id
    }

    pub fn register_chunk_actors(&mut self, chunk_actor_ids: HashSet<ChunkActorID>) {
        self.registered_chunk_actors.extend(chunk_actor_ids);
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

    pub fn load_chunk_actor(&mut self, chunk_actor_id: ChunkActorID, entity: Entity) {
        self.loaded_chunk_actors.insert(chunk_actor_id, entity);
    }

    pub fn load_chunk_actors(&mut self, chunk_actor_entities: HashMap<ChunkActorID, Entity>) {
        self.loaded_chunk_actors.extend(chunk_actor_entities);
    }

    pub fn unload_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) -> Option<Entity> {
        self.loaded_chunk_actors.remove(&chunk_actor_id)
    }

    pub fn unload_chunk_actors(&mut self, chunk_actor_ids: HashSet<ChunkActorID>) {
        self.loaded_chunk_actors.retain(|&chunk_actor_id, _| !chunk_actor_ids.contains(&chunk_actor_id));
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

    pub fn registered_chunk_actors(&self) -> &HashSet<ChunkActorID> {
        &self.registered_chunk_actors
    }

    pub fn registered_chunk_actors_mut(&mut self) -> &mut HashSet<ChunkActorID> {
        &mut self.registered_chunk_actors
    }

    pub fn get_loaded_chunk_actor_entity(&self, chunk_actor_id: ChunkActorID) -> Option<Entity> {
        self.loaded_chunk_actors.get(&chunk_actor_id).copied()
    }

    pub fn loaded_chunk_actor_entity(&self, chunk_actor_id: ChunkActorID) -> Entity {
        self.loaded_chunk_actors[&chunk_actor_id]
    }

    pub fn loaded_chunk_actors(&self) -> &HashMap<ChunkActorID, Entity> {
        &self.loaded_chunk_actors
    }

    pub fn loaded_chunk_actors_mut(&mut self) -> &mut HashMap<ChunkActorID, Entity> {
        &mut self.loaded_chunk_actors
    }

    pub fn loaded_chunk_actor_ids(&self) -> HashSet<ChunkActorID> {
        self.loaded_chunk_actors.keys().copied().collect()
    }

    pub fn loaded_chunk_actor_entities(&self) -> HashSet<Entity> {
        self.loaded_chunk_actors.values().copied().collect()
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
