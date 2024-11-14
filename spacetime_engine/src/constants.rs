use std::any::TypeId;
use structs::{LockingType, TypeBinding};
use wrappers::{RootType, RootTypeData, RootTypeRegistry};

use crate::*;
use super::command::structs::Command;
use super::operation::structs::Operation;

pub const RESERVED_STRING_IDS: [&str; 1] = ["root"];
pub const RESERVED_NUMERIC_IDS: [i32; 1] = [0];
pub const ROOT_TYPE_BINDING: TypeBinding = TypeBinding {
    type_name: "root",
    type_id: TypeId::of::<RootTypeRegistry>(),
    type_pre_setup: |hierarchy| {
        let root_path = AbsoluteLockingPath::new();
        let root_mutex = hierarchy.get_node_raw(root_path).unwrap();

        let core_path_segment = LockingPathSegment::new_string("core");
        let core_path = AbsoluteLockingPath::new().push(core_path_segment).unwrap();
        let core = LockingType::new::<Core>("core");
        hierarchy.insert_branch::<RootTypeRegistry, RootType<Core>, RootTypeData>(root_path, root_mutex, core_path_segment, core).unwrap();
        hierarchy.pre_startup::<RootType<Core>>(core_path).unwrap();

        let operation_path_segment = LockingPathSegment::new_string("operation");
        let operation_path = AbsoluteLockingPath::new().push(operation_path_segment).unwrap();
        let operation = LockingType::new::<Operation>("operation");
        hierarchy.insert_branch::<RootTypeRegistry, RootType, RootTypeData>(root_path, root_mutex, operation_path_segment, operation).unwrap();
        hierarchy.pre_startup::<RootType>(operation_path).unwrap();
        
        let command_path_segment = LockingPathSegment::new_string("command");
        let command_path = AbsoluteLockingPath::new().push(command_path_segment).unwrap();
        let command = LockingType::new::<Command>("command");
        hierarchy.insert_branch::<RootTypeRegistry, RootType, RootTypeData>(root_path, root_mutex, command_path_segment, command).unwrap();
        hierarchy.pre_startup::<RootType>(command_path).unwrap();

        let entity_path_segment = LockingPathSegment::new_string("entity");
        let entity_path = AbsoluteLockingPath::new().push(entity_path_segment).unwrap();
        let entity = LockingType::new::<Entity>("entity");
        hierarchy.insert_branch::<RootTypeRegistry, RootType, RootTypeData>(root_path, root_mutex, entity_path_segment, entity).unwrap();
        hierarchy.pre_startup::<RootType>(entity_path).unwrap();

        let chunk_path_segment = LockingPathSegment::new_string("chunk");
        let chunk_path = AbsoluteLockingPath::new().push(chunk_path_segment).unwrap();
        let chunk = LockingType::new::<Chunk>("chunk");
        hierarchy.insert_branch::<RootTypeRegistry, RootType, RootTypeData>(root_path, root_mutex, chunk_path_segment, chunk).unwrap();
        hierarchy.pre_startup::<RootType>(chunk_path).unwrap();

        let chunk_actor_path_segment = LockingPathSegment::new_string("chunk_actor");
        let chunk_actor_path = AbsoluteLockingPath::new().push(chunk_actor_path_segment).unwrap();
        let chunk_actor = LockingType::new::<ChunkActor>("chunk_actor");
        hierarchy.insert_branch::<RootTypeRegistry, RootType, RootTypeData>(root_path, root_mutex, chunk_actor_path_segment, chunk_actor).unwrap();
        hierarchy.pre_startup::<RootType>(chunk_actor_path).unwrap();

        let chunk_loader_path_segment = LockingPathSegment::new_string("chunk_loader");
        let chunk_loader_path = AbsoluteLockingPath::new().push(chunk_loader_path_segment).unwrap();
        let chunk_loader = LockingType::new::<ChunkLoader>("chunk_loader");
        hierarchy.insert_branch::<RootTypeRegistry, RootType, RootTypeData>(root_path, root_mutex, chunk_loader_path_segment, chunk_loader).unwrap();
        hierarchy.pre_startup::<RootType>(chunk_loader_path).unwrap();

        let camera_path_segment = LockingPathSegment::new_string("camera");
        let camera_path = AbsoluteLockingPath::new().push(camera_path_segment).unwrap();
        let camera = LockingType::new::<Camera>("camera");
        hierarchy.insert_branch::<RootTypeRegistry, RootType, RootTypeData>(root_path, root_mutex, camera_path_segment, camera).unwrap();
        hierarchy.pre_startup::<RootType>(camera_path).unwrap();

        let camera_2d_bundle_path_segment = LockingPathSegment::new_string("camera_2d_bundle");
        let camera_2d_bundle_path = AbsoluteLockingPath::new().push(camera_2d_bundle_path_segment).unwrap();
        let camera_2d_bundle = LockingType::new::<Camera2DBundle>("camera_2d_bundle");
        hierarchy.insert_branch::<RootTypeRegistry, RootType, RootTypeData>(root_path, root_mutex, camera_2d_bundle_path_segment, camera_2d_bundle).unwrap();
        hierarchy.pre_startup::<RootType>(camera_2d_bundle_path).unwrap();

        let player_path_segment = LockingPathSegment::new_string("player");
        let player_path = AbsoluteLockingPath::new().push(player_path_segment).unwrap();
        let player = LockingType::new::<Player>("player");
        hierarchy.insert_branch::<RootTypeRegistry, RootType, RootTypeData>(root_path, root_mutex, player_path_segment, player).unwrap();
        hierarchy.pre_startup::<RootType>(player_path).unwrap();

        let sprite_bundle_path_segment = LockingPathSegment::new_string("sprite_bundle");
        let sprite_bundle_path = AbsoluteLockingPath::new().push(sprite_bundle_path_segment).unwrap();
        let sprite_bundle = LockingType::new::<SpriteBundle>("sprite_bundle");
        hierarchy.insert_branch::<RootTypeRegistry, RootType, RootTypeData>(root_path, root_mutex, sprite_bundle_path_segment, sprite_bundle).unwrap();
        hierarchy.pre_startup::<RootType>(sprite_bundle_path).unwrap();
    },
    type_setup: |hierarchy| {
        let core_path = AbsoluteLockingPath::new_from_literal("core");
        hierarchy.startup::<RootType>(core_path).unwrap();

        let operation_path = AbsoluteLockingPath::new_from_literal("operation");
        hierarchy.startup::<RootType>(operation_path).unwrap();

        let command_path = AbsoluteLockingPath::new_from_literal("command");
        hierarchy.startup::<RootType>(command_path).unwrap();

        let entity_path = AbsoluteLockingPath::new_from_literal("entity");
        hierarchy.startup::<RootType>(entity_path).unwrap();

        let chunk_path = AbsoluteLockingPath::new_from_literal("chunk");
        hierarchy.startup::<RootType>(chunk_path).unwrap();

        let chunk_actor_path = AbsoluteLockingPath::new_from_literal("chunk_actor");
        hierarchy.startup::<RootType>(chunk_actor_path).unwrap();

        let chunk_loader_path = AbsoluteLockingPath::new_from_literal("chunk_loader");
        hierarchy.startup::<RootType>(chunk_loader_path).unwrap();

        let camera_path = AbsoluteLockingPath::new_from_literal("camera");
        hierarchy.startup::<RootType>(camera_path).unwrap();

        let camera_2d_bundle_path = AbsoluteLockingPath::new_from_literal("camera_2d_bundle");
        hierarchy.startup::<RootType>(camera_2d_bundle_path).unwrap();

        let player_path = AbsoluteLockingPath::new_from_literal("player");
        hierarchy.startup::<RootType>(player_path).unwrap();

        let sprite_bundle_path = AbsoluteLockingPath::new_from_literal("sprite_bundle");
        hierarchy.startup::<RootType>(sprite_bundle_path).unwrap();
    },
    type_post_setup: |hierarchy| {
        let core_path = AbsoluteLockingPath::new_from_literal("core");
        hierarchy.post_startup::<RootType>(core_path).unwrap();

        let operation_path = AbsoluteLockingPath::new_from_literal("operation");
        hierarchy.post_startup::<RootType>(operation_path).unwrap();

        let command_path = AbsoluteLockingPath::new_from_literal("command");
        hierarchy.post_startup::<RootType>(command_path).unwrap();

        let entity_path = AbsoluteLockingPath::new_from_literal("entity");
        hierarchy.post_startup::<RootType>(entity_path).unwrap();

        let chunk_path = AbsoluteLockingPath::new_from_literal("chunk");
        hierarchy.post_startup::<RootType>(chunk_path).unwrap();

        let chunk_actor_path = AbsoluteLockingPath::new_from_literal("chunk_actor");
        hierarchy.post_startup::<RootType>(chunk_actor_path).unwrap();

        let chunk_loader_path = AbsoluteLockingPath::new_from_literal("chunk_loader");
        hierarchy.post_startup::<RootType>(chunk_loader_path).unwrap();

        let camera_path = AbsoluteLockingPath::new_from_literal("camera");
        hierarchy.post_startup::<RootType>(camera_path).unwrap();

        let camera_2d_bundle_path = AbsoluteLockingPath::new_from_literal("camera_2d_bundle");
        hierarchy.post_startup::<RootType>(camera_2d_bundle_path).unwrap();

        let player_path = AbsoluteLockingPath::new_from_literal("player");
        hierarchy.post_startup::<RootType>(player_path).unwrap();

        let sprite_bundle_path = AbsoluteLockingPath::new_from_literal("sprite_bundle");
        hierarchy.post_startup::<RootType>(sprite_bundle_path).unwrap();
    },
};