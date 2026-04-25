use crate::bevy::prelude::Entity as BevyEntity;
use rhai::{Array, Dynamic, FnPtr, ImmutableString, NativeCallContext};

use crate::rhai_binding::runtime::ecs::component::bindings::types::Component;

pub trait CommandsApi {
    fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn spawn_components(&self, components: Array) -> BevyEntity;
    fn spawn_components_batch(&self, rows: Array) -> Array;
    fn entity(&self, entity: BevyEntity, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn despawn(&self, entity: BevyEntity);
}

pub trait EntityCommandsApi {
    fn commands(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn id(&self) -> BevyEntity;
    fn insert_component(&self, component: Component);
    fn insert_components(&self, components: Array);
    fn remove_component(&self, component_type_id: ImmutableString);
    fn despawn(&self);
}
