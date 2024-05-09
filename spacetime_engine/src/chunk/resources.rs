use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::chunk::id::structs::*;
use crate::chunk::actor::id::structs::*;

#[derive(Resource, Debug, Default)]
pub struct ChunkManager {
    pub(in crate) registered_chunks: HashSet<ChunkID>,
    pub(in crate) loaded_chunks: HashMap<ChunkID, Entity>,
    pub(in crate) serialized_chunks: HashMap<ChunkID, String>,
    pub(in crate) creating_chunks: HashSet<ChunkID>,
    pub(in crate) destroying_chunks: HashSet<ChunkID>,
    pub(in crate) loading_chunks: HashSet<ChunkID>,
    pub(in crate) unloading_chunks: HashSet<ChunkID>,
    pub(in crate) registered_chunk_actors: HashSet<ChunkActorID>,
    pub(in crate) loaded_chunk_actors: HashMap<ChunkActorID, Entity>,
    pub(in crate) creating_chunk_actors: HashSet<ChunkActorID>,
    pub(in crate) destroying_chunk_actors: HashSet<ChunkActorID>,
    pub(in crate) loading_chunk_actors: HashSet<ChunkActorID>,
    pub(in crate) unloading_chunk_actors: HashSet<ChunkActorID>,
    pub(in crate) current_chunk_actor_id: ChunkActorID,
    pub(in crate) recycled_chunk_actor_ids: Vec<ChunkActorID>,
}

impl ChunkManager {
    pub fn get_unused_chunk_actor_id(&mut self) -> ChunkActorID {
        if let Some(recycled_chunk_actor_id) = self.recycled_chunk_actor_ids.pop() {
            recycled_chunk_actor_id
        } else {
            let new_chunk_actor_id = self.current_chunk_actor_id;
            self.current_chunk_actor_id = ChunkActorID(new_chunk_actor_id.0 + 1);

            new_chunk_actor_id
        }
    }

    pub fn recycle_chunk_actor_id(&mut self, chunk_actor_id: ChunkActorID) {
        self.recycled_chunk_actor_ids.push(chunk_actor_id);
    }
}
