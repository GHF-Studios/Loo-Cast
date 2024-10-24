use crate::core::{structs::*, traits::*};
use bevy::prelude::*;

#[derive(Deref, DerefMut)]
pub struct OperationTypeRegistry(TypeRegistry);
impl OperationTypeRegistry {
    pub fn new(operation_type_registry_id: &'static str) -> Self {
        Self(TypeRegistry::new(operation_type_registry_id))
    }
}
impl LockingNodePartialData for OperationTypeRegistry {}
impl LockingNodeData for OperationTypeRegistry {}
impl LockingNode for OperationTypeRegistry {
    fn node_info(&self) -> LockingNodeInfo {
        self.0.node_info()
    }
}
impl LockingNodeParent for OperationTypeRegistry {}
impl LockingNodeChild for OperationTypeRegistry {}
impl LockingNodeParentChild for OperationTypeRegistry {}
