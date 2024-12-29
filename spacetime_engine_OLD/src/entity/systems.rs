use super::components::*;
use super::wrappers::*;
use super::hooks::*;
use super::operations::*;
use crate::core::singletons::*;
use bevy::prelude::*;

pub(in super) fn startup(world: &mut World) {
    let mut main_type_registry = MAIN_TYPE_REGISTRY.lock().unwrap();

    main_type_registry.register::<Entity>();
    main_type_registry.manage::<Entity>();

    main_type_registry.set_data::<Entity, _>(EntityInstanceRegistry::new());
    main_type_registry.set_data::<Entity, _>(EntityOperationTypeRegistry::new());

    let entity_operation_type_registry: &mut EntityOperationTypeRegistry = main_type_registry.get_data_mut::<Entity, _>().unwrap();

    entity_operation_type_registry.register::<CreateEntity>();
    entity_operation_type_registry.manage::<CreateEntity>();

    entity_operation_type_registry.register::<DestroyEntity>();
    entity_operation_type_registry.manage::<DestroyEntity>();

    world
        .register_component_hooks::<SpacetimeEntity>()
        .on_add(on_add_entity)
        .on_remove(on_remove_entity);
}