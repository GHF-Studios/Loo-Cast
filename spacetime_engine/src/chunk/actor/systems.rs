use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk::events::CreatedChunk;
use crate::chunk::id::structs::*;
use crate::chunk::position::structs::*;
use crate::chunk::actor::position::structs::*;
use crate::chunk::actor::components::*;
use crate::chunk::actor::resources::*;
use crate::chunk::actor::structs::*;
use crate::chunk::components::*;
use crate::chunk::resources::*;
use crate::entity::resources::EntityRegistry;
use super::events::*;
use super::functions;


pub(in crate) fn update(
    world: &mut World,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
    )>,
) {
    let (updates, despawns) = collect_actor_updates(world, registry_parameters);

    apply_actor_updates(
        world,
        registry_parameters,
        updates,
        despawns,
    );
}

fn collect_actor_updates(
    world: &mut World,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
    )>,
) -> (Vec<ChunkActorUpdateInfo>, Vec<ChunkActorDespawnInfo>) {
    let mut chunk_actor_query = world.query::<(Entity, &Transform, &ChunkActor)>();
    let chunk_actor_query_size = chunk_actor_query.iter(world).count();
    let mut chunk_ids = Vec::new();
    let mut chunk_actor_ids = Vec::new();
    let mut chunk_actor_entities = Vec::new();
    let mut old_chunk_ids = Vec::new();

    for (chunk_actor_entity, chunk_actor_transform, chunk_actor) in chunk_actor_query.iter(world) {
        let actor_position: ChunkActorPosition = chunk_actor_transform.translation.into();
        let chunk_position: ChunkPosition = actor_position.into();
        let chunk_id: ChunkID = chunk_position.into();
        let chunk_actor_id = chunk_actor.id();
        let old_chunk_id = chunk_actor.current_chunk();

        chunk_ids.push(chunk_id);
        chunk_actor_ids.push(chunk_actor_id);
        chunk_actor_entities.push(chunk_actor_entity);
        old_chunk_ids.push(old_chunk_id);
    }

    let mut updates = Vec::new();
    let mut despawns = Vec::new();

    for i in 0..chunk_actor_query_size {
        let chunk_id = chunk_ids[i];
        let chunk_actor_id = chunk_actor_ids[i];
        let chunk_actor_entity = chunk_actor_entities[i];
        let old_chunk_id = old_chunk_ids[i];

        let (chunk_registry, _) = registry_parameters.get_mut(world);
        
        if !chunk_registry.is_chunk_loaded(chunk_id) {
            despawns.push(ChunkActorDespawnInfo {
                actor_entity: chunk_actor_entity,
                actor_id: chunk_actor_id,
            });
        } else if old_chunk_id != chunk_id {
            updates.push(ChunkActorUpdateInfo {
                actor_entity: chunk_actor_entity,
                old_chunk_id,
                new_chunk_id: chunk_id,
                actor_id: chunk_actor_id,
            });
        }
    }

    (updates, despawns)
}

fn apply_actor_updates(
    world: &mut World,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
    )>,
    updates: Vec<ChunkActorUpdateInfo>,
    despawns: Vec<ChunkActorDespawnInfo>,
) {
    let mut chunk_query = world.query::<&mut Chunk>();

    for update in updates {
        let (chunk_registry, _) = registry_parameters.get_mut(world);
        let old_chunk_entity = chunk_registry.get_loaded_chunk_entity(update.old_chunk_id).unwrap();
        let mut old_chunk = chunk_query.get_mut(world, old_chunk_entity).unwrap();
        old_chunk.remove_chunk_actor(update.actor_id);

        let (chunk_registry, _) = registry_parameters.get_mut(world);
        let new_chunk_entity = chunk_registry.get_loaded_chunk_entity(update.new_chunk_id).unwrap();
        let mut new_chunk = chunk_query.get_mut(world, new_chunk_entity).unwrap();
        new_chunk.add_chunk_actor(update.actor_id);
    }


    for despawn in despawns {
        world.despawn(despawn.actor_entity);
        let (_, mut chunk_actor_registry) = registry_parameters.get_mut(world);
        chunk_actor_registry.unload_chunk_actor(despawn.actor_id);
        chunk_actor_registry.unregister_chunk_actor(despawn.actor_id);
    }
}

pub(in crate) fn handle_create_chunk_actor_entity_events(
    mut create_chunk_actor_entity_event_reader: EventReader<CreateChunkActorEntity>,
    mut created_chunk_actor_entity_event_writer: EventWriter<CreatedChunkActorEntity>,
    chunk_registry: ResMut<ChunkRegistry>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    mut chunk_query: Query<&mut Chunk>,
) {
    let mut create_chunk_actor_entity_events = Vec::new();
    for create_chunk_actor_entity_event in create_chunk_actor_entity_event_reader.read() {
        create_chunk_actor_entity_events.push(create_chunk_actor_entity_event.clone());
    }

    for create_chunk_actor_entity_event in create_chunk_actor_entity_events {
        let chunk_actor_id = create_chunk_actor_entity_event.chunk_actor_id;
        let chunk_actor_entity_id = create_chunk_actor_entity_event.chunk_actor_entity_id;
        let chunk_id = create_chunk_actor_entity_event.chunk_id;
        let world_position = create_chunk_actor_entity_event.world_position;

        info!("Trying to create chunk actor entity '{:?}' ...", chunk_actor_entity_id);
        
        if let Some(chunk_entity) = chunk_registry.get_loaded_chunk_entity(chunk_id) {
            info!("Chunk loaded, creating chunk actor entity '{:?}' ...", chunk_actor_entity_id);

            // create the chunk and register it everywhere with the module function new_chunk_actor_entity
            // TODO: Change the existing module function so that it also registers the entity and chunk actor everywhere necessary, including the starting chunk, aka so that the resulting chunk actor entity is fully and immediately functional after having called this function
        } else {
            info!("Chunk not loaded, issuing request to create chunk actor entity '{:?}' ...", chunk_actor_entity_id);

            if chunk_actor_registry.is_chunk_actor_entity_creating(chunk_actor_id) {
                error!("The request for creating chunk actor entity '{:?}' has already been issued!", chunk_actor_entity_id);

                created_chunk_actor_entity_event_writer.send(CreatedChunkActorEntity {
                    chunk_actor_id,
                    chunk_actor_entity_id,
                    chunk_id,
                    world_position,
                    success: false,
                });

                continue;
            }
            
            chunk_actor_registry.start_creating_chunk_actor_entity(
                ChunkActorCreateRequest {
                    chunk_actor_id,
                    chunk_actor_entity_id,
                    chunk_id,
                    world_position,
                }
            );
        }
    }
}

pub(in crate) fn process_create_chunk_actor_requests(
    mut commands: Commands,
    mut created_chunk_event_reader: EventReader<CreatedChunk>,
    mut created_chunk_actor_entity_event_writer: EventWriter<CreatedChunkActorEntity>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    mut chunk_registry: ResMut<ChunkRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
    mut chunk_query: Query<&Chunk>,
) {
    for created_chunk_event in created_chunk_event_reader.read() {

        let chunk_id = created_chunk_event.chunk_id;
        let success = created_chunk_event.success;

        if !success {
            // check if any of the requested chunk actor entities are waiting for this chunk to be loaded
            // if so, essentially do nothing and wait, but send a warning log message, stating that the chunk actor entity creation has been postponed due to the starting chunk loading failure

            continue;
        }

        let chunk_entity = match chunk_registry.get_loaded_chunk_entity(chunk_id) {
            Some(chunk_entity) => chunk_entity,
            None => {
                continue;
            }
        };

        // LOGIC COMMENTS
        // create the chunk actor entity using the associated module function "new_chunk_actor_entity"
        // register it with the entity registry
        // register it with the chunk actor registry
        // register it with the chunk entity
        // send the CreatedChunkActorEntity event
        return;
        // END LOGIC COMMENTS

        // then tie this new chunk into the chunk creation system or whatever. 
        //Like, have a look at the revamped chunk actor system and see how this can benefit the chunk systems?
    }
}