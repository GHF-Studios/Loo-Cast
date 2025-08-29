use bevy::ecs::{component::HookContext, world::DeferredWorld};

use crate::{chunk::components::Chunk, chunk_loader::resources::RemovedChunkLoaders};

pub(crate) fn hook_on_add_chunk(mut world: DeferredWorld<'_>, hook_context: HookContext) {
    let HookContext { entity, component_id: _, caller: _, relationship_hook_mode: _ } = hook_context;
    let (chunk, chunk_coord) = match world.get::<Chunk>(entity) {
        Some(chunk) => (chunk, chunk.coord),
        None => return,
    };

    let chunk_owner_id = match chunk.owner_id.clone() {
        Some(owner_id) => owner_id,
        None => return,
    };

    let removed_chunk_loaders = world.resource_mut::<RemovedChunkLoaders>();
    let found_removal_event = removed_chunk_loaders.0.iter().any(|rcl| rcl.id == chunk_owner_id);

    if world.get_entity(chunk_owner_id.entity()).is_err() && !found_removal_event {
        panic!(
            "Spawned chunk {:?} with non-existent owner_id {:?}. 
            The chunk cannot be safely despawned, due to no respective removal event being found.
            In other words: The chunk references a non-existent owner at creation, which is an invariant violation in combination with no removal event being found!", 
            chunk_coord, chunk_owner_id
        );
    }
}

