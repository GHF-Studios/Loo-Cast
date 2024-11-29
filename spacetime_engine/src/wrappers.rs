use bevy::prelude::*;
use crate::{structs::*, LockingNodeData};

#[derive(Deref, DerefMut)]
pub struct RootType<T: 'static + Send + Sync>(Type<T>);
impl<T: 'static + Send + Sync> RootType<T> {
    pub fn new(type_binding: TypeBinding) -> Self {
        Self(Type::<T>::new(type_binding))
    }

    pub fn new_unchecked(type_binding: TypeBinding) -> Self {
        Self(Type::<T>::new_unchecked(type_binding))
    }
}

#[derive(Deref, DerefMut)]
pub struct RootTypeData<T: 'static + Send + Sync + LockingNodeData>(TypeData<T>);
impl<T: 'static + Send + Sync + LockingNodeData> RootTypeData<T> {
    pub fn new(data_type_binding: TypeBinding, data: T) -> Self {
        Self(TypeData::<T>::new(data_type_binding, data))
    }
}

#[derive(Deref, DerefMut)]
pub struct RootTypeRegistry(#[deref]TypeRegistry, TypeBinding);
impl RootTypeRegistry {
    pub fn new(root_type_binding: TypeBinding) -> Self {
        Self(LockingTypeRegistry::new(), root_type_binding)
    }
}

#[derive(Deref, DerefMut)]
pub struct RootTypeDataRegistry(LockingTypeDataRegistry);
impl RootTypeDataRegistry {
    pub fn new() -> Self {
        Self(LockingTypeDataRegistry::new())
    }
}