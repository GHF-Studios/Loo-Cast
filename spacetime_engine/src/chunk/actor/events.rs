use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub(in crate) struct UpgradeToChunkActor(pub ChunkActorRequest);

#[derive(Debug, Clone, Event)]
pub(in crate) struct DowngradeFromChunkActor(pub ChunkActorRequest);

#[derive(Debug, Clone, Event)]
pub struct UpgradedToChunkActor(pub ChunkActorResponse);

#[derive(Debug, Clone, Event)]
pub struct DowngradedFromChunkActor(pub ChunkActorResponse);
