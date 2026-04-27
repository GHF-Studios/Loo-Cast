use bevy::prelude::*;

use crate::core::structs::*;
use crate::operation::wrappers::*;
use super::components::ChunkLoader;

#[derive(Deref, DerefMut)]
pub struct ChunkLoaderInstanceRegistry(DynamicInstanceRegistry<NumericID<ChunkLoader>, Entity>);
impl ChunkLoaderInstanceRegistry {
    pub fn new() -> Self {
        Self(DynamicInstanceRegistry::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct ChunkLoaderOperationTypeRegistry(OperationTypeRegistry);
impl ChunkLoaderOperationTypeRegistry {
    pub fn new() -> Self {
        Self(OperationTypeRegistry::new())
    }
}
