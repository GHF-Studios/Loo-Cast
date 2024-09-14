use bevy::prelude::*;
use crate::operations::{structs::InstanceID, traits::Operation};
use super::components::*;

pub struct UpgradeToChunkLoaderArgs {
    pub target_entity_id: InstanceID<Entity>,
    pub chunk_loader_load_radius: u16
}
pub enum UpgradeToChunkLoaderResult {
    Ok{
        chunk_loader_id: InstanceID<ChunkLoader>,
    },
    Err(()),
}
pub struct UpgradeToChunkLoader {
    args: UpgradeToChunkLoaderArgs,
    callback: fn(UpgradeToChunkLoaderResult),
}
impl UpgradeToChunkLoader {
    pub fn new(args: UpgradeToChunkLoaderArgs, callback: Option<fn(UpgradeToChunkLoaderResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for UpgradeToChunkLoader {
    fn execute(&self, world: &mut World) {
        // Step 1: Error if the chunk loader is present in the world
        // Step 2: Error if the chunk loader is present in the serialized chunk storage
        // Step 3: Insert the chunk loader component into the target entity
    }
}

pub struct DowngradeFromChunkLoaderArgs {
    pub chunk_loader_entity_id: InstanceID<Entity>,
    pub chunk_loader_id: InstanceID<ChunkLoader>,
}
pub enum DowngradeFromChunkLoaderResult {
    Ok(()),
    Err(()),
}
pub struct DowngradeFromChunkLoader {
    args: DowngradeFromChunkLoaderArgs,
    callback: fn(DowngradeFromChunkLoaderResult),
}
impl DowngradeFromChunkLoader {
    pub fn new(args: DowngradeFromChunkLoaderArgs, callback: Option<fn(DowngradeFromChunkLoaderResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for DowngradeFromChunkLoader {
    fn execute(&self, world: &mut World) {
        // Step 1: Error if the chunk loader is not actually a chunk loader
        // Step 2: Error if the chunk loader is marked as serialized
        // Step 3: Error if the chunk loader is present in the serialized chunk storage
        // Step 4: Error if the chunk loader is not managed
        // Step 5: Error if the chunk loader is not registered
        // Step 6: Remove the chunk loader component from the chunk loader entity
    }
}
