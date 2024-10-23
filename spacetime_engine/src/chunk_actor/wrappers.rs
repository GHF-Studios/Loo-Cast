use bevy::prelude::*;
use crate::operations::wrappers::*;
use crate::core::structs::*;
use super::components::ChunkActor;

#[derive(Deref, DerefMut)]
pub struct ChunkActorInstanceRegistry(DynamicInstanceRegistry<DynamicID<ChunkActor>, Entity>);
impl ChunkActorInstanceRegistry {
    pub fn new() -> Self {
        Self(DynamicInstanceRegistry::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct ChunkActorOperationTypeRegistry(OperationTypeRegistry);
impl ChunkActorOperationTypeRegistry {
    pub fn new() -> Self {
        Self(OperationTypeRegistry::new())
    }
}
