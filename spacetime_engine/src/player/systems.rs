use crate::chunk::actor::id::structs::ChunkActorID;
use crate::chunk::actor::resources::ChunkActorRegistry;
use crate::chunk::components::Chunk;
use crate::chunk::id::structs::ChunkID;
use crate::chunk::loader::components::ChunkLoader;
use crate::chunk::resources::ChunkRegistry;
use crate::entity::id::structs::EntityID;
use crate::entity::resources::*;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use super::components::Player;
use super::functions;

pub(in crate) fn startup(
    world: &mut World,
    event_writer_parameter: &mut SystemState<(
        EventWriter<super::events::Startup>,
    )>,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
        ResMut<EntityRegistry>,
    )>,
) {
    let (
        spawn_chunk_entity_id, 
        spawn_chunk_id, 
        player_entity_id, 
        player_chunk_actor_id
    ) = prepare_startup_data(world);

    let (spawn_chunk_entity, player_entity) = create_entities(
        world,
        spawn_chunk_id,
        player_chunk_actor_id,
    );

    apply_startup_state(
        world,
        event_writer_parameter,
        registry_parameters,
        spawn_chunk_entity,
        player_entity,
        spawn_chunk_entity_id,
        spawn_chunk_id,
        player_entity_id,
        player_chunk_actor_id,
    );
}

pub(in crate) fn change_player_chunk_load_radius(
    mut chunk_loader_query: Query<(&mut ChunkLoader, &Player)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut chunk_loader, _) in chunk_loader_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            *chunk_loader.load_radius_mut() = (chunk_loader.load_radius() as i16 - 1).max(0) as u16;
        }
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            *chunk_loader.load_radius_mut() = (chunk_loader.load_radius() as i16 + 1) as u16;
        }
    }
}

fn prepare_startup_data(
    world: &mut World,
) -> (EntityID, ChunkID, EntityID, ChunkActorID) {
    let mut entity_registry = world.get_resource_mut::<EntityRegistry>().unwrap();
    let spawn_chunk_entity_id = entity_registry.register_entity();

    let spawn_chunk_id = ChunkID::default();
    let player_entity_id = entity_registry.register_entity();

    let mut chunk_actor_registry = world.get_resource_mut::<ChunkActorRegistry>().unwrap();
    let player_chunk_actor_id = chunk_actor_registry.register_chunk_actor();

    (spawn_chunk_entity_id, spawn_chunk_id, player_entity_id, player_chunk_actor_id)
}

fn create_entities(
    world: &mut World,
    spawn_chunk_id: ChunkID,
    player_chunk_actor_id: ChunkActorID,
) -> (Entity, Entity) {
    let spawn_chunk_entity = crate::chunk::functions::new_chunk_entity(world, spawn_chunk_id);
    let player_entity = functions::new_player_entity(world, spawn_chunk_id, player_chunk_actor_id);

    (spawn_chunk_entity, player_entity)
}

// TODO: See player/functions.rs
#[allow(clippy::too_many_arguments)]
fn apply_startup_state(
    world: &mut World,
    event_writer_parameter: &mut SystemState<(
        EventWriter<super::events::Startup>,
    )>,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
        ResMut<EntityRegistry>,
    )>,
    spawn_chunk_entity: Entity,
    player_entity: Entity,
    spawn_chunk_entity_id: EntityID,
    spawn_chunk_id: ChunkID,
    player_entity_id: EntityID,
    player_chunk_actor_id: ChunkActorID,
) {
    let (mut chunk_registry, mut chunk_actor_registry, mut entity_registry) = registry_parameters.get_mut(world);
    //entity_registry.load_entity(spawn_chunk_entity_id, spawn_chunk_entity);
    //chunk_registry.register_chunk(spawn_chunk_id);
    //chunk_registry.load_chunk(spawn_chunk_id, spawn_chunk_entity);

    //entity_registry.load_entity(player_entity_id, player_entity);
    //chunk_actor_registry.load_chunk_actor(player_chunk_actor_id, player_entity);

    let mut chunk_query = world.query::<&mut Chunk>();
    let mut spawn_chunk = chunk_query.get_mut(world, spawn_chunk_entity).unwrap();
    spawn_chunk.add_chunk_actor(player_chunk_actor_id);

    let mut player_startup_event_writer = event_writer_parameter.get_mut(world).0;
    player_startup_event_writer.send(super::events::Startup { player_entity_id });
}