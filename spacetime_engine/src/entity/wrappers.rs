use bevy::prelude::*;
use crate::core::structs::*;
use crate::core::traits::*;
use crate::operations::wrappers::*;

#[derive(Deref, DerefMut)]
pub struct EntityInstanceRegistry(DynamicInstanceRegistry<DynamicID<Entity>, Entity>);
impl EntityInstanceRegistry {
    pub fn new() -> Self {
        Self(DynamicInstanceRegistry::new())
    }
}
impl LockingNodePartialData for EntityInstanceRegistry {}
impl LockingNodeData for EntityInstanceRegistry {}

#[derive(Deref, DerefMut)]
pub struct EntityOperationTypeRegistry(OperationTypeRegistry);
impl EntityOperationTypeRegistry {
    pub fn new() -> Self {
        Self(OperationTypeRegistry::new())
    }
}
impl LockingNodePartialData for EntityOperationTypeRegistry {}
impl LockingNodeData for EntityOperationTypeRegistry {}
