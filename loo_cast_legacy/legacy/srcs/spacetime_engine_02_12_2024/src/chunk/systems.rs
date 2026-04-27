use bevy::prelude::*;
use crate::core::singletons::MAIN_TYPE_REGISTRY;
use super::{components::Chunk, operations::*, wrappers::*};

pub(in super) fn startup(world: &mut World) {
    let mut main_type_registry = MAIN_TYPE_REGISTRY.lock().unwrap();

    main_type_registry.register::<Chunk>();
    main_type_registry.manage::<Chunk>();

    main_type_registry.set_data::<Chunk, _>(ChunkInstanceRegistry::new());
    main_type_registry.set_data::<Chunk, _>(ChunkOperationTypeRegistry::new());

    let chunk_operation_type_registry: &mut ChunkOperationTypeRegistry = main_type_registry.get_data_mut::<Chunk, _>().unwrap();

    chunk_operation_type_registry.register::<UpgradeToChunk>();
    chunk_operation_type_registry.manage::<UpgradeToChunk>();

    chunk_operation_type_registry.register::<DowngradeFromChunk>();
    chunk_operation_type_registry.manage::<DowngradeFromChunk>();

    chunk_operation_type_registry.register::<LoadChunk>();
    chunk_operation_type_registry.manage::<LoadChunk>();

    chunk_operation_type_registry.register::<UnloadChunk>();
    chunk_operation_type_registry.manage::<UnloadChunk>();

    chunk_operation_type_registry.register::<SaveChunk>();
    chunk_operation_type_registry.manage::<SaveChunk>();

    chunk_operation_type_registry.register::<UnsaveChunk>();
    chunk_operation_type_registry.manage::<UnsaveChunk>();

    world
        .register_component_hooks::<Chunk>()
        .on_add(super::hooks::on_add_chunk)
        .on_remove(super::hooks::on_remove_chunk);
}
