use std::any::Any;
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
pub struct TypeDataRegistry(SingletonRegistry<TypeData>);
impl TypeDataRegistry {
    pub fn new() -> Self {
        Self(SingletonRegistry::new())
    }
}
impl LockingNodePartialData for TypeDataRegistry {}
impl LockingNodeData for TypeDataRegistry {}

#[derive(Deref, DerefMut)]
pub struct Type(TypeDataRegistry);
impl Type {
    pub fn new<T: 'static + Send + Sync>() -> Self {
        Self(TypeDataRegistry::new())
    }

    pub fn type_data_registry(&self) -> &TypeDataRegistry {
        &self.0
    }

    pub fn type_data_registry_mut(&mut self) -> &mut TypeDataRegistry {
        &mut self.0
    }
}
impl LockingNodePartialData for Type {}
impl LockingNodeData for Type {}

#[derive(Deref, DerefMut)]
pub struct TypeData(Box<dyn Any + Send + Sync>);
impl TypeData {
    pub fn new<T: 'static + Send + Sync>(data: T) -> Self {
        Self(Box::new(data))
    }

    pub fn data<T: 'static + Send + Sync>(&self) -> Option<&T> {
        self.0.downcast_ref::<T>()
    }

    pub fn data_mut<T: 'static + Send + Sync>(&mut self) -> Option<&mut T> {
        self.0.downcast_mut::<T>()
    }
}
impl LockingNodePartialData for TypeData {}
impl LockingNodeData for TypeData {}
impl Singleton for TypeData {}