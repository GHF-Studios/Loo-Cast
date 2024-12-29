use bevy::prelude::*;
use crate::core::singletons::MAIN_TYPE_REGISTRY;
use super::{components::*, operations::*, wrappers::*};

pub(in super) fn startup(world: &mut World) {
    let mut main_type_registry = MAIN_TYPE_REGISTRY.lock().unwrap();

    main_type_registry.register::<ChunkLoader>();
    main_type_registry.manage::<ChunkLoader>();

    main_type_registry.set_data::<ChunkLoader, _>(ChunkLoaderInstanceRegistry::new());
    main_type_registry.set_data::<ChunkLoader, _>(ChunkLoaderOperationTypeRegistry::new());

    let chunk_loader_operation_type_registry: &mut ChunkLoaderOperationTypeRegistry = main_type_registry.get_data_mut::<ChunkLoader, _>().unwrap();

    chunk_loader_operation_type_registry.register::<UpgradeToChunkLoader>();
    chunk_loader_operation_type_registry.manage::<UpgradeToChunkLoader>();

    chunk_loader_operation_type_registry.register::<DowngradeFromChunkLoader>();
    chunk_loader_operation_type_registry.manage::<DowngradeFromChunkLoader>();

    world
        .register_component_hooks::<ChunkLoader>()
        .on_add(super::hooks::on_add_chunk_loader)
        .on_remove(super::hooks::on_remove_chunk_loader);
}
