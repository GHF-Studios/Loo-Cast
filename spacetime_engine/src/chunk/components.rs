use bevy::{ecs::component::StorageType, prelude::*};

#[derive(Default, Debug)]
pub struct ChunkComponent {
    pub coord: (i32, i32),
    pub owner: Option<Entity>,
}

impl Component for ChunkComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|world, entity, _component_id| {
            let chunk = match world.get::<ChunkComponent>(entity) {
                Some(chunk) => chunk,
                None => return
            };

            let chunk_owner = match chunk.owner {
                Some(owner) => owner,
                None => return
            };

            if world.get_entity(chunk_owner).is_none() {
                error!("Spawned chunk {:?} with invalid owner {}", chunk.coord, chunk_owner);
            }
        });
    }
}