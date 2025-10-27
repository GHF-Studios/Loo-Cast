use bevy::{
    ecs::component::{ComponentHook, Mutable, StorageType},
    prelude::*,
};

use crate::chunk::types::GridCoord;
use crate::chunk_loader::types::ChunkLoaderId;
use crate::chunk::hooks::hook_on_add_chunk;
use crate::usf::scale::Scale;

#[derive(Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Chunk {
    pub coord: GridCoord,
    pub(crate) owner_id: Option<ChunkLoaderId>,
    pub scale: Scale,
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
