use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunkActor(pub ChunkActorRequest);

#[derive(Debug, Clone, Event)]
pub struct DowngradeFromChunkActor(pub ChunkActorRequest);

#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunkActorInternal(pub InternalChunkActorRequest);

#[derive(Debug, Clone, Event)]
pub struct DowngradeFromChunkActorInternal(pub InternalChunkActorRequest);

#[derive(Debug, Clone, Event)]
pub struct UpgradedToChunkActorInternal(pub InternalChunkActorResponse);

#[derive(Debug, Clone, Event)]
pub struct DowngradedFromChunkActorInternal(pub InternalChunkActorResponse);

#[derive(Debug, Clone, Event)]
pub struct UpgradedToChunkActor(pub ChunkActorResponse);

#[derive(Debug, Clone, Event)]
pub struct DowngradedFromChunkActor(pub ChunkActorResponse);

#[derive(Debug, Clone, Event)]
pub struct StartedChunkActor(pub ChunkActorResponse);

#[derive(Debug, Clone, Event)]
pub struct StoppedChunkActor(pub ChunkActorResponse);
