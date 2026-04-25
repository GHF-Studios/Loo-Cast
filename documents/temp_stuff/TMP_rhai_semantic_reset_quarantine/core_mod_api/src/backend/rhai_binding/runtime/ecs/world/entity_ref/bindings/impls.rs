use rhai::{Array, ImmutableString};

use crate::bevy::ecs::entity::Entity as BevyEntity;
use crate::rhai_binding::runtime::ecs::component::{
    bindings::types::Component,
    internals::statics::{COMPONENT_CTOR_REGISTRY, COMPONENT_REMOVE_REGISTRY},
};
use crate::rhai_binding::runtime::ecs::world::entity_ref::{
    bindings::types::{EntityMut, EntityRef, EntityWorldMut},
    internals::traits::{EntityMutApi, EntityRefApi, EntityWorldMutApi},
};
use crate::rhai_binding::value_semantics::access_traits::ReadAccessProvider;

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

    fn insert_component(&self, component: Component) {
        let mut entity_world_mut = self.entity_world_mut.start_write();
        let (component_id, params) = component.0;
        let ctor = COMPONENT_CTOR_REGISTRY()
            .get(component_id.as_ref())
            .copied()
            .unwrap_or_else(|| panic!("Component ctor '{}' is not registered", component_id));
        ctor(&mut entity_world_mut, params);
        self.entity_world_mut.end_write(entity_world_mut);
    }

    fn insert_components(&self, components: Array) {
        let mut entity_world_mut = self.entity_world_mut.start_write();
        for (idx, value) in components.into_iter().enumerate() {
            let Some(component) = value.clone().try_cast::<Component>() else {
                panic!("EntityWorldMut::insert_components expects Component at index {idx}");
            };
            let (component_id, params) = component.0;
            let ctor = COMPONENT_CTOR_REGISTRY()
                .get(component_id.as_ref())
                .copied()
                .unwrap_or_else(|| panic!("Component ctor '{}' is not registered", component_id));
            ctor(&mut entity_world_mut, params);
        }
        self.entity_world_mut.end_write(entity_world_mut);
    }

    fn remove_component(&self, component_type_id: ImmutableString) {
        let mut entity_world_mut = self.entity_world_mut.start_write();
        let remove = COMPONENT_REMOVE_REGISTRY()
            .get(component_type_id.as_str())
            .copied()
            .unwrap_or_else(|| panic!("Component remover '{}' is not registered", component_type_id));
        remove(&mut entity_world_mut);
        self.entity_world_mut.end_write(entity_world_mut);
    }
}
