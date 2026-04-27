use rhai::{Dynamic, ImmutableString};

use crate::rhai_binding::meta::abstract_::trait_identity::{GetTraitId, StaticTraitObject};

use super::ids::{DynamicTraitId, StaticTraitId, TypeId};

pub type TraitObjectUseRefFn = fn(Dynamic, &str, Dynamic) -> Dynamic;
pub type TraitObjectUseMutFn = fn(Dynamic, &str, Dynamic) -> Dynamic;
pub type TraitObjectUseOwnedFn = fn(Dynamic, &str, Dynamic) -> Dynamic;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TraitTypeKey {
    pub instance_type_id: TypeId,
    pub trait_id: DynamicTraitId,
    pub method_name: &'static str,
}

#[derive(Clone)]
pub struct TraitTypeVTables {
    pub use_ref: TraitObjectUseRefFn,
    pub use_mut: TraitObjectUseMutFn,
    pub use_owned: TraitObjectUseOwnedFn,
}

inventory::collect!(TraitTypeEntry);
#[derive(Clone)]
pub struct TraitTypeEntry {
    pub key: TraitTypeKey,
    pub value: TraitTypeVTables,
}

pub struct DynamicTraitObject {
    pub value: Dynamic,
    pub trait_id: DynamicTraitId,
    pub instance_type_id: TypeId,
}

impl<T: GetTraitId> From<StaticTraitObject<T>> for DynamicTraitObject {
    fn from(value: StaticTraitObject<T>) -> Self {
        Self {
            value: value.value,
            trait_id: StaticTraitId::<T>::new().id,
            instance_type_id: TypeId::from(ImmutableString::from(value.instance_type_id)),
        }
    }
}
