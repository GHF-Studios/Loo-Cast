use bevy::{
    ecs::component::{ComponentHook, Mutable, StorageType},
    prelude::*,
};

use crate::chunk::hooks::hook_on_add_chunk;
use crate::usf::scale::Scale;

use super::types::ChunkOwnerId;

#[derive(Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Chunk {
    pub coord: (i32, i32),
    pub(crate) owner_id: Option<ChunkOwnerId>,
    pub scale: Scale,
}
impl Chunk {
    pub fn owner_id(&self) -> &ChunkOwnerId {
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
