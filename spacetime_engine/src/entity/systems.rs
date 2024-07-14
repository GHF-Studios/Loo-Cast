use bevy::prelude::*;

pub(super) fn handle_create_entity_events(
    mut create_entity_event_reader: EventReader<CreateEntity>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    let mut create_chunk_actor_entity_events = Vec::new();
    for create_chunk_actor_entity_event in create_chunk_actor_entity_event_reader.read() {
        create_chunk_actor_entity_events.push(create_chunk_actor_entity_event.clone());
    }

    for create_chunk_actor_entity_event in create_chunk_actor_entity_events {
        let chunk_actor_request_id = create_chunk_actor_entity_event.chunk_actor_request_id;
        let chunk_actor_entity_id = entity_registry.register_entity();
        let chunk_actor_id = chunk_actor_registry.register_chunk_actor();

        let chunk_id = {
            let world_position = create_chunk_actor_entity_event.world_position;
            let chunk_actor_position: ChunkActorPosition = world_position.into();
            let chunk_position: ChunkPosition = chunk_actor_position.into();
            let chunk_id: ChunkID = chunk_position.into();

            chunk_id
        };
        let world_position = create_chunk_actor_entity_event.world_position;

        info!("Trying to create chunk actor entity '{:?}' at world position '{:?}' in chunk '{:?}' ...", chunk_actor_entity_id, world_position, chunk_id);

        chunk_actor_registry.start_creating_chunk_actor(ChunkActorCreateRequest {
            chunk_actor_request_id,
            chunk_actor_id,
            chunk_actor_entity_id,
            chunk_id,
            world_position
        });
    }
}
