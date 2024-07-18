use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub struct UpgradeToChunk(pub ChunkRequest);

#[derive(Debug, Clone, Event)]
pub struct DowngradeFromChunk(pub ChunkRequest);

#[derive(Debug, Clone, Event)]
pub struct DeserializeChunk(pub ChunkRequest);

#[derive(Debug, Clone, Event)]
pub struct SerializeChunk(pub ChunkRequest);

#[derive(Debug, Clone, Event)]
pub struct UpgradedToChunk(pub ChunkResponse);

#[derive(Debug, Clone, Event)]
pub struct DowngradedFromChunk(pub ChunkResponse);

#[derive(Debug, Clone, Event)]
pub struct DeserializedChunk(pub ChunkResponse);

#[derive(Debug, Clone, Event)]
pub struct SerializedChunk(pub ChunkResponse);