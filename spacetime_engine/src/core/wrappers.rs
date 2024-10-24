use std::any::TypeId;
use crate::core::structs::*;
use bevy::prelude::*;
use super::traits::*;

#[derive(Deref, DerefMut)]
pub struct MainTypeRegistry(TypeRegistry);
impl MainTypeRegistry {
    pub fn new() -> Self {
        Self(TypeRegistry::new())
    }
}
impl LockingNodePartialData for MainTypeRegistry {}
impl LockingNodeData for MainTypeRegistry {}

#[derive(Deref, DerefMut)]
pub struct TypeDataRegistry(SingletonRegistry);
impl TypeDataRegistry {
    pub fn new() -> Self {
        Self(SingletonRegistry::new())
    }
}
impl LockingNodePartialData for TypeDataRegistry {}
impl LockingNodeData for TypeDataRegistry {}

#[derive(Deref, DerefMut)]
pub struct Type((TypeId, TypeDataRegistry));
impl Type {
    pub fn new<T: 'static + Send + Sync>() -> Self {
        Self((TypeId::of::<T>(), TypeDataRegistry::new()))
    }

    pub fn type_id(&self) -> TypeId {
        self.0.0
    }

    pub fn type_data_registry(&self) -> &TypeDataRegistry {
        &self.0.1
    }

    pub fn type_data_registry_mut(&mut self) -> &mut TypeDataRegistry {
        &mut self.0.1
    }
}
impl LockingNodePartialData for Type {}
impl LockingNodeData for Type {}
