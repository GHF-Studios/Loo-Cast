use crate::{structs::{LockingType, LockingTypeData}, AbsoluteLockingPath, LockingHierarchy, LockingNodeData, LockingPathSegment};
use super::{constants::COMMAND_TYPE_BINDING, traits::*};

#[derive(Deref, DerefMut)]
pub struct CommandTypeRegistry(TypeRegistry);
impl CommandTypeRegistry {
    pub fn new() -> Self {
        Self(TypeRegistry::new())
    }
}
impl LockingNodeData for CommandTypeRegistry {
    fn pre_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        (ROOT_TYPE_BINDING.type_pre_setup)(hierarchy)
    }

    fn startup(&mut self, hierarchy: &mut LockingHierarchy) {
        (ROOT_TYPE_BINDING.type_pre_setup)(hierarchy)
    }

    fn post_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        (ROOT_TYPE_BINDING.type_pre_setup)(hierarchy)
    }
}

#[derive(Deref, DerefMut)]
pub struct CommandType(LockingType);
impl CommandType {
    pub fn new<T: 'static + Send + Sync + LockingNodeData + Command>(type_name: &str) -> Self {
        Self(LockingType::new::<T>(type_name))
    }
}
impl LockingNodeData for CommandType {
    fn pre_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        let core_path = AbsoluteLockingPath::new();
        let core_mutex = hierarchy.get_node_raw(core_path).unwrap();
        let type_path_segment = LockingPathSegment::new_string(self.0.1);
        let type_path = AbsoluteLockingPath::new().push(type_path_segment);
        let type_ = CommandType::new_unchecked(self.0, self.1);
        hierarchy.insert_leaf::<CommandType, CommandTypeData>(core_path, core_mutex, type_path_segment, type_).unwrap();
        let type_binding = COMMAND_TYPE_BINDING;
        (type_binding.type_pre_setup)(hierarchy);
    }

    fn startup(&mut self, hierarchy: &mut LockingHierarchy) {
        let core_path = AbsoluteLockingPath::new();
        let type_path_segment = LockingPathSegment::new_string(self.1);
        let type_path = AbsoluteLockingPath::new().push(type_path_segment);
        let type_binding = COMMAND_TYPE_BINDING;
        (type_binding.type_setup)(hierarchy);
    }

    fn post_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        let core_path = AbsoluteLockingPath::new();
        let type_path_segment = LockingPathSegment::new_string(self.1);
        let type_path = AbsoluteLockingPath::new().push(type_path_segment);
        let type_binding = COMMAND_TYPE_BINDING;
        (type_binding.type_post_setup)(hierarchy);
    }
}