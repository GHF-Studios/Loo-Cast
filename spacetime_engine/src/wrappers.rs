use bevy::prelude::*;
use crate::{structs::*, LockingHierarchy, LockingNodeData};

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
impl<T: 'static + Send + Sync> LockingNodeData for RootType<T> {
    fn pre_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        (self.0.type_pre_setup)(hierarchy)
    }

    fn startup(&mut self, hierarchy: &mut LockingHierarchy) {
        (self.0.type_setup)(hierarchy)
    }

    fn post_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        (self.0.type_post_setup)(hierarchy)
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
    pub fn new(main_type_binding: TypeBinding) -> Self {
        Self(LockingTypeRegistry::new(), main_type_binding)
    }
}
impl LockingNodeData for RootTypeRegistry {
    fn pre_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        (self.1.type_pre_setup)(hierarchy)
    }

    fn startup(&mut self, hierarchy: &mut LockingHierarchy) {
        (self.1.type_setup)(hierarchy)
    }

    fn post_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        (self.1.type_post_setup)(hierarchy)
    }
}

#[derive(Deref, DerefMut)]
pub struct RootTypeDataRegistry(LockingTypeDataRegistry);
impl RootTypeDataRegistry {
    pub fn new() -> Self {
        Self(LockingTypeDataRegistry::new())
    }
}