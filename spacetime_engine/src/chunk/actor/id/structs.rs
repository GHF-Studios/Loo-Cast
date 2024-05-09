use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkActorID(pub u64);

impl From<u64> for ChunkActorID {
    fn from(chunk_actor_id: u64) -> Self {
        ChunkActorID(chunk_actor_id)
    }
}

impl From<ChunkActorID> for u64 {
    fn from(chunk_actor_id: ChunkActorID) -> Self {
        chunk_actor_id.0
    }
}
