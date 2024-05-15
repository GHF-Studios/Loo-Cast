use crate::chunk::actor::resources::ChunkActorRegistry;
use crate::chunk::components::Chunk;
use crate::chunk::id::structs::ChunkID;
use crate::chunk::resources::ChunkRegistry;
use crate::entity::resources::*;
use bevy::prelude::*;
use super::functions;

pub(in crate) fn startup(
    mut commands: Commands,
    mut player_startup_event_writer: EventWriter<super::events::Startup>,
    mut chunk_registry: ResMut<ChunkRegistry>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
    mut chunk_query: Query<&mut Chunk>,
) {
    let spawn_chunk_entity_id = entity_registry.register_entity();
    let spawn_chunk_id = ChunkID::default();
    
    chunk_registry.register_chunk(spawn_chunk_id);

    let spawn_chunk_entity = crate::chunk::functions::new_chunk_entity(&mut commands, spawn_chunk_id);





    // TODO: Also make all the chunk systems (except update) and the chunk actor system and the player create system and the player startup system use direct ECS world access and separetly read and handle events and use blocks and such to make this work
    // TODO: Use direct ECS world access instead of commands and generally fix everywhere else
    // TODO: Also implement/refactor & integrate custom position types for flat world position, deep world position(flat world position, but including the z-axis as a depth index) and chunk position, essentially generalizing chunk coordionate and chunk actor coordinate and other current coordinate/position types
    // TODO: Refer back to the main.rs file for further, more high-level TODOs

    // This do make big error because not direct ecs world access used to spawn chunk entity and to query the chunk
    let mut spawn_chunk = chunk_query.get_mut(spawn_chunk_entity).unwrap();
    
    entity_registry.load_entity(spawn_chunk_entity_id, spawn_chunk_entity);
    chunk_registry.load_chunk(spawn_chunk_id, spawn_chunk_entity);

    let player_entity_id = entity_registry.register_entity();
    let player_chunk_actor_id = chunk_actor_registry.register_chunk_actor();

    let player_entity = functions::new_player_entity(&mut commands, spawn_chunk_id, player_chunk_actor_id);

    entity_registry.load_entity(player_entity_id, player_entity);
    chunk_actor_registry.load_chunk_actor(player_chunk_actor_id, player_entity);

    spawn_chunk.add_chunk_actor(player_chunk_actor_id);

    player_startup_event_writer.send(super::events::Startup { player_entity_id });
}