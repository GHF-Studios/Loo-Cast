use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub(in crate) struct UpgradeToChunkLoader(pub ChunkLoaderRequest);

#[derive(Debug, Clone, Event)]
pub(in crate) struct DowngradeFromChunkLoader(pub ChunkLoaderRequest);

#[derive(Debug, Clone, Event)]
pub struct UpgradedToChunkLoader(pub ChunkLoaderResponse);

#[derive(Debug, Clone, Event)]
pub struct DowngradedFromChunkLoader(pub ChunkLoaderResponse);
