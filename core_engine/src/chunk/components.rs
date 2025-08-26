use bevy::{ecs::component::StorageType, prelude::*};

use crate::chunk_loader::resources::{RemovedChunkLoader, RemovedChunkLoaders};

use super::types::ChunkOwnerId;

#[derive(Default, Debug)]
pub struct Chunk {
    pub coord: (i32, i32),
    pub(crate) owner_id: Option<ChunkOwnerId>,
}
impl Chunk {
    pub fn owner_id(&self) -> &ChunkOwnerId {
        self.owner_id.as_ref().expect("Unreachable state: Chunk has no owner_id")
    }
}

impl Component for Chunk {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            let (chunk, chunk_coord) = match world.get::<Chunk>(entity) {
                Some(chunk) => (chunk, chunk.coord),
                None => return,
            };

            let chunk_owner_id = match chunk.owner_id.clone() {
                Some(owner_id) => owner_id,
                None => return,
            };

            let removed_chunk_loaders = world.resource_mut::<RemovedChunkLoaders>();
            let found_removal_event = removed_chunk_loaders.0.iter().any(|rcl| rcl.id == chunk_owner_id);

            if world.get_entity(chunk_owner_id.entity()).is_err() && !found_removal_event {
                panic!(
                    "Spawned chunk {:?} with non-existent owner_id {:?}. 
                    The chunk cannot be safely despawned, due to no respective removal event being found.
                    In other words: The chunk references a non-existent owner at creation, which is an invariant violation in combination with no removal event being found!", 
                    chunk_coord, chunk_owner_id
                );
            }
        });
    }
}
