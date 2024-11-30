use crate::{LockingHierarchy, LockingNodeData};
use super::traits::*;

pub struct Operation;
impl LockingNodeData for Operation {
    fn on_pre_insert(&mut self, _hierarchy: &mut LockingHierarchy) {
        
    }

    fn on_insert(&mut self, _hierarchy: &mut LockingHierarchy) {
        
    }

    fn on_post_insert(&mut self, _hierarchy: &mut LockingHierarchy) {
        
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