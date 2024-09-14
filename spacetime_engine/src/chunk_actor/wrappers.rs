use bevy::prelude::*;
use crate::operations::{structs::{InstanceID, InstanceRegistry}, wrappers::OperationTypeRegistry};
use super::components::ChunkActor;

#[derive(Deref, DerefMut)]
pub struct ChunkActorInstanceRegistry(InstanceRegistry<InstanceID<ChunkActor>, Entity>);
impl ChunkActorInstanceRegistry {
    pub fn new() -> Self {
        Self(InstanceRegistry::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct ChunkActorOperationTypeRegistry(OperationTypeRegistry);
impl ChunkActorOperationTypeRegistry {
    pub fn new() -> Self {
        Self(OperationTypeRegistry::new())
    }
}
