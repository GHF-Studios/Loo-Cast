use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub struct PromoteToChunkActor(pub ChunkActorRequest);

#[derive(Debug, Clone, Event)]
pub struct DemoteFromChunkActor(pub ChunkActorRequest);

#[derive(Debug, Clone, Event)]
pub struct PromoteToChunkActorInternal(pub InternalChunkActorRequest);

#[derive(Debug, Clone, Event)]
pub struct DemoteFromChunkActorInternal(pub InternalChunkActorRequest);

#[derive(Debug, Clone, Event)]
pub struct PromotedToChunkActorInternal(pub InternalChunkActorResponse);

#[derive(Debug, Clone, Event)]
pub struct DemotedFromChunkActorInternal(pub InternalChunkActorResponse);

#[derive(Debug, Clone, Event)]
pub struct PromotedToChunkActor(pub ChunkActorResponse);

#[derive(Debug, Clone, Event)]
pub struct DemotedFromChunkActor(pub ChunkActorResponse);
