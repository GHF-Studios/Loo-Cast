use rhai::Dynamic;

use crate::rhai_binding::meta::abstract_::trait_identity::GetTraitId;
use crate::rhai_binding::value_semantics::ids::{DynamicTraitId, StaticTraitId, TypeId};
use crate::rhai_binding::value_semantics::statics::{TRAIT_OBJECT_VTABLE_REGISTRY, TYPE_REGISTRY};

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

#[derive(Clone)]
pub struct StaticTraitObject<T: GetTraitId> {
    pub value: Dynamic,
    pub trait_id: StaticTraitId<T>,
    pub instance_type_id: TypeId,
}

pub struct DynamicTraitObject {
    pub value: Dynamic,
    pub trait_id: DynamicTraitId,
    pub instance_type_id: TypeId,
}
impl DynamicTraitObject {
    fn assert_safety(&self) {
        let trait_id = &self.trait_id;
        let instance_type_id = &self.instance_type_id;
        let instance_type_info = TYPE_REGISTRY()
            .get(instance_type_id)
            .unwrap_or_else(|| panic!("Unknown type '{instance_type_id}'"));

        if !instance_type_info.implemented_trait_ids.contains(trait_id) {
            panic!("Instance type '{instance_type_id}' does not implement the trait '{trait_id}'")
        }
    }

    pub fn use_ref(self, method: &'static str, params: Dynamic) -> Dynamic {
        self.assert_safety();

        let trait_type_key = TraitTypeKey {
            instance_type_id: self.instance_type_id,
            trait_id: self.trait_id,
            method_name: method,
        };
        let vtable = TRAIT_OBJECT_VTABLE_REGISTRY().get(&trait_type_key).unwrap();

        (vtable.use_ref)(self.value, method, params)
    }

    pub fn use_mut(self, method: &'static str, params: Dynamic) -> Dynamic {
        self.assert_safety();

        let trait_type_key = TraitTypeKey {
            instance_type_id: self.instance_type_id,
            trait_id: self.trait_id,
            method_name: method,
        };
        let vtable = TRAIT_OBJECT_VTABLE_REGISTRY().get(&trait_type_key).unwrap();

        (vtable.use_mut)(self.value, method, params)
    }

    pub fn use_owned(self, method: &'static str, params: Dynamic) -> Dynamic {
        self.assert_safety();

        let trait_type_key = TraitTypeKey {
            instance_type_id: self.instance_type_id,
            trait_id: self.trait_id,
            method_name: method,
        };
        let vtable = TRAIT_OBJECT_VTABLE_REGISTRY().get(&trait_type_key).unwrap();

        (vtable.use_owned)(self.value, method, params)
    }
}
impl<T: GetTraitId> From<StaticTraitObject<T>> for DynamicTraitObject {
    fn from(value: StaticTraitObject<T>) -> Self {
        Self {
            value: value.value,
            trait_id: value.trait_id.id,
            instance_type_id: value.instance_type_id,
        }
    }
}
