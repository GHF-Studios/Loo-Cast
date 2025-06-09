use bevy::{ecs::component::StorageType, prelude::*};

use super::types::ChunkOwnerId;

#[derive(Default, Debug)]
pub struct ChunkComponent {
    pub coord: (i32, i32),
    pub(crate) owner: Option<ChunkOwnerId>,
}
impl ChunkComponent {
    pub fn owner(&self) -> &ChunkOwnerId {
        self.owner.as_ref().expect("Unreachable state: Chunk has no owner")
    }
}

impl Component for ChunkComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|world, entity, _component_id| {
            let chunk = match world.get::<ChunkComponent>(entity) {
                Some(chunk) => chunk,
                None => return,
            };

            let chunk_owner_id = match chunk.owner.clone() {
                Some(owner) => owner,
                None => return,
            };

            if world.get_entity(chunk_owner_id.entity()).is_none() {
                error!("Spawned chunk {:?} with non-existent owner {:?}", chunk.coord, chunk_owner_id);
            }
        });
    }
}
