use rhai::{Array, Dynamic, FnPtr, ImmutableString, NativeCallContext};

use crate::bevy::prelude::Entity as BevyEntity;
use crate::rhai_binding::runtime::ecs::bundle::internals::trait_objects::BundleTraitObject;
use crate::rhai_binding::runtime::ecs::system::query::bindings::types::{Query, QueryData, QueryFilter};
use crate::rhai_binding::runtime::std::iter::bindings::types::StringIter;

pub trait WorldApi {
    fn commands(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn flush(&self);
    fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn spawn_single(&self, bundle: BundleTraitObject, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn spawn_batch(&self, bundles: Array) -> Array;
    fn spawn_components(&self, components: Array) -> BevyEntity;
    fn entity(&self, entity: BevyEntity, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn entity_mut(&self, entity: BevyEntity, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn get_entity(&self, entity: BevyEntity, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn get_entity_mut(&self, entity: BevyEntity, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn despawn(&self, entity: BevyEntity) -> bool;
    fn entities(&self) -> Array;
    fn query(&self, data: QueryData) -> Query;
    fn query_filtered(&self, data: QueryData, filter: QueryFilter) -> Query;
    fn single(&self, data: QueryData) -> Dynamic;
    fn single_filtered(&self, data: QueryData, filter: QueryFilter) -> Dynamic;
    fn exists(&self, data: QueryData) -> bool;
    fn exists_filtered(&self, data: QueryData, filter: QueryFilter) -> bool;
    fn insert_resource(&self, resource_type_id: ImmutableString, payload: ImmutableString);
    fn init_resource(&self, resource_type_id: ImmutableString);
    fn remove_resource(&self, resource_type_id: ImmutableString) -> Dynamic;
    fn get_resource(&self, resource_type_id: ImmutableString) -> Dynamic;
    fn get_resource_mut(&self, resource_type_id: ImmutableString) -> Dynamic;
    fn has_resource(&self, resource_type_id: ImmutableString) -> bool;
    fn known_schedules(&self) -> Array;
    fn run_schedule(&self, schedule_name: ImmutableString) -> bool;
    fn write_message(&self, message_type_id: ImmutableString, payload: ImmutableString);
    fn drain_messages(&self, message_type_id: ImmutableString) -> StringIter;
    fn write_probe_message(&self, payload: ImmutableString);
    fn drain_probe_messages(&self) -> StringIter;
}
// Binding coverage backlog and World API hierarchy live in:
// docs/RhaiBindingRoadmap.md
