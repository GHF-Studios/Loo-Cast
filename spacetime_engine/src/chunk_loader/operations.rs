use bevy::prelude::*;
use crate::operations::traits::*;
use crate::core::structs::*;
use super::components::*;
use tokio::sync::oneshot;

pub struct UpgradeToChunkLoaderArgs {
    pub target_entity_id: InstanceID<Entity>,
    pub chunk_loader_load_radius: u16
}
impl OpArgs for UpgradeToChunkLoaderArgs {}
pub enum UpgradeToChunkLoaderResult {
    Ok{
        chunk_loader_id: InstanceID<ChunkLoader>,
    },
    Err(()),
}
impl OpResult for UpgradeToChunkLoaderResult {}
pub struct UpgradeToChunkLoader {
    args: UpgradeToChunkLoaderArgs,
    callback: Option<oneshot::Sender<UpgradeToChunkLoaderResult>>,
}
impl Operation for UpgradeToChunkLoader {
    type Args = UpgradeToChunkLoaderArgs;
    type Result = UpgradeToChunkLoaderResult;

    fn new(args: UpgradeToChunkLoaderArgs, callback: oneshot::Sender<UpgradeToChunkLoaderResult>) -> Self {
        Self {
            args,
            callback: Some(callback),
        }
    }

    fn execute(&mut self, world: &mut World) {
        // Step 1: Error if the chunk loader is present in the world
        // Step 2: Error if the chunk loader is present in the serialized chunk storage
        // Step 3: Insert the chunk loader component into the target entity
    }
}

pub struct DowngradeFromChunkLoaderArgs {
    pub chunk_loader_entity_id: InstanceID<Entity>,
    pub chunk_loader_id: InstanceID<ChunkLoader>,
}
impl OpArgs for DowngradeFromChunkLoaderArgs {}
pub enum DowngradeFromChunkLoaderResult {
    Ok(()),
    Err(()),
}
impl OpResult for DowngradeFromChunkLoaderResult {}
pub struct DowngradeFromChunkLoader {
    args: DowngradeFromChunkLoaderArgs,
    callback: Option<oneshot::Sender<DowngradeFromChunkLoaderResult>>,
}
impl Operation for DowngradeFromChunkLoader {
    type Args = DowngradeFromChunkLoaderArgs;
    type Result = DowngradeFromChunkLoaderResult;

    fn new(args: DowngradeFromChunkLoaderArgs, callback: oneshot::Sender<DowngradeFromChunkLoaderResult>) -> Self {
        Self {
            args,
            callback: Some(callback),
        }
    }

    fn execute(&mut self, world: &mut World) {
        // Step 1: Error if the chunk loader is not actually a chunk loader
        // Step 2: Error if the chunk loader is marked as serialized
        // Step 3: Error if the chunk loader is present in the serialized chunk storage
        // Step 4: Error if the chunk loader is not managed
        // Step 5: Error if the chunk loader is not registered
        // Step 6: Remove the chunk loader component from the chunk loader entity
    }
}
