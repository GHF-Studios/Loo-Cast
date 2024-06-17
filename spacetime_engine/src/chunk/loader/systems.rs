use bevy::prelude::*;
use crate::chunk::events::*;
use crate::chunk::resources::*;
use crate::chunk::loader::components::*;
use crate::chunk::loader::events::*;
use crate::chunk::functions as chunk_functions;
use super::functions as chunk_loader_functions;
use crate::entity::resources::EntityRegistry;
use super::resources::ChunkLoaderRegistry;	

pub(in crate) fn start(
    create_chunk_event_writer: EventWriter<CreateChunkEntity>,
    load_chunk_event_writer: EventWriter<LoadChunkEntity>,
    mut started_chunk_loader_event_writer: EventWriter<StartedChunkLoader>,
    mut chunk_loader_query: Query<(&Transform, &mut ChunkLoader), Added<ChunkLoader>>,
    chunk_registry: Res<ChunkRegistry>,
) {
    let (chunk_loader_transform, mut chunk_loader) = chunk_loader_query.single_mut();
    let chunk_loader_id = chunk_loader.id();
    let chunk_loader_load_radius = chunk_loader.load_radius();
    
    let detected_chunk_ids = chunk_functions::detect_chunks(chunk_loader_transform, chunk_loader_load_radius);

    chunk_functions::start_chunks(
        create_chunk_event_writer, 
        load_chunk_event_writer, 
        &chunk_registry, 
        &detected_chunk_ids
    );

    *chunk_loader.current_chunk_ids_mut() = detected_chunk_ids;

    started_chunk_loader_event_writer.send(StartedChunkLoader::Success { chunk_loader_id });
}

pub(in crate) fn update(
    create_chunk_event_writer: EventWriter<CreateChunkEntity>,
    load_chunk_event_writer: EventWriter<LoadChunkEntity>,
    unload_chunk_event_writer: EventWriter<UnloadChunkEntity>,
    mut chunk_loader_query: Query<(&Transform, &mut ChunkLoader)>,
    chunk_registry: Res<ChunkRegistry>,
) {
    let (chunk_loader_transform, mut chunk_loader) = chunk_loader_query.single_mut();
    let chunk_loader_load_radius = chunk_loader.load_radius();

    let detected_chunk_ids = chunk_functions::detect_chunks(chunk_loader_transform, chunk_loader_load_radius);

    let (
        old_chunk_ids, 
        unchanged_chunk_ids, 
        new_chunk_ids
    ) = chunk_functions::categorize_chunks(detected_chunk_ids, &chunk_registry);

    chunk_functions::update_chunks(
        old_chunk_ids, 
        new_chunk_ids.clone(), 
        &chunk_registry, 
        create_chunk_event_writer, 
        load_chunk_event_writer, 
        unload_chunk_event_writer
    );

    *chunk_loader.current_chunk_ids_mut() = vec![unchanged_chunk_ids, new_chunk_ids].concat();
}

pub(super) fn handle_create_chunk_loader_entity_events(
    mut commands: Commands,
    mut create_chunk_loader_entity_event_reader: EventReader<CreateChunkLoaderEntity>,
    mut created_chunk_loader_entity_event_writer: EventWriter<CreatedChunkLoaderEntity>,
    mut chunk_loader_registry: ResMut<ChunkLoaderRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    let mut create_chunk_loader_entity_events = Vec::new();
    for create_chunk_loader_entity_event in create_chunk_loader_entity_event_reader.read() {
        create_chunk_loader_entity_events.push(create_chunk_loader_entity_event);
    }

    for create_chunk_loader_entity_event in create_chunk_loader_entity_events {
        let chunk_loader_entity_id = entity_registry.register_entity();
        let chunk_loader_id = chunk_loader_registry.register_chunk_loader();
        let world_position = create_chunk_loader_entity_event.world_position;

        info!("Creating chunk loader entity '{:?}' at world position '{:?}'...", chunk_loader_entity_id, world_position);

        let chunk_loader_entity_reference = chunk_loader_functions::new_chunk_loader_entity(&mut commands, chunk_loader_id, world_position);

        entity_registry.load_entity(chunk_loader_entity_id, chunk_loader_entity_reference);
        chunk_loader_registry.load_chunk_loader(chunk_loader_id, chunk_loader_entity_reference);

        created_chunk_loader_entity_event_writer.send(CreatedChunkLoaderEntity::Success {
            chunk_loader_id,
            chunk_loader_entity_id,
            world_position
        });
    }
}

pub(super) fn handle_destroy_chunk_loader_entity_events(
    mut commands: Commands,
    mut destroy_chunk_loader_entity_event_reader: EventReader<DestroyChunkLoaderEntity>,
    mut destroyed_chunk_loader_entity_event_writer: EventWriter<DestroyedChunkLoaderEntity>,
    mut chunk_loader_registry: ResMut<ChunkLoaderRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    let mut destroy_chunk_loader_entity_events = Vec::new();
    for destroy_chunk_loader_entity_event in destroy_chunk_loader_entity_event_reader.read() {
        destroy_chunk_loader_entity_events.push(destroy_chunk_loader_entity_event);
    }

    for destroy_chunk_loader_entity_event in destroy_chunk_loader_entity_events {
        let chunk_loader_id = destroy_chunk_loader_entity_event.chunk_loader_id;

        let chunk_loader_entity_reference = match chunk_loader_registry.get_loaded_chunk_loader(chunk_loader_id) {
            Some(chunk_loader_entity) => chunk_loader_entity,
            None => {
                error!("The request for destroying the chunk loader entity '{:?}' has been cancelled due to the chunk loader not being loaded!", chunk_loader_id);

                destroyed_chunk_loader_entity_event_writer.send(DestroyedChunkLoaderEntity::Failure { chunk_loader_id });

                continue;
            }
        };

        let chunk_loader_entity_id = match entity_registry.get_loaded_entity_id(&chunk_loader_entity_reference) {
            Some(chunk_loader_entity_id) => chunk_loader_entity_id,
            None => {
                error!("The request for destroying the chunk loader entity '{:?}' has been cancelled due to the respective chunk loader entity id not being found!", chunk_loader_id);

                destroyed_chunk_loader_entity_event_writer.send(DestroyedChunkLoaderEntity::Failure { chunk_loader_id });

                continue;
            }
        };

        chunk_loader_registry.unload_chunk_loader(chunk_loader_id);
        entity_registry.unload_entity(chunk_loader_entity_id);

        chunk_loader_registry.unregister_chunk_loader(chunk_loader_id);
        entity_registry.unregister_entity(chunk_loader_entity_id);

        commands.entity(chunk_loader_entity_reference).despawn();

        destroyed_chunk_loader_entity_event_writer.send(DestroyedChunkLoaderEntity::Success {
            chunk_loader_id,
        });
    }
}

pub(super) fn handle_upgrade_to_chunk_loader_entity_events(
    mut commands: Commands,
    mut upgrade_to_chunk_loader_entity_event_reader: EventReader<UpgradeToChunkLoaderEntity>,
    mut upgraded_to_chunk_loader_entity_event_writer: EventWriter<UpgradedToChunkLoaderEntity>,
    mut chunk_loader_registry: ResMut<ChunkLoaderRegistry>,
    entity_registry: Res<EntityRegistry>,
    mut ineligible_entity_query_0: Query<Entity, Without<Transform>>,
    mut ineligible_entity_query_1: Query<Entity, With<ChunkLoader>>,
    mut eligible_entity_query: Query<Entity, (With<Transform>, Without<ChunkLoader>)>,
) {
    let mut upgrade_to_chunk_loader_entity_events = Vec::new();
    for upgrade_to_chunk_loader_entity_event in upgrade_to_chunk_loader_entity_event_reader.read() {
        upgrade_to_chunk_loader_entity_events.push(upgrade_to_chunk_loader_entity_event);
    }

    for upgrade_to_chunk_loader_entity_event in upgrade_to_chunk_loader_entity_events {
        let target_entity_id = upgrade_to_chunk_loader_entity_event.target_entity_id;
        let chunk_loader_id = chunk_loader_registry.register_chunk_loader();

        info!("Upgrading entity '{:?}' to a chunk loader entity '{:?}'...", target_entity_id, chunk_loader_id);

        let target_entity_reference = match entity_registry.get_loaded_entity_reference(&target_entity_id) {
            Some(target_entity) => target_entity,
            None => {
                error!("The request for upgrading entity '{:?}' to a chunk loader entity has been cancelled due to the entity reference not being found!", target_entity_id);

                chunk_loader_registry.unregister_chunk_loader(chunk_loader_id);

                upgraded_to_chunk_loader_entity_event_writer.send(UpgradedToChunkLoaderEntity::Failure {
                    target_entity_id,
                });

                continue;
            }
        };

        let chunk_loader_entity_reference = match chunk_loader_functions::upgrade_to_chunk_loader_entity(
            &mut commands, 
            chunk_loader_id, 
            target_entity_reference,
            &mut ineligible_entity_query_0,
            &mut ineligible_entity_query_1,
            &mut eligible_entity_query
        ) {
            Ok(chunk_loader_entity_reference) => chunk_loader_entity_reference,
            Err(_) => {
                error!("The request for upgrading entity '{:?}' to a chunk loader entity has been cancelled due to the upgrade failing!", target_entity_id);

                chunk_loader_registry.unregister_chunk_loader(chunk_loader_id);

                upgraded_to_chunk_loader_entity_event_writer.send(UpgradedToChunkLoaderEntity::Failure {
                    target_entity_id,
                });

                continue;
            }
        
        };

        chunk_loader_registry.load_chunk_loader(chunk_loader_id, chunk_loader_entity_reference);

        upgraded_to_chunk_loader_entity_event_writer.send(UpgradedToChunkLoaderEntity::Success {
            chunk_loader_id,
            target_entity_id,
        });
    }
}
