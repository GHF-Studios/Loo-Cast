use crate::{LockingHierarchy, LockingNodeData};
use super::traits::*;

pub struct Operation;
impl LockingNodeData for Operation {
    fn pre_startup(&mut self, _hierarchy: &mut LockingHierarchy) {
        
    }

    fn startup(&mut self, _hierarchy: &mut LockingHierarchy) {
        
    }

    fn post_startup(&mut self, _hierarchy: &mut LockingHierarchy) {
        
    }

    fn pre_update(&mut self, hierarchy: &mut LockingHierarchy) {
        
    }

    fn update(&mut self, _hierarchy: &mut LockingHierarchy) {
        
    }

    fn post_update(&mut self, hierarchy: &mut LockingHierarchy) {
        
    }
}

pub struct OperationQueue {
    queue: Vec<Box<dyn DynOperation>>,
}
impl OperationQueue {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }

    pub fn add_operation(&mut self, operation: Box<dyn DynOperation>) {
        self.queue.push(operation);
    }

    pub fn remove_operations(&mut self) -> Vec<Box<dyn DynOperation>> {
        self.queue.drain(..).collect()
    }
}