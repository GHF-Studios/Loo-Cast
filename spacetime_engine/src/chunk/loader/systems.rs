use std::collections::HashSet;
use std::panic;

use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk::events::*;
use crate::chunk::id::structs::ChunkID;
use crate::chunk::resources::*;
use crate::chunk::loader::components::*;
use crate::chunk::loader::events::*;
use crate::chunk::functions as chunk_functions;
use super::functions as chunk_loader_functions;
use super::ChunkLoaderRequestRegistry;
use crate::entity::resources::EntityRegistry;
use super::resources::ChunkLoaderRegistry;	

#[allow(clippy::too_many_arguments)]
pub(in crate) fn start(
    create_chunk_event_writer: EventWriter<CreateChunkEntity>,
    load_chunk_event_writer: EventWriter<LoadChunkEntity>,
    mut started_chunk_loader_event_writer: EventWriter<StartedChunkLoader>,
    mut chunk_loader_query: Query<(&Transform, &mut ChunkLoader), Added<ChunkLoader>>,
    mut chunk_registry: ResMut<ChunkRegistry>,
    mut chunk_request_registry: ResMut<ChunkRequestRegistry>,
    mut chunk_loader_registry: ResMut<ChunkLoaderRegistry>,
    mut chunk_loader_request_registry: ResMut<ChunkLoaderRequestRegistry>,
) {
    let (chunk_loader_transform, mut chunk_loader) = match chunk_loader_query.get_single_mut() {
        Ok((chunk_loader_transform, chunk_loader)) => (chunk_loader_transform, chunk_loader),
        Err(_) => {
            return;
        }
    };
    
    let chunk_loader_id = chunk_loader.id();
    let chunk_loader_load_radius = chunk_loader.load_radius();
    
    let start_chunk_ids = chunk_functions::detect_chunks(chunk_loader_transform, chunk_loader_load_radius);

    if !start_chunk_ids.is_empty() {
        error!("Start chunks: {:?}", start_chunk_ids);
    }

    let mut failed_allocate = false;
    for start_chunk_id in start_chunk_ids.clone() {
        if !chunk_registry.try_allocate_chunk(start_chunk_id) {
            error!("Failed to allocate start chunk '{:?}'!", start_chunk_id);

            failed_allocate = true;
        }
    }

    if failed_allocate {
        panic!("Failed to allocate start chunks!");
    }

    chunk_functions::start_chunks(
        create_chunk_event_writer, 
        load_chunk_event_writer, 
        &mut chunk_loader,
        &mut chunk_registry, 
        &mut chunk_request_registry,
        &start_chunk_ids,
    );

    *chunk_loader.current_chunk_ids_mut() = start_chunk_ids;

    let chunk_loader_request_id = chunk_loader_request_registry.get_unused_chunk_loader_request_id();

    chunk_loader_registry.start_chunk_loader(chunk_loader_id);

    started_chunk_loader_event_writer.send(StartedChunkLoader::Success {
        chunk_loader_request_id,
        chunk_loader_id
    });
}

pub(in crate) fn update(
    create_chunk_event_writer: EventWriter<CreateChunkEntity>,
    load_chunk_event_writer: EventWriter<LoadChunkEntity>,
    unload_chunk_event_writer: EventWriter<UnloadChunkEntity>,
    mut chunk_loader_query: Query<(&Transform, &mut ChunkLoader)>,
    mut chunk_registry: ResMut<ChunkRegistry>,
    mut chunk_request_registry: ResMut<ChunkRequestRegistry>,
    chunk_loader_registry: Res<ChunkLoaderRegistry>,
) {
    let (chunk_loader_transform, mut chunk_loader) = match chunk_loader_query.get_single_mut() {
        Ok((chunk_loader_transform, chunk_loader)) => (chunk_loader_transform, chunk_loader),
        Err(_) => {
            return;
        }
    };

    if !chunk_loader_registry.is_chunk_loader_started(chunk_loader.id()) {
        return;
    }

    let chunk_loader_load_radius = chunk_loader.load_radius();

    let detected_chunk_ids = chunk_functions::detect_chunks(chunk_loader_transform, chunk_loader_load_radius);

    let (
        old_chunk_ids, 
        unchanged_chunk_ids, 
        new_chunk_ids
    ) = chunk_functions::categorize_chunks(&mut chunk_registry, &mut chunk_loader, detected_chunk_ids);

    if !old_chunk_ids.is_empty() {
        error!("Old chunks: {:?}", old_chunk_ids);
    }

    if !new_chunk_ids.is_empty() {
        error!("New chunks: {:?}", new_chunk_ids);
    }

    for chunk_id in old_chunk_ids.iter() {
        if !chunk_registry.try_allocate_chunk(*chunk_id) {
            error!("Failed to allocate old chunk '{:?}'!", chunk_id);
        }
    }

    for chunk_id in new_chunk_ids.iter() {
        if !chunk_registry.try_allocate_chunk(*chunk_id) {
            error!("Failed to allocate new chunk '{:?}'!", chunk_id);
        }
    }

    chunk_functions::update_chunks(
        create_chunk_event_writer, 
        load_chunk_event_writer, 
        unload_chunk_event_writer,
        &mut chunk_loader,
        &mut chunk_registry, 
        &mut chunk_request_registry,
        old_chunk_ids, 
        new_chunk_ids.clone(), 
    );

    *chunk_loader.current_chunk_ids_mut() = [unchanged_chunk_ids, new_chunk_ids].concat();
}

pub(super) fn handle_create_chunk_loader_entity_events(
    mut create_chunk_loader_entity_event_reader: EventReader<CreateChunkLoaderEntity>,
    mut create_chunk_loader_entity_internal_event_writer: EventWriter<CreateChunkLoaderEntityInternal>,
    mut chunk_loader_registry: ResMut<ChunkLoaderRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    let mut create_chunk_loader_entity_events = Vec::new();
    for create_chunk_loader_entity_event in create_chunk_loader_entity_event_reader.read() {
        create_chunk_loader_entity_events.push(create_chunk_loader_entity_event);
    }

    for create_chunk_loader_entity_event in create_chunk_loader_entity_events {
        let chunk_loader_request_id = create_chunk_loader_entity_event.chunk_loader_request_id;
        let chunk_loader_entity_id = entity_registry.register_entity();
        let chunk_loader_id = chunk_loader_registry.register_chunk_loader();
        let world_position = create_chunk_loader_entity_event.world_position;

        info!("Trying to create chunk loader entity '{:?}' at world position '{:?}' ...", chunk_loader_entity_id, world_position);

        create_chunk_loader_entity_internal_event_writer.send(CreateChunkLoaderEntityInternal {
            chunk_loader_request_id,
            chunk_loader_id,
            chunk_loader_entity_id,
            world_position
        });
    }
}

pub(super) fn handle_destroy_chunk_loader_entity_events(
    mut destroy_chunk_loader_entity_event_reader: EventReader<DestroyChunkLoaderEntity>,
    mut destroy_chunk_loader_entity_internal_event_writer: EventWriter<DestroyChunkLoaderEntityInternal>,
) {
    let mut destroy_chunk_loader_entity_events = Vec::new();
    for destroy_chunk_loader_entity_event in destroy_chunk_loader_entity_event_reader.read() {
        destroy_chunk_loader_entity_events.push(destroy_chunk_loader_entity_event);
    }

    for destroy_chunk_loader_entity_event in destroy_chunk_loader_entity_events {
        let chunk_loader_request_id = destroy_chunk_loader_entity_event.chunk_loader_request_id;
        let chunk_loader_id = destroy_chunk_loader_entity_event.chunk_loader_id;

        info!("Trying to destroy chunk loader entity '{:?}' ...", chunk_loader_id);

        destroy_chunk_loader_entity_internal_event_writer.send(DestroyChunkLoaderEntityInternal {
            chunk_loader_request_id,
            chunk_loader_id,
        });
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn handle_upgrade_to_chunk_loader_entity_events(
    mut upgrade_to_chunk_loader_entity_event_reader: EventReader<UpgradeToChunkLoaderEntity>,
    mut upgrade_to_chunk_loader_entity_internal_event_writer: EventWriter<UpgradeToChunkLoaderEntityInternal>,
    mut chunk_loader_registry: ResMut<ChunkLoaderRegistry>,
) {
    let mut upgrade_to_chunk_loader_entity_events = Vec::new();
    for upgrade_to_chunk_loader_entity_event in upgrade_to_chunk_loader_entity_event_reader.read() {
        upgrade_to_chunk_loader_entity_events.push(upgrade_to_chunk_loader_entity_event);
    }

    for upgrade_to_chunk_loader_entity_event in upgrade_to_chunk_loader_entity_events {
        let chunk_loader_request_id = upgrade_to_chunk_loader_entity_event.chunk_loader_request_id;
        let target_entity_id = upgrade_to_chunk_loader_entity_event.target_entity_id;
        let chunk_loader_id = chunk_loader_registry.register_chunk_loader();

        info!("Trying to upgrade entity '{:?}' to a chunk loader entity ...", target_entity_id);

        upgrade_to_chunk_loader_entity_internal_event_writer.send(UpgradeToChunkLoaderEntityInternal {
            chunk_loader_request_id,
            chunk_loader_id,
            target_entity_id,
        });
    }
}

pub(super) fn handle_create_chunk_loader_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<CreateChunkLoaderEntityInternal>,
        EventWriter<CreatedChunkLoaderEntityInternal>,
    )>,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkLoaderRegistry>,
        ResMut<EntityRegistry>,
    )>,
) {
    let mut create_chunk_loader_entity_event_reader = event_parameters.get_mut(world).0;

    let mut create_chunk_loader_entity_events: Vec<CreateChunkLoaderEntityInternal> = Vec::new();
    for create_chunk_loader_entity_event in create_chunk_loader_entity_event_reader.read() {
        create_chunk_loader_entity_events.push(create_chunk_loader_entity_event.clone());
    }

    for create_chunk_loader_entity_event in create_chunk_loader_entity_events {
        let chunk_loader_request_id = create_chunk_loader_entity_event.chunk_loader_request_id;
        let chunk_loader_id = create_chunk_loader_entity_event.chunk_loader_id;
        let chunk_loader_entity_id = create_chunk_loader_entity_event.chunk_loader_entity_id;
        let world_position = create_chunk_loader_entity_event.world_position;

        let chunk_loader_entity_reference = chunk_loader_functions::new_chunk_loader_entity(world, chunk_loader_id, world_position);

        let (mut chunk_loader_registry, mut entity_registry) = registry_parameters.get_mut(world);
        entity_registry.load_entity(chunk_loader_entity_id, chunk_loader_entity_reference);
        chunk_loader_registry.load_chunk_loader(chunk_loader_id, chunk_loader_entity_reference);

        chunk_loader_registry.stop_creating_chunk_loader(chunk_loader_id);

        info!("Successfully created chunk loader entity '{:?}' at world position '{:?}'!", chunk_loader_entity_id, world_position);

        let mut created_chunk_loader_entity_event_writer = event_parameters.get_mut(world).1;
        created_chunk_loader_entity_event_writer.send(CreatedChunkLoaderEntityInternal::Success {
            chunk_loader_request_id,
            chunk_loader_id,
            chunk_loader_entity_id,
            world_position
        });
    }
}

pub(super) fn handle_destroy_chunk_loader_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<DestroyChunkLoaderEntityInternal>,
        EventWriter<DestroyedChunkLoaderEntityInternal>,
    )>,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkLoaderRegistry>,
        ResMut<EntityRegistry>,
    )>,
) {
    let mut destroy_chunk_loader_entity_event_reader = event_parameters.get_mut(world).0;

    let mut destroy_chunk_loader_entity_events: Vec<DestroyChunkLoaderEntityInternal> = Vec::new();
    for destroy_chunk_loader_entity_event in destroy_chunk_loader_entity_event_reader.read() {
        destroy_chunk_loader_entity_events.push(destroy_chunk_loader_entity_event.clone());
    }

    for destroy_chunk_loader_entity_event in destroy_chunk_loader_entity_events {
        let chunk_loader_request_id = destroy_chunk_loader_entity_event.chunk_loader_request_id;
        let chunk_loader_id = destroy_chunk_loader_entity_event.chunk_loader_id;

        let (mut chunk_loader_registry, mut entity_registry) = registry_parameters.get_mut(world);

        let chunk_loader_entity_reference = match chunk_loader_registry.get_loaded_chunk_loader(chunk_loader_id) {
            Some(chunk_loader_entity) => chunk_loader_entity,
            None => {
                panic!("The chunk loader entity reference for chunk loader '{:?}' could not be found!", chunk_loader_id);
            }
        };

        let chunk_loader_entity_id = match entity_registry.get_loaded_entity_id(&chunk_loader_entity_reference) {
            Some(chunk_loader_entity_id) => chunk_loader_entity_id,
            None => {
                panic!("The chunk loader entity ID for chunk loader '{:?}' could not be found!", chunk_loader_id);
            }
        };

        let _ = chunk_loader_registry.unload_chunk_loader(chunk_loader_id);
        let _ = entity_registry.unload_entity(chunk_loader_entity_id);

        world.despawn(chunk_loader_entity_reference);

        let (mut chunk_loader_registry, mut entity_registry) = registry_parameters.get_mut(world);
        entity_registry.unregister_entity(chunk_loader_entity_id);
        chunk_loader_registry.unregister_chunk_loader(chunk_loader_id);

        chunk_loader_registry.stop_destroying_chunk_loader(chunk_loader_id);

        info!("Successfully destroyed chunk loader '{:?}' entity '{:?}'!", chunk_loader_id, chunk_loader_entity_id);

        let mut destroyed_chunk_loader_entity_event_writer = event_parameters.get_mut(world).1;
        destroyed_chunk_loader_entity_event_writer.send(DestroyedChunkLoaderEntityInternal::Success {
            chunk_loader_request_id,
            chunk_loader_id
        });
    }
}

pub(super) fn handle_upgrade_to_chunk_loader_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<UpgradeToChunkLoaderEntityInternal>,
        EventWriter<UpgradedToChunkLoaderEntityInternal>,
    )>,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkLoaderRegistry>,
        Res<EntityRegistry>,
    )>,
) {
    let mut upgrade_to_chunk_loader_entity_event_reader = event_parameters.get_mut(world).0;

    let mut upgrade_to_chunk_loader_entity_events: Vec<UpgradeToChunkLoaderEntityInternal> = Vec::new();
    for upgrade_to_chunk_loader_entity_event in upgrade_to_chunk_loader_entity_event_reader.read() {
        upgrade_to_chunk_loader_entity_events.push(upgrade_to_chunk_loader_entity_event.clone());
    }

    for upgrade_to_chunk_loader_entity_event in upgrade_to_chunk_loader_entity_events {
        let chunk_loader_request_id = upgrade_to_chunk_loader_entity_event.chunk_loader_request_id;
        let chunk_loader_id = upgrade_to_chunk_loader_entity_event.chunk_loader_id;
        let target_entity_id = upgrade_to_chunk_loader_entity_event.target_entity_id;

        let (_, entity_registry) = registry_parameters.get_mut(world);

        let target_entity_reference = match entity_registry.get_loaded_entity_reference(&target_entity_id) {
            Some(target_entity) => target_entity,
            None => {
                warn!("The request for upgrading entity '{:?}' to a chunk loader entity has been cancelled due to the entity reference not being found!", target_entity_id);

                let mut upgraded_to_chunk_loader_entity_event_writer = event_parameters.get_mut(world).1;
                upgraded_to_chunk_loader_entity_event_writer.send(UpgradedToChunkLoaderEntityInternal::Failure {
                    chunk_loader_request_id,
                    target_entity_id,
                });

                continue;
            }
        };

        let chunk_loader_entity_reference = match chunk_loader_functions::upgrade_to_chunk_loader_entity(
            world, 
            chunk_loader_id, 
            target_entity_reference
        ) {
            Ok(chunk_loader_entity_reference) => chunk_loader_entity_reference,
            Err(_) => {
                warn!("The request for upgrading entity '{:?}' to a chunk loader entity has been cancelled due to the upgrade failing!", target_entity_id);

                let mut upgraded_to_chunk_loader_entity_event_writer = event_parameters.get_mut(world).1;
                upgraded_to_chunk_loader_entity_event_writer.send(UpgradedToChunkLoaderEntityInternal::Failure {
                    chunk_loader_request_id,
                    target_entity_id,
                });

                continue;
            }
        
        };

        let (mut chunk_loader_registry, _) = registry_parameters.get_mut(world);

        chunk_loader_registry.load_chunk_loader(chunk_loader_id, chunk_loader_entity_reference);

        info!("Successfully upgraded entity '{:?}' to a chunk loader '{:?}' entity!", target_entity_id, chunk_loader_id);

        let mut upgraded_to_chunk_loader_entity_event_writer = event_parameters.get_mut(world).1;

        upgraded_to_chunk_loader_entity_event_writer.send(UpgradedToChunkLoaderEntityInternal::Success {
            chunk_loader_request_id,
            chunk_loader_id,
            target_entity_id,
        });
    }
}

pub(super) fn handle_created_chunk_loader_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<CreatedChunkLoaderEntityInternal>,
        EventWriter<CreatedChunkLoaderEntity>,
    )>,
) {
    let mut created_chunk_loader_entity_event_reader = event_parameters.get_mut(world).0;

    let mut created_chunk_loader_entity_events: Vec<CreatedChunkLoaderEntityInternal> = Vec::new();
    for created_chunk_loader_entity_event in created_chunk_loader_entity_event_reader.read() {
        created_chunk_loader_entity_events.push(created_chunk_loader_entity_event.clone());
    }

    for created_chunk_loader_entity_event in created_chunk_loader_entity_events {
        let mut created_chunk_loader_entity_event_writer = event_parameters.get_mut(world).1;

        match created_chunk_loader_entity_event {
            CreatedChunkLoaderEntityInternal::Success {
                chunk_loader_request_id, 
                chunk_loader_id, 
                chunk_loader_entity_id, 
                world_position
            } => {
                info!("Successfully created chunk loader '{:?}' at world position '{:?}'!", chunk_loader_id, world_position);

                created_chunk_loader_entity_event_writer.send(CreatedChunkLoaderEntity::Success { chunk_loader_request_id, chunk_loader_id, chunk_loader_entity_id, world_position });
            },
            CreatedChunkLoaderEntityInternal::Failure { chunk_loader_request_id, world_position } => {
                error!("Failed to create chunk loader at world position '{:?}'!", world_position);

                created_chunk_loader_entity_event_writer.send(CreatedChunkLoaderEntity::Failure { chunk_loader_request_id, world_position });
            },
        }
    }
}

pub(super) fn handle_destroyed_chunk_loader_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<DestroyedChunkLoaderEntityInternal>,
        EventWriter<DestroyedChunkLoaderEntity>,
    )>,
) {
    let mut destroyed_chunk_loader_entity_event_reader = event_parameters.get_mut(world).0;

    let mut destroyed_chunk_loader_entity_events: Vec<DestroyedChunkLoaderEntityInternal> = Vec::new();
    for destroyed_chunk_loader_entity_event in destroyed_chunk_loader_entity_event_reader.read() {
        destroyed_chunk_loader_entity_events.push(destroyed_chunk_loader_entity_event.clone());
    }

    for destroyed_chunk_loader_entity_event in destroyed_chunk_loader_entity_events {
        let mut destroyed_chunk_loader_entity_event_writer = event_parameters.get_mut(world).1;

        match destroyed_chunk_loader_entity_event {
            DestroyedChunkLoaderEntityInternal::Success { chunk_loader_request_id, chunk_loader_id } => {
                info!("Successfully destroyed chunk loader '{:?}'!", chunk_loader_id);

                destroyed_chunk_loader_entity_event_writer.send(DestroyedChunkLoaderEntity::Success { chunk_loader_request_id, chunk_loader_id });
            },
            DestroyedChunkLoaderEntityInternal::Failure { chunk_loader_request_id, chunk_loader_id } => {
                error!("Failed to destroy chunk loader '{:?}'!", chunk_loader_id);

                destroyed_chunk_loader_entity_event_writer.send(DestroyedChunkLoaderEntity::Failure { chunk_loader_request_id, chunk_loader_id });
            },
        }
    }
}

pub(super) fn handle_upgraded_to_chunk_loader_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<UpgradedToChunkLoaderEntityInternal>,
        EventWriter<UpgradedToChunkLoaderEntity>,
    )>,
) {
    let mut upgraded_to_chunk_loader_entity_event_reader = event_parameters.get_mut(world).0;

    let mut upgraded_to_chunk_loader_entity_events: Vec<UpgradedToChunkLoaderEntityInternal> = Vec::new();
    for upgraded_to_chunk_loader_entity_event in upgraded_to_chunk_loader_entity_event_reader.read() {
        upgraded_to_chunk_loader_entity_events.push(upgraded_to_chunk_loader_entity_event.clone());
    }

    for upgraded_to_chunk_loader_entity_event in upgraded_to_chunk_loader_entity_events {
        let mut upgraded_to_chunk_loader_entity_event_writer = event_parameters.get_mut(world).1;

        match upgraded_to_chunk_loader_entity_event {
            UpgradedToChunkLoaderEntityInternal::Success { chunk_loader_request_id, chunk_loader_id, target_entity_id } => {
                info!("Successfully upgraded entity '{:?}' to a chunk loader '{:?}' entity!", target_entity_id, chunk_loader_id);

                upgraded_to_chunk_loader_entity_event_writer.send(UpgradedToChunkLoaderEntity::Success { chunk_loader_request_id, chunk_loader_id, target_entity_id });
            },
            UpgradedToChunkLoaderEntityInternal::Failure { chunk_loader_request_id, target_entity_id } => {
                error!("Failed to upgrade entity '{:?}' to a chunk loader entity!", target_entity_id);

                upgraded_to_chunk_loader_entity_event_writer.send(UpgradedToChunkLoaderEntity::Failure { chunk_loader_request_id, target_entity_id });
            },
        }
    }
}
