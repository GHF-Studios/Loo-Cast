use bevy::prelude::*;
use crate::chunk::actor::id::structs::ChunkActorEventID;

use super::structs::PlayerID;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub id: PlayerID,
    pub(super) create_chunk_actor_event_ids: Vec<ChunkActorEventID>
}