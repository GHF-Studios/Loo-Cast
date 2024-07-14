use bevy::prelude::*;

use super::resources::*;
use super::events::*;
use super::structs::*;

pub(super) fn handle_create_entity_events(
    mut create_entity_event_reader: EventReader<CreateEntity>,
    mut create_entity_internal_event_writer: EventWriter<CreateEntityInternal>,
) {
    let mut create_entity_events = Vec::new();
    for create_entity_event in create_entity_event_reader.read() {
        create_entity_events.push(create_entity_event);
    }

    for create_entity_event in create_entity_events {
        let create_entity_request = &create_entity_event.0;
        
        let entity_request_id = create_entity_request.entity_request_id;
        let entity_id = create_entity_request.entity_id;

        info!("Trying to create entity '{:?}' ...", entity_id);

        create_entity_internal_event_writer.send(CreateEntityInternal(InternalEntityRequest {
            entity_request_id,
            entity_id,
            world_position: Vec2::new(0.0, 0.0),
        }));
    }
}

pub(super) fn handle_destroy_entity_events(
    mut destroy_entity_event_reader: EventReader<DestroyEntity>,
    mut destroy_entity_internal_event_writer: EventWriter<DestroyEntityInternal>,
) {
    let mut destroy_entity_events = Vec::new();
    for destroy_entity_event in destroy_entity_event_reader.read() {
        destroy_entity_events.push(destroy_entity_event);
    }

    for destroy_entity_event in destroy_entity_events {
        let destroy_entity_request = &destroy_entity_event.0;
        
        let entity_request_id = destroy_entity_request.entity_request_id;
        let entity_id = destroy_entity_request.entity_id;

        info!("Trying to destroy entity '{:?}' ...", entity_id);

        destroy_entity_internal_event_writer.send(DestroyEntityInternal(InternalEntityRequest {
            entity_request_id,
            entity_id,
            world_position: Vec2::new(0.0, 0.0),
        }));
    }
}

pub(super) fn handle_create_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<CreateEntityInternal>,
        EventWriter<CreatedEntityInternal>,
    )>,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkLoaderRegistry>,
        ResMut<EntityRegistry>,
    )>,
) {
    let mut create_entity_event_reader = event_parameters.get_mut(world).0;

    let mut create_entity_events: Vec<CreateEntityInternal> = Vec::new();
    for create_entity_event in create_entity_event_reader.read() {
        create_entity_events.push(create_entity_event.clone());
    }

    for create_entity_event in create_entity_events {
        let chunk_loader_request_id = create_entity_event.chunk_loader_request_id;
        let chunk_loader_id = create_entity_event.chunk_loader_id;
        let entity_id = create_entity_event.entity_id;
        let world_position = create_entity_event.world_position;

        let entity_reference = chunk_loader_functions::new_entity(world, chunk_loader_id, world_position);

        let (mut chunk_loader_registry, mut entity_registry) = registry_parameters.get_mut(world);
        entity_registry.load_entity(entity_id, entity_reference);
        chunk_loader_registry.load_chunk_loader(chunk_loader_id, entity_reference);

        chunk_loader_registry.stop_creating_chunk_loader(chunk_loader_id);

        let mut created_entity_event_writer = event_parameters.get_mut(world).1;
        created_entity_event_writer.send(CreatedEntityInternal::Success {
            chunk_loader_request_id,
            chunk_loader_id,
            entity_id,
            world_position
        });
    }
}

pub(super) fn handle_destroy_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<DestroyEntityInternal>,
        EventWriter<DestroyedEntityInternal>,
    )>,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkLoaderRegistry>,
        ResMut<EntityRegistry>,
    )>,
) {
    let mut destroy_entity_event_reader = event_parameters.get_mut(world).0;

    let mut destroy_entity_events: Vec<DestroyEntityInternal> = Vec::new();
    for destroy_entity_event in destroy_entity_event_reader.read() {
        destroy_entity_events.push(destroy_entity_event.clone());
    }

    for destroy_entity_event in destroy_entity_events {
        let chunk_loader_request_id = destroy_entity_event.chunk_loader_request_id;
        let chunk_loader_id = destroy_entity_event.chunk_loader_id;

        let (mut chunk_loader_registry, mut entity_registry) = registry_parameters.get_mut(world);

        let entity_reference = match chunk_loader_registry.get_loaded_chunk_loader(chunk_loader_id) {
            Some(entity) => entity,
            None => {
                panic!("The chunk loader entity reference for chunk loader '{:?}' could not be found!", chunk_loader_id);
            }
        };

        let entity_id = match entity_registry.get_loaded_entity_id(&entity_reference) {
            Some(entity_id) => entity_id,
            None => {
                panic!("The chunk loader entity ID for chunk loader '{:?}' could not be found!", chunk_loader_id);
            }
        };

        let _ = chunk_loader_registry.unload_chunk_loader(chunk_loader_id);
        let _ = entity_registry.unload_entity(entity_id);

        world.despawn(entity_reference);

        let (mut chunk_loader_registry, mut entity_registry) = registry_parameters.get_mut(world);
        entity_registry.unregister_entity(entity_id);
        chunk_loader_registry.unregister_chunk_loader(chunk_loader_id);

        chunk_loader_registry.stop_destroying_chunk_loader(chunk_loader_id);

        let mut destroyed_entity_event_writer = event_parameters.get_mut(world).1;
        destroyed_entity_event_writer.send(DestroyedEntityInternal::Success {
            chunk_loader_request_id,
            chunk_loader_id
        });
    }
}

pub(super) fn handle_created_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<CreatedEntityInternal>,
        EventWriter<CreatedEntity>,
    )>,
) {
    let mut created_entity_event_reader = event_parameters.get_mut(world).0;

    let mut created_entity_events: Vec<CreatedEntityInternal> = Vec::new();
    for created_entity_event in created_entity_event_reader.read() {
        created_entity_events.push(created_entity_event.clone());
    }

    for created_entity_event in created_entity_events {
        let mut created_entity_event_writer = event_parameters.get_mut(world).1;

        match created_entity_event {
            CreatedEntityInternal::Success {
                chunk_loader_request_id, 
                chunk_loader_id, 
                entity_id, 
                world_position
            } => {
                info!("Successfully created chunk loader '{:?}' at world position '{:?}'!", chunk_loader_id, world_position);

                created_entity_event_writer.send(CreatedEntity::Success { chunk_loader_request_id, chunk_loader_id, entity_id, world_position });
            },
            CreatedEntityInternal::Failure { chunk_loader_request_id, world_position } => {
                error!("Failed to create chunk loader at world position '{:?}'!", world_position);

                created_entity_event_writer.send(CreatedEntity::Failure { chunk_loader_request_id, world_position });
            },
        }
    }
}

pub(super) fn handle_destroyed_entity_internal_events(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<DestroyedEntityInternal>,
        EventWriter<DestroyedEntity>,
    )>,
) {
    let mut destroyed_entity_event_reader = event_parameters.get_mut(world).0;

    let mut destroyed_entity_events: Vec<DestroyedEntityInternal> = Vec::new();
    for destroyed_entity_event in destroyed_entity_event_reader.read() {
        destroyed_entity_events.push(destroyed_entity_event.clone());
    }

    for destroyed_entity_event in destroyed_entity_events {
        let mut destroyed_entity_event_writer = event_parameters.get_mut(world).1;

        match destroyed_entity_event {
            DestroyedEntityInternal::Success { chunk_loader_request_id, chunk_loader_id } => {
                info!("Successfully destroyed chunk loader '{:?}'!", chunk_loader_id);

                destroyed_entity_event_writer.send(DestroyedEntity::Success { chunk_loader_request_id, chunk_loader_id });
            },
            DestroyedEntityInternal::Failure { chunk_loader_request_id, chunk_loader_id } => {
                error!("Failed to destroy chunk loader '{:?}'!", chunk_loader_id);

                destroyed_entity_event_writer.send(DestroyedEntity::Failure { chunk_loader_request_id, chunk_loader_id });
            },
        }
    }
}
