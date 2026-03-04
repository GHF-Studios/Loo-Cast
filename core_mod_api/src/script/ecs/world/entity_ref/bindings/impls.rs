use crate::bevy::ecs::entity::Entity as BevyEntity;
use crate::rhai_binding::value_semantics::access_traits::ReadAccessProvider;
use crate::script::{
    ecs::world::entity_ref::{
        bindings::types::{EntityMut, EntityRef, EntityWorldMut},
        internals::traits::{EntityMutApi, EntityRefApi, EntityWorldMutApi}
    }
};

impl EntityRefApi for EntityRef {
    fn id(&self) -> BevyEntity {
        let entity_ref = self.entity_ref.start_read();
        let id = entity_ref.access("id", Box::new(()));
        self.entity_ref.end_read(entity_ref);
        id
    }
}

impl EntityMutApi for EntityMut {
    fn id(&self) -> BevyEntity {
        let entity_mut = self.entity_mut.start_read();
        let id = entity_mut.access("id", Box::new(()));
        self.entity_mut.end_read(entity_mut);
        id
    }
}

impl EntityWorldMutApi for EntityWorldMut {
    fn id(&self) -> BevyEntity {
        let entity_world_mut = self.entity_world_mut.start_read();
        let id = entity_world_mut.access("id", Box::new(()));
        self.entity_world_mut.end_read(entity_world_mut);
        id
    }
}
