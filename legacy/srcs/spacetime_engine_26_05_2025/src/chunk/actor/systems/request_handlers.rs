use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::chunk::actor::events::*;
use crate::chunk::actor::position::structs::ChunkActorPosition;
use crate::chunk::actor::resources::*;
use crate::chunk::id::structs::ChunkID;
use crate::chunk::position::structs::ChunkPosition;
use crate::entity::resources::*;
use crate::chunk::actor::functions::util::*;

pub fn handle_upgrade_to_chunk_actor(
    world: &mut World,
    event_parameters: &mut SystemState<
        EventReader<UpgradeToChunkActor>,
    >,
    registry_parameters: &mut SystemState<(
        ResMut<EntityRegistry>,
        ResMut<ChunkActorRegistry>
    )>,
) {
    let mut upgrade_to_chunk_actor_event_reader = event_parameters.get_mut(world);

    let mut upgrade_to_chunk_actor_events = Vec::new();
    for upgrade_to_chunk_actor_event in upgrade_to_chunk_actor_event_reader.read() {
        upgrade_to_chunk_actor_events.push(upgrade_to_chunk_actor_event.clone());
    }

    for upgrade_to_chunk_actor_event in upgrade_to_chunk_actor_events {
        let upgrade_chunk_actor_request = upgrade_to_chunk_actor_event.0;

        let chunk_actor_id = upgrade_chunk_actor_request.chunk_actor_id;
        let chunk_actor_entity_id = upgrade_chunk_actor_request.chunk_actor_entity_id;

        let entity_reference = {
            let entity_registry = registry_parameters.get_mut(world).0;

            let entity_reference = match entity_registry.get_loaded_entity_reference(&chunk_actor_entity_id) {
                Some(entity_reference) => entity_reference.clone(),
                None => {
                    panic!("Entity reference associated with entity id '{:?}' not found!", chunk_actor_entity_id);
                }
            };

            entity_reference
        };

        let start_chunk_id = {
            let transform = match world.get::<Transform>(entity_reference) {
                Some(transform) => transform,
                None => {
                    panic!("Entity '{:?}' has no Transform!", entity_reference);
                }
            };

            let position = transform.translation;
            let chunk_actor_pos: ChunkActorPosition = position.into();
            let chunk_pos: ChunkPosition = chunk_actor_pos.into();
            let chunk_id: ChunkID = chunk_pos.into();

            chunk_id
        };

        upgrade_to_chunk_actor(world, start_chunk_id, chunk_actor_id, entity_reference);
    }
}

pub fn handle_downgrade_from_chunk_actor(
    world: &mut World,
    event_parameters: &mut SystemState<
        EventReader<DowngradeFromChunkActor>,
    >,
    registry_parameters: &mut SystemState<(
        ResMut<EntityRegistry>,
        ResMut<ChunkActorRegistry>
    )>,
) {
    let mut downgrade_from_chunk_actor_event_reader = event_parameters.get_mut(world);

    let mut downgrade_from_chunk_actor_events = Vec::new();
    for downgrade_from_chunk_actor_event in downgrade_from_chunk_actor_event_reader.read() {
        downgrade_from_chunk_actor_events.push(downgrade_from_chunk_actor_event.clone());
    }

    for downgrade_from_chunk_actor_event in downgrade_from_chunk_actor_events {
        let downgrade_chunk_actor_request = downgrade_from_chunk_actor_event.0;

        let chunk_actor_id = downgrade_chunk_actor_request.chunk_actor_id;
        let entity_reference = {
            let chunk_actor_registry = registry_parameters.get_mut(world).1;

            let entity_reference = match chunk_actor_registry.get_loaded_chunk_actor_entity(chunk_actor_id) {
                Some(chunk_actor_entity_reference) => chunk_actor_entity_reference.clone(),
                None => {
                    panic!("Chunk actor entity reference associated with chunk actor id '{:?}' not found!", chunk_actor_id);
                }
            };

            entity_reference
        };

        downgrade_from_chunk_actor(world, entity_reference);
    }
}
