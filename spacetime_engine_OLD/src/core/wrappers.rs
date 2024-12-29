use crate::{core::{structs::*, traits::*}, structs::*};
use bevy::prelude::*;

#[derive(Deref, DerefMut)]
pub struct CoreCommandTypes(LockingTypeRegistry);
impl CoreCommandTypes {
    pub fn new() -> Self {
        Self(LockingTypeRegistry::new())
    }
}
impl LockingNodeData for CoreCommandTypes {
    fn on_insert(&mut self, hierarchy: &mut LockingHierarchy) {
        let command_types_path = AbsoluteLockingPath::new_from_literal("core.command_types");
        let command_types_mutex = hierarchy.get_node_raw(command_types_path.clone()).unwrap();

        let spawn_main_camera_path_segment = LockingPathSegment::new_string("spawn_main_camera");
        let spawn_main_camera_path = command_types_path.clone().push(spawn_main_camera_path_segment).unwrap();
        hierarchy.insert(command_types_path.clone(), command_types_mutex.clone(), spawn_main_camera_path_segment, CoreCommandTypes::new()).unwrap();

        let spawn_start_chunks_path_segment = LockingPathSegment::new_string("spawn_start_chunks");
        let spawn_start_chunks_path = command_types_path.clone().push(spawn_start_chunks_path_segment).unwrap();
        hierarchy.insert(command_types_path.clone(), command_types_mutex.clone(), spawn_start_chunks_path_segment, CoreCommandTypes::new()).unwrap();

        let spawn_start_chunk_actors_path_segment = LockingPathSegment::new_string("spawn_start_chunk_actors");
        let spawn_start_chunk_actors_path = command_types_path.clone().push(spawn_start_chunk_actors_path_segment).unwrap();
        hierarchy.insert(command_types_path.clone(), command_types_mutex.clone(), spawn_start_chunk_actors_path_segment, CoreCommandTypes::new()).unwrap();
    }

    fn on_remove(&mut self, hierarchy: &mut LockingHierarchy) {
        let command_types_path = AbsoluteLockingPath::new_from_literal("core.command_types");
        let command_types_mutex = hierarchy.get_node_raw(command_types_path.clone()).unwrap();

        let spawn_main_camera_path_segment = LockingPathSegment::new_string("spawn_main_camera");
        let spawn_main_camera_path = command_types_path.clone().push(spawn_main_camera_path_segment).unwrap();
        hierarchy.remove(command_types_path.clone()).unwrap();

        let spawn_start_chunks_path_segment = LockingPathSegment::new_string("spawn_start_chunks");
        let spawn_start_chunks_path = command_types_path.clone().push(spawn_start_chunks_path_segment).unwrap();
        hierarchy.remove(command_types_path.clone()).unwrap();

        let spawn_start_chunk_actors_path_segment = LockingPathSegment::new_string("spawn_start_chunk_actors");
        let spawn_start_chunk_actors_path = command_types_path.clone().push(spawn_start_chunk_actors_path_segment).unwrap();
        hierarchy.remove(command_types_path.clone()).unwrap();
    }
}