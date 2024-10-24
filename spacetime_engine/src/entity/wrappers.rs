use bevy::prelude::*;
use crate::core::structs::*;
use crate::core::traits::*;
use crate::operations::wrappers::*;

#[derive(Deref, DerefMut)]
pub struct EntityInstanceRegistry(DynamicInstanceRegistry<DynamicID<Entity>, Entity>);
impl EntityInstanceRegistry {
    pub fn new() -> Self {
        Self(DynamicInstanceRegistry::new("EntityInstanceRegistry"))
    }
}
impl LockingNodePartialData for EntityInstanceRegistry {}
impl LockingNodeData for EntityInstanceRegistry {}
impl LockingNode for EntityInstanceRegistry {
    fn node_info(&self) -> LockingNodeInfo {
        self.0.node_info()
    }
}
impl LockingNodeParent for EntityInstanceRegistry {}
impl LockingNodeChild for EntityInstanceRegistry {}
impl LockingNodeParentChild for EntityInstanceRegistry {}

#[derive(Deref, DerefMut)]
pub struct EntityOperationTypeRegistry(OperationTypeRegistry);
impl EntityOperationTypeRegistry {
    pub fn new() -> Self {
        Self(OperationTypeRegistry::new("EntityOperationTypeRegistry"))
    }
}
impl LockingNodePartialData for EntityOperationTypeRegistry {}
impl LockingNodeData for EntityOperationTypeRegistry {}
impl LockingNode for EntityOperationTypeRegistry {
    fn node_info(&self) -> LockingNodeInfo {
        self.0.node_info()
    }
}
impl LockingNodeParent for EntityOperationTypeRegistry {}
impl LockingNodeChild for EntityOperationTypeRegistry {}
impl LockingNodeParentChild for EntityOperationTypeRegistry {}
