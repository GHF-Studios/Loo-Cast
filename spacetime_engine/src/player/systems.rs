use crate::chunk::actor::id::structs::ChunkActorID;
use crate::chunk::actor::resources::ChunkActorRegistry;
use crate::chunk::components::Chunk;
use crate::chunk::id::structs::ChunkID;
use crate::chunk::resources::ChunkRegistry;
use crate::entity::id::structs::EntityID;
use crate::entity::resources::*;
use bevy::prelude::*;
use super::functions;

pub(in crate) fn startup(
    mut commands: Commands,
    mut player_startup_event_writer: EventWriter<super::events::Startup>,
    mut chunk_query: Query<&mut Chunk>,
    mut chunk_registry: ResMut<ChunkRegistry>,
    mut chunk_actor_registry: ResMut<ChunkActorRegistry>,
    mut entity_registry: ResMut<EntityRegistry>,
) {
    let (spawn_chunk_entity_id, spawn_chunk_id, player_entity_id, player_chunk_actor_id) =
        prepare_startup_data(&mut entity_registry, &mut chunk_actor_registry);

    let (spawn_chunk_entity, player_entity) = create_entities(
        &mut commands,
        spawn_chunk_id,
        player_chunk_actor_id,
    );

    apply_startup_state(
        &mut player_startup_event_writer,
        &mut chunk_query,
        &mut chunk_registry,
        &mut entity_registry,
        &mut chunk_actor_registry,
        spawn_chunk_entity,
        player_entity,
        spawn_chunk_entity_id,
        spawn_chunk_id,
        player_entity_id,
        player_chunk_actor_id,
    );
}

fn prepare_startup_data(
    entity_registry: &mut ResMut<EntityRegistry>,
    chunk_actor_registry: &mut ResMut<ChunkActorRegistry>,
) -> (EntityID, ChunkID, EntityID, ChunkActorID) {
    let spawn_chunk_entity_id = entity_registry.register_entity();
    let spawn_chunk_id = ChunkID::default();
    let player_entity_id = entity_registry.register_entity();
    let player_chunk_actor_id = chunk_actor_registry.register_chunk_actor();

    (spawn_chunk_entity_id, spawn_chunk_id, player_entity_id, player_chunk_actor_id)
}

fn create_entities(
    commands: &mut Commands,
    spawn_chunk_id: ChunkID,
    player_chunk_actor_id: ChunkActorID,
) -> (Entity, Entity) {
    let spawn_chunk_entity = crate::chunk::functions::new_chunk_entity(commands, spawn_chunk_id);
    let player_entity = functions::new_player_entity(commands, spawn_chunk_id, player_chunk_actor_id);

    (spawn_chunk_entity, player_entity)
}

#[allow(clippy::too_many_arguments)]
fn apply_startup_state(
    player_startup_event_writer: &mut EventWriter<super::events::Startup>,
    chunk_query: &mut Query<&mut Chunk>,
    chunk_registry: &mut ResMut<ChunkRegistry>,
    entity_registry: &mut ResMut<EntityRegistry>,
    chunk_actor_registry: &mut ResMut<ChunkActorRegistry>,
    spawn_chunk_entity: Entity,
    player_entity: Entity,
    spawn_chunk_entity_id: EntityID,
    spawn_chunk_id: ChunkID,
    player_entity_id: EntityID,
    player_chunk_actor_id: ChunkActorID,
) {
    entity_registry.load_entity(spawn_chunk_entity_id, spawn_chunk_entity);
    chunk_registry.register_chunk(spawn_chunk_id);
    chunk_registry.load_chunk(spawn_chunk_id, spawn_chunk_entity);

    entity_registry.load_entity(player_entity_id, player_entity);
    chunk_actor_registry.load_chunk_actor(player_chunk_actor_id, player_entity);

    let mut spawn_chunk = chunk_query.get_mut(spawn_chunk_entity).unwrap();
    spawn_chunk.add_chunk_actor(player_chunk_actor_id);

    player_startup_event_writer.send(super::events::Startup { player_entity_id });
}