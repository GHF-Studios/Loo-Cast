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
    // Create a new chunk actor entity, and also attach a ProxyPlayer to it, 
    // containing all of the startup information, which can be used to register the player with the starting chunk
    // (which is a necessary step in being useable in my game world) specifically after said starting chunk has been loaded.
    // We achieve this by also having a system for starting the player entity, which will wait until the starting chunk has been loaded (via event) 
    // and then will remove the proxy player component and add the player component to the entity according to the contained information.








    let mut entity_registry = world.get_resource_mut::<EntityRegistry>().unwrap();
    let spawn_chunk_entity_id = entity_registry.register_entity();

    let spawn_chunk_id = ChunkID::default();
    let player_entity_id = entity_registry.register_entity();

    let mut chunk_actor_registry = world.get_resource_mut::<ChunkActorRegistry>().unwrap();
    let player_chunk_actor_id = chunk_actor_registry.register_chunk_actor();

    let player_entity = new_chunk_actor_entity(world, player_chunk_actor_id);

    apply_proxy_startup_state(
        world,
        event_writer_parameter,
        registry_parameters,
        player_entity,
        spawn_chunk_entity_id,
        spawn_chunk_id,
        player_entity_id,
        player_chunk_actor_id,
    );
}

pub(in crate) fn startup_internal(
    world: &mut World,
    event_parameters: &mut SystemState<(
        EventReader<crate::chunk::events::StartPlayerInternal>,
        EventWriter<super::events::StartedPlayer>,
    )>,
) {
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

// TODO: See player/functions.rs
#[allow(clippy::too_many_arguments)]
fn apply_proxy_startup_state(
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