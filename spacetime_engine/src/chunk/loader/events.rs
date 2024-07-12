use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunkLoader(pub ChunkLoaderRequest);

#[derive(Debug, Clone, Event)]
pub struct DowngradeFromChunkLoader(pub ChunkLoaderRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct UpgradeToChunkLoaderInternal(pub InternalChunkLoaderRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct DowngradeFromChunkLoaderInternal(pub InternalChunkLoaderRequest);

#[derive(Debug, Clone, Event)]
pub(crate) struct UpgradedToChunkLoaderInternal(pub InternalChunkLoaderResponse);

#[derive(Debug, Clone, Event)]
pub(crate) struct DowngradedFromChunkLoaderInternal(pub InternalChunkLoaderResponse);

#[derive(Debug, Clone, Event)]
pub struct UpgradedToChunkLoader(pub ChunkLoaderResponse);

#[derive(Debug, Clone, Event)]
pub struct DowngradedFromChunkLoader(pub ChunkLoaderResponse);

#[derive(Debug, Clone, Event)]
pub struct StartedChunkLoader(pub ChunkLoaderResponse);

#[derive(Debug, Clone, Event)]
pub struct StoppedChunkLoader(pub ChunkLoaderResponse);
