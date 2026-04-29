use bevy::prelude::*;
use crate::chunk::actor::id::structs::ChunkActorRequestID;

use super::id::structs::PlayerID;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub id: PlayerID,
    pub(super) create_chunk_actor_request_ids: Vec<ChunkActorRequestID>
}