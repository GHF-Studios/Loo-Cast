use bevy::prelude::*;

use crate::core::structs::*;
use crate::operations::wrappers::*;

#[derive(Deref, DerefMut)]
pub struct EntityInstanceRegistry(DynamicInstanceRegistry<DynamicID<Entity>, Entity>);
impl EntityInstanceRegistry {
    pub fn new() -> Self {
        Self(DynamicInstanceRegistry::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct EntityOperationTypeRegistry(OperationTypeRegistry);
impl EntityOperationTypeRegistry {
    pub fn new() -> Self {
        Self(OperationTypeRegistry::new())
    }
}
