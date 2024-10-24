use std::any::{Any, TypeId};
use crate::core::structs::*;
use bevy::prelude::*;
use super::{enums::LockingState, traits::*};

#[derive(Deref, DerefMut)]
pub struct MainTypeRegistry(TypeRegistry);
impl MainTypeRegistry {
    pub fn new() -> Self {
        Self(TypeRegistry::new("MainTypeRegistry"))
    }
}
impl LockingNodePartialData for MainTypeRegistry {}
impl LockingNodeData for MainTypeRegistry {}
impl LockingNode for MainTypeRegistry {
    fn node_info(&self) -> LockingNodeInfo {
        self.0.node_info()
    }
}
impl LockingNodeParent for MainTypeRegistry {}
impl LockingNodeChild for MainTypeRegistry {}
impl LockingNodeParentChild for MainTypeRegistry {}

#[derive(Deref, DerefMut)]
pub struct Type(#[deref](TypeId, Box<dyn Any + Send + Sync>), LockingNodeInfo);
impl Type {
    pub fn new<T: 'static + Send + Sync, D: Any + Send + Sync>(data: D, static_id: &'static str) -> Self {
        Self(
            (TypeId::of::<T>(), Box::new(data)),
            LockingNodeInfo::new(
                LockingPathSegment::new_static::<Type>(static_id), 
                LockingState::Unlocked
            )
        )
    }
}
impl LockingNodePartialData for Type {}
impl LockingNodeData for Type {}
impl LockingNode for Type {
    fn node_info(&self) -> LockingNodeInfo {
        self.1.clone()
    }
}
impl LockingNodeParent for Type {}
impl LockingNodeChild for Type {}
impl LockingNodeParentChild for Type {}