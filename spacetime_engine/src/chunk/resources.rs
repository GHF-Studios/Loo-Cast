use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum ChunkRetryAction {
    Spawn {
        chunk_coord: (i32, i32),
        chunk_owner: Entity,
    },
    Despawn {
        chunk_coord: (i32, i32),
        chunk_entity: Entity,
    },
}

#[derive(Resource, Default, Debug)]
pub struct ChunkRetryQueue {
    pub actions: VecDeque<ChunkRetryAction>,
}