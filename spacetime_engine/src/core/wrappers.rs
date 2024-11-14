use crate::{core::{structs::*, traits::*}, structs::*};
use bevy::prelude::*;

#[derive(Deref, DerefMut)]
pub struct CoreCommandTypeRegistry(LockingTypeRegistry);
impl CoreCommandTypeRegistry {
    pub fn new() -> Self {
        Self(LockingTypeRegistry::new())
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