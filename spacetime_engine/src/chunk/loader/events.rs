use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunkLoader(pub ChunkLoaderRequest);

#[derive(Debug, Clone, Event)]
pub struct DowngradeFromChunkLoader(pub ChunkLoaderRequest);

#[derive(Debug, Clone, Event)]
pub struct UpgradedToChunkLoader(pub ChunkLoaderResponse);

#[derive(Debug, Clone, Event)]
pub struct DowngradedFromChunkLoader(pub ChunkLoaderResponse);
