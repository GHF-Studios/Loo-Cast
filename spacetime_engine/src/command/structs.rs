use crate::{LockingHierarchy, LockingNodeData};

pub struct Command;
impl LockingNodeData for Command {
    fn pre_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        
    }

    fn startup(&mut self, hierarchy: &mut LockingHierarchy) {
        
    }

    fn post_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        
    }

    fn update(&mut self, hierarchy: &mut LockingHierarchy) {
        
    }
}