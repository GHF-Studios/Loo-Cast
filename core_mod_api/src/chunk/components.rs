use bevy::{
    ecs::component::{ComponentHook, Mutable, StorageType},
    prelude::*,
};

use crate::chunk::hooks::hook_on_add_chunk;
use crate::chunk_loader::types::ChunkLoaderId;
use crate::usf::pos::grid::types::GridVec;

#[derive(Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Chunk {
    pub coord: GridVec,
    pub(crate) owner_id: Option<ChunkLoaderId>,
}
impl Chunk {
    pub fn owner_id(&self) -> &ChunkLoaderId {
        self.owner_id.as_ref().expect("Unreachable state: Chunk has no owner_id")
    }
}

impl Component for Chunk {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;

    fn on_add() -> Option<ComponentHook> {
        Some(hook_on_add_chunk)
    }
}
