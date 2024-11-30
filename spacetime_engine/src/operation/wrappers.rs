use crate::{core::{structs::*, traits::*}, structs::TypeRegistry};
use bevy::prelude::*;

#[derive(Deref, DerefMut)]
pub struct OperationTypeRegistry(TypeRegistry);
impl OperationTypeRegistry {
    pub fn new() -> Self {
        Self(TypeRegistry::new())
    }
}
impl LockingNodeData for OperationTypeRegistry {
    fn on_pre_insert(&mut self, hierarchy: &mut LockingHierarchy) {
        todo!()
    }

    fn on_insert(&mut self, hierarchy: &mut LockingHierarchy) {
        todo!()
    }

    fn on_post_insert(&mut self, hierarchy: &mut LockingHierarchy) {
        todo!()
    }
}