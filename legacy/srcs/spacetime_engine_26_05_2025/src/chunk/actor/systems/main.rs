use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use crate::chunk::actor::components::ChunkActor;
use crate::chunk::actor::functions::{hooks::*, util::*};
use crate::chunk::actor::ChunkActorRegistry;
use crate::chunk::ChunkRegistry;

pub(in crate) fn setup(world: &mut World) {
    world
        .register_component_hooks::<ChunkActor>()
        .on_add(on_add_chunk_actor)
        .on_remove(on_remove_chunk_actor);
}

pub(in crate) fn update(
    world: &mut World,
    registry_parameters: &mut SystemState<(
        ResMut<ChunkRegistry>,
        ResMut<ChunkActorRegistry>,
    )>,
) {
    let (updates, despawns) = collect_chunk_actor_updates(world, registry_parameters);

    apply_chunk_actor_updates(
        world,
        registry_parameters,
        updates,
        despawns,
    );
}