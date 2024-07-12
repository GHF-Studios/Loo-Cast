use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunkActor(ChunkActorRequest);

#[derive(Debug, Clone, Event)]
pub struct DowngradeFromChunkActor(ChunkActorRequest);

#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunkActorInternal(InternalChunkActorRequest);

#[derive(Debug, Clone, Event)]
pub struct DowngradeFromChunkActorInternal(InternalChunkActorRequest);

#[derive(Debug, Clone, Event)]
pub struct UpgradedToChunkActorInternal(InternalChunkActorResponse);

#[derive(Debug, Clone, Event)]
pub struct DowngradedFromChunkActorInternal(InternalChunkActorResponse);

#[derive(Debug, Clone, Event)]
pub struct UpgradedToChunkActor(ChunkActorResponse);

#[derive(Debug, Clone, Event)]
pub struct DowngradedFromChunkActor(ChunkActorResponse);

#[derive(Debug, Clone, Event)]
pub struct StartedChunkActor(ChunkActorResponse);

#[derive(Debug, Clone, Event)]
pub struct StoppedChunkActor(ChunkActorResponse);
