use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use super::components::*;
use super::id::structs::*;

#[derive(Resource, Debug, Default)]
pub struct ChunkActorRegistry {
    registered_chunk_actors: HashSet<ChunkActorID>,
    loaded_chunk_actors: HashMap<ChunkActorID, Entity>,
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

    pub fn load_chunk_actor(&mut self, chunk_actor_id: ChunkActorID, chunk_actor_entity: Entity) {
        self.loaded_chunk_actors.insert(chunk_actor_id, chunk_actor_entity);
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