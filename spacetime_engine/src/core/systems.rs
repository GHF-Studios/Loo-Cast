use bevy::prelude::*;
use crate::core::components::Serialized;
use super::hooks::*;
use super::structs::*;
use super::singletons::*;
use super::traits::LockingPath;
use super::wrappers::MainTypeRegistry;
use super::wrappers::Type;
use super::wrappers::TypeData;

pub(in super) fn startup(world: &mut World) {
    world
        .register_component_hooks::<Serialized>()
        .on_add(on_add_serialized)
        .on_remove(on_remove_serialized);

    let mut locking_hierarchy = LOCKING_HIERARCHY.lock().unwrap();
    
    let operations_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<MainTypeRegistry, Type, TypeData>>("operation")).unwrap();
    let operations_type_node = LockingBranchNode::<MainTypeRegistry, Type, TypeData>::new(Type::new::<Operation>());
    
    let commands_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<MainTypeRegistry, Type, TypeData>>("command")).unwrap();
    let commands_type_node = LockingBranchNode::<MainTypeRegistry, Type, TypeData>::new(Type::new::<Command>());

    let entity_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<MainTypeRegistry, Type, TypeData>>("entity")).unwrap();
    let entity_type_node = LockingBranchNode::<MainTypeRegistry, Type, TypeData>::new(Type::new::<Entity>());

    let chunk_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<MainTypeRegistry, Type, TypeData>>("chunk")).unwrap();
    let chunk_type_node = LockingBranchNode::<MainTypeRegistry, Type, TypeData>::new(Type::new::<Chunk>());

    let chunk_actor_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<MainTypeRegistry, Type, TypeData>>("chunk_actor")).unwrap();
    let chunk_actor_type_node = LockingBranchNode::<MainTypeRegistry, Type, TypeData>::new(Type::new::<ChunkActor>());

    let chunk_loader_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<MainTypeRegistry, Type, TypeData>>("chunk_loader")).unwrap();
    let chunk_loader_type_node = LockingBranchNode::<MainTypeRegistry, Type, TypeData>::new(Type::new::<ChunkLoader>());

    let camera_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<MainTypeRegistry, Type, TypeData>>("camera")).unwrap();
    let camera_type_node = LockingBranchNode::<MainTypeRegistry, Type, TypeData>::new(Type::new::<Camera>());

    let camera_2d_bundle_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<MainTypeRegistry, Type, TypeData>>("camera_2d_bundle")).unwrap();
    let camera_2d_bundle_type_node = LockingBranchNode::<MainTypeRegistry, Type, TypeData>::new(Type::new::<Camera2DBundle>());

    let player_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<MainTypeRegistry, Type, TypeData>>("player")).unwrap();
    let player_type_node = LockingBranchNode::<MainTypeRegistry, Type, TypeData>::new(Type::new::<Player>());

    let sprite_bundle_type_path = AbsoluteLockingPath::new().push(LockingPathSegment::new_static::<LockingBranchNode<MainTypeRegistry, Type, TypeData>>("sprite_bundle")).unwrap();
    let sprite_bundle_type_node = LockingBranchNode::<MainTypeRegistry, Type, TypeData>::new(Type::new::<SpriteBundle>());

    locking_hierarchy.insert(entity_type_path, Box::new(entity_type_node)).unwrap();
}