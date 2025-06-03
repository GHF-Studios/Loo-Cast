use bevy::{ecs::component::StorageType, prelude::*};

#[derive(Default, Debug)]
pub struct ChunkComponent {
    pub coord: (i32, i32),
    owner: Option<Entity>,
}
impl ChunkComponent {
    pub fn transfer_ownership(&mut self, new_owner: Entity) -> Entity {
        self.owner.replace(new_owner).expect("Attempted to transfer ownership, but chunk state was invalidated: Chunk does not have an owner")
    }
}

impl Component for ChunkComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|world, entity, _component_id| {
            let chunk = match world.get::<ChunkComponent>(entity) {
                Some(chunk) => chunk,
                None => return,
            };

            let chunk_owner = match chunk.owner {
                Some(owner) => owner,
                None => return,
            };

            if world.get_entity(chunk_owner).is_none() {
                error!("Spawned chunk {:?} with non-existent owner {}", chunk.coord, chunk_owner);
            }
        });
    }
}
