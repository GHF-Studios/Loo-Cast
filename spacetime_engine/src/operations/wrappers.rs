use crate::core::{structs::*, traits::*};
use bevy::prelude::*;

#[derive(Deref, DerefMut)]
pub struct OperationTypeRegistry(TypeRegistry);
impl OperationTypeRegistry {
    pub fn new() -> Self {
        Self(TypeRegistry::new())
    }
}
impl LockingNodePartialData for OperationTypeRegistry {}
impl LockingNodeData for OperationTypeRegistry {}