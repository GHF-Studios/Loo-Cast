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