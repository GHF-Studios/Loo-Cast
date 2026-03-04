use rhai::{Dynamic, FnPtr, ImmutableString, NativeCallContext};

use crate::rhai_binding::runtime::ecs::bundle::internals::trait_objects::BundleTraitObject;
use crate::rhai_binding::runtime::ecs::system::query::bindings::types::{Query, QueryData, QueryFilter};
use crate::rhai_binding::runtime::std::iter::bindings::types::StringIter;

pub trait WorldApi {
    fn commands(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn flush(&self);
    fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn spawn_single(&self, bundle: BundleTraitObject, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn query(&self, data: QueryData) -> Query;
    fn query_filtered(&self, data: QueryData, filter: QueryFilter) -> Query;
    fn write_probe_message(&self, payload: ImmutableString);
    fn drain_probe_messages(&self) -> StringIter;
    // fn spawn_batch(&self, bundles: Array, ctx: NativeCallContext, callback: FnPtr) -> Array;
}
// Binding coverage backlog and World API hierarchy live in:
// docs/RhaiBindingRoadmap.md
