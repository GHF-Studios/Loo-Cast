use rhai::{Array, ImmutableString};

use crate::rhai_binding::runtime::ecs::component::bindings::types::Component;
use crate::bevy::prelude::Entity as BevyEntity;

pub trait EntityRefApi {
    fn id(&self) -> BevyEntity;
}

pub trait EntityMutApi {
    fn id(&self) -> BevyEntity;
}

pub trait EntityWorldMutApi {
    fn id(&self) -> BevyEntity;
    fn insert_component(&self, component: Component);
    fn insert_components(&self, components: Array);
    fn remove_component(&self, component_type_id: ImmutableString);
}
