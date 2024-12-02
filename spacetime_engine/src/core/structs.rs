use crate::*;
use crate::singletons::*;
use crate::core::wrappers::CoreCommandTypes;
use std::collections::HashMap;
use std::any::*;
use std::sync::{Arc, Mutex, MutexGuard};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use bevy::prelude::*;
use structs::{NumericID, StringID, TypeRegistry};
use super::enums::*;
use super::errors::*;
use super::traits::*;

pub struct Core;
impl LockingNodeData for Core {
    fn on_insert(&mut self, hierarchy: &mut LockingHierarchy) {
        let core_path = AbsoluteLockingPath::new_from_literal("core");
        let core_mutex = hierarchy.get_node_raw(core_path.clone()).unwrap();
    
        let command_types_path_segment = LockingPathSegment::new_string("command_types");
        hierarchy.insert(core_path.clone(), core_mutex.clone(), command_types_path_segment, CoreCommandTypes::new()).unwrap();

        dispatch_cmds!(async, batch, [
            ("core.command_types.spawn_main_camera"),
            ("core.command_types.spawn_start_chunks", 2),
            ("core.command_types.spawn_start_chunk_actors", 2),
        ]);
    }

    fn on_remove(&mut self, hierarchy: &mut LockingHierarchy) {
        dispatch_cmds!(async, sequence, [
            ("core.command_types.despawn_main_camera"),
            ("core.command_types.despawn_start_chunks", 2),
            ("core.command_types.despawn_start_chunk_actors", 2),
        ]);

        let core_path = AbsoluteLockingPath::new_from_literal("core");
        let core_mutex = hierarchy.get_node_raw(core_path.clone()).unwrap();

        let command_types_path_segment = LockingPathSegment::new_string("command_types");
        let command_types_path = core_path.clone().push(command_types_path_segment).unwrap();
        hierarchy.remove(command_types_path);
    }
}