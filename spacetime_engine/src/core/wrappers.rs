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
        let command_types_path = AbsoluteLockingPath::new_from_literal("core.command_types");
        let command_types_mutex = hierarchy.get_node_raw(command_types_path.clone()).unwrap();
    
        let spawn_main_camera_path_segment = LockingPathSegment::new_string("spawn_main_camera");
        let spawn_main_camera_path = command_types_path.clone().push(spawn_main_camera_path_segment).unwrap();
        hierarchy.insert_branch(command_types_path.clone(), command_types_mutex.clone(), spawn_main_camera_path_segment, CoreCommandTypeRegistry::new()).unwrap();
        hierarchy.pre_startup(command_types_path).unwrap();

        let spawn_start_chunks_path_segment = LockingPathSegment::new_string("spawn_start_chunks");
        let spawn_start_chunks_path = command_types_path.clone().push(spawn_start_chunks_path_segment).unwrap();
        hierarchy.insert_branch(command_types_path.clone(), command_types_mutex.clone(), spawn_start_chunks_path_segment, CoreCommandTypeRegistry::new()).unwrap();
        hierarchy.pre_startup(command_types_path).unwrap();

        let spawn_start_chunk_actors_path_segment = LockingPathSegment::new_string("spawn_start_chunk_actors");
        let spawn_start_chunk_actors_path = command_types_path.clone().push(spawn_start_chunk_actors_path_segment).unwrap();
        hierarchy.insert_branch(command_types_path.clone(), command_types_mutex.clone(), spawn_start_chunk_actors_path_segment, CoreCommandTypeRegistry::new()).unwrap();
        hierarchy.pre_startup(command_types_path).unwrap();
    }

    fn startup(&mut self, hierarchy: &mut LockingHierarchy) {

    }

    fn post_startup(&mut self, hierarchy: &mut LockingHierarchy) {

    }

    fn pre_update(&mut self, hierarchy: &mut LockingHierarchy) {
        
    }

    fn update(&mut self, hierarchy: &mut LockingHierarchy) {
        
    }

    fn post_update(&mut self, hierarchy: &mut LockingHierarchy) {
        
    }
}