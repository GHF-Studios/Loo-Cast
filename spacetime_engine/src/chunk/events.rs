use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunk(pub ChunkRequest);

#[derive(Debug, Clone, Event)]
pub struct DowngradeFromChunk(pub ChunkRequest);

#[derive(Debug, Clone, Event)]
pub struct LoadChunk(pub ChunkRequest);

#[derive(Debug, Clone, Event)]
pub struct SaveChunk(pub ChunkRequest);

#[derive(Debug, Clone, Event)]
pub struct UpgradedToChunk(pub ChunkResponse);

#[derive(Debug, Clone, Event)]
pub struct DowngradedFromChunk(pub ChunkResponse);

#[derive(Debug, Clone, Event)]
pub struct LoadedChunk(pub ChunkResponse);

#[derive(Debug, Clone, Event)]
pub struct SavedChunk(pub ChunkResponse);