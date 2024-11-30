use crate::{AbsoluteLockingPath, LockingHierarchy, LockingNodeData};

pub struct Command;
impl LockingNodeData for Command {
    fn on_insert(&mut self, hierarchy: &mut LockingHierarchy) {
        let core_path = AbsoluteLockingPath::new_from_literal("core");
        let core_mutex = hierarchy.get_node_raw(core_path.clone()).unwrap();
        
        
    }

    fn on_remove(&mut self, hierarchy: &mut LockingHierarchy) {
        
    }
}