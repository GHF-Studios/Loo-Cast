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
    fn pre_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        todo!()
    }

    fn startup(&mut self, hierarchy: &mut LockingHierarchy) {
        todo!()
    }

    fn post_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        todo!()
    }
}