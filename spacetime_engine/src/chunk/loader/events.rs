use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub struct PromoteToChunkLoader(pub ChunkLoaderRequest);

#[derive(Debug, Clone, Event)]
pub struct DemoteFromChunkLoader(pub ChunkLoaderRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct PromoteToChunkLoaderInternal(pub InternalChunkLoaderRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct DemoteFromChunkLoaderInternal(pub InternalChunkLoaderRequest);

#[derive(Debug, Clone, Event)]
pub(crate) struct PromotedToChunkLoaderInternal(pub InternalChunkLoaderResponse);

#[derive(Debug, Clone, Event)]
pub(crate) struct DemotedFromChunkLoaderInternal(pub InternalChunkLoaderResponse);

#[derive(Debug, Clone, Event)]
pub struct PromotedToChunkLoader(pub ChunkLoaderResponse);

#[derive(Debug, Clone, Event)]
pub struct DemotedFromChunkLoader(pub ChunkLoaderResponse);
