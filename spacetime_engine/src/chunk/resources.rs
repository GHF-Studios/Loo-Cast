use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::chunk::id::structs::*;
use crate::chunk::actor::id::structs::*;

#[derive(Resource, Debug, Default)]
pub struct ChunkManager {
    registered_chunks: HashSet<ChunkID>,
    loaded_chunks: HashMap<ChunkID, Entity>,
    serialized_chunks: HashMap<ChunkID, String>,
    creating_chunks: HashSet<ChunkID>,
    destroying_chunks: HashSet<ChunkID>,
    loading_chunks: HashSet<ChunkID>,
    unloading_chunks: HashSet<ChunkID>,
    registered_chunk_actors: HashSet<ChunkActorID>,
    loaded_chunk_actors: HashMap<ChunkActorID, Entity>,
    creating_chunk_actors: HashSet<ChunkActorID>,
    destroying_chunk_actors: HashSet<ChunkActorID>,
    loading_chunk_actors: HashSet<ChunkActorID>,
    unloading_chunk_actors: HashSet<ChunkActorID>,
    current_chunk_actor_id: ChunkActorID,
    recycled_chunk_actor_ids: Vec<ChunkActorID>,
}

impl ChunkManager {
    pub fn register_chunk(&mut self, chunk_id: ChunkID) {
        self.registered_chunks.insert(chunk_id);
    }

    pub fn unregister_chunk(&mut self, chunk_id: ChunkID) {
        self.registered_chunks.remove(&chunk_id);
    }

    pub fn load_chunk(&mut self, chunk_id: ChunkID, entity: Entity) {
        self.loaded_chunks.insert(chunk_id, entity);
    }

    pub fn unload_chunk(&mut self, chunk_id: ChunkID) -> Option<Entity> {
        self.loaded_chunks.remove(&chunk_id)
    }

    pub fn is_chunk_registered(&self, chunk_id: ChunkID) -> bool {
        self.registered_chunks.contains(&chunk_id)
    }

    pub fn is_chunk_loaded(&self, chunk_id: ChunkID) -> bool {
        self.loaded_chunks.contains_key(&chunk_id)
    }

    pub fn get_chunk_entity(&self, chunk_id: ChunkID) -> Option<Entity> {
        self.loaded_chunks.get(&chunk_id).copied()
    }

    pub fn chunk_entity(&self, chunk_id: ChunkID) -> Entity {
        self.loaded_chunks[&chunk_id]
    }

    pub fn register_chunk_actor(&mut self) -> ChunkActorID {
        let chunk_actor_id = self.get_unused_chunk_actor_id();
        self.registered_chunk_actors.insert(chunk_actor_id);

        chunk_actor_id
    }

    pub fn unregister_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) {
        self.registered_chunk_actors.remove(&chunk_actor_id);
        self.recycle_chunk_actor_id(chunk_actor_id);
    }

    pub fn load_chunk_actor(&mut self, chunk_actor_id: ChunkActorID, entity: Entity) {
        self.loaded_chunk_actors.insert(chunk_actor_id, entity);
    }

    pub fn unload_chunk_actor(&mut self, chunk_actor_id: ChunkActorID) -> Option<Entity> {
        self.loaded_chunk_actors.remove(&chunk_actor_id)
    }

    pub fn is_chunk_actor_registered(&self, chunk_actor_id: ChunkActorID) -> bool {
        self.registered_chunk_actors.contains(&chunk_actor_id)
    }

    pub fn is_chunk_actor_loaded(&self, chunk_actor_id: ChunkActorID) -> bool {
        self.loaded_chunk_actors.contains_key(&chunk_actor_id)
    }

    pub fn get_chunk_actor_entity(&self, chunk_actor_id: ChunkActorID) -> Option<Entity> {
        self.loaded_chunk_actors.get(&chunk_actor_id).copied()
    }

    pub fn chunk_actor_entity(&self, chunk_actor_id: ChunkActorID) -> Entity {
        self.loaded_chunk_actors[&chunk_actor_id]
    }

    fn get_unused_chunk_actor_id(&mut self) -> ChunkActorID {
        if let Some(recycled_chunk_actor_id) = self.recycled_chunk_actor_ids.pop() {
            recycled_chunk_actor_id
        } else {
            let new_chunk_actor_id = self.current_chunk_actor_id;
            self.current_chunk_actor_id = ChunkActorID(new_chunk_actor_id.0 + 1);

            new_chunk_actor_id
        }
    }

    fn recycle_chunk_actor_id(&mut self, chunk_actor_id: ChunkActorID) {
        self.recycled_chunk_actor_ids.push(chunk_actor_id);
    }
}
