use rhai::{Dynamic, ImmutableString};

use crate::reflection::ids::{TypeId, StaticTraitId, DynamicTraitId};
use crate::reflection::internals::statics::{TRAIT_OBJECT_VTABLE_REGISTRY, TYPE_REGISTRY};
use crate::reflection::internals::traits::Trait;
use crate::script::access::{ScopedAccessHandle, ScopedAccessHandleExt};

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

pub struct StaticTraitObject<T: Trait> {
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
        let instance_type_info = TYPE_REGISTRY().get(instance_type_id).unwrap_or_else(|| panic!("Unknown type '{instance_type_id}'"));
        
        if !instance_type_info.implemented_trait_ids.contains(trait_id) {
            panic!("Instance type '{instance_type_id}' does not implement the trait '{trait_id}'")
        }
    }

    pub fn use_ref(self, method: &'static str, params: Dynamic) -> Dynamic {
        self.assert_safety();

        // let instance_handle = self.0.0.read_lock::<ScopedAccessHandle<I>>().unwrap();
        // let instance_guard = ScopedAccessHandleExt::as_ref(&*instance_handle);
        // let instance_ref = &*instance_guard;

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

        // let mut instance_handle = self.0.0.write_lock::<ScopedAccessHandle<I>>().unwrap();
        // let instance_guard = ScopedAccessHandleExt::as_mut(&mut *instance_handle);
        // let instance_mut = &mut *instance_guard;

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

        // let instance = self.0.0.cast::<ScopedAccessHandle<I>>().into_inner();

        let trait_type_key = TraitTypeKey {
            instance_type_id: self.instance_type_id,
            trait_id: self.trait_id,
            method_name: method,
        };
        let vtable = TRAIT_OBJECT_VTABLE_REGISTRY().get(&trait_type_key).unwrap();

        (vtable.use_owned)(self.value, method, params)
    }
}
impl<T: Trait> From<StaticTraitObject<T>> for DynamicTraitObject {
    fn from(value: StaticTraitObject<T>) -> Self {
        Self {
            value: value.value,
            trait_id: value.trait_id.id,
            instance_type_id: value.instance_type_id,
        }
    }
}
