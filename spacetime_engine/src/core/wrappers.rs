use crate::{core::{structs::*, traits::*}, TypeRegistry};
use bevy::prelude::*;

#[derive(Deref, DerefMut)]
pub struct CoreCommandTypeRegistry(TypeRegistry);
impl CoreCommandTypeRegistry {
    pub fn new() -> Self {
        Self(TypeRegistry::new())
    }
}
impl LockingNodeData for CoreCommandTypeRegistry {
    fn pre_startup(&mut self, hierarchy: &mut LockingHierarchy) {

    }

    fn startup(&mut self, hierarchy: &mut LockingHierarchy) {

    }

    fn post_startup(&mut self, hierarchy: &mut LockingHierarchy) {

    }
}