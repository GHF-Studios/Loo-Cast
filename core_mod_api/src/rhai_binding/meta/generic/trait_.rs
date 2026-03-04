use rhai::ImmutableString;

use crate::{
    rhai_binding::{
        meta::{
            abstract_::trait_identity::{DynGetTraitName, DynGetTraitObjectName},
            generic::abstract_primitive::ConstDynMetadata,
            monomorphized::trait_::{TraitMetadata, TraitObjectMetadata},
        },
        path::trait_path::TraitPath,
    },
    utils::clone_lazy::CloneLazy,
};


pub const trait TraitConstDynMetadata: ConstDynMetadata + DynGetTraitName {
    fn id_path(&self) -> CloneLazy<TraitPath>;
    fn super_traits(&self) -> CloneLazy<Vec<TraitPath>>;
    fn is_dyn_safe(&self) -> CloneLazy<bool>;
    fn object_safety_notes(&self) -> CloneLazy<Vec<ImmutableString>>;
}
pub const trait TraitObjectConstDynMetadata: ConstDynMetadata + DynGetTraitObjectName {
    fn id_path(&self) -> CloneLazy<TraitPath>;
}

pub trait TraitDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: TraitConstDynMetadata>(&self, const_dyn_metadata: &T) -> TraitMetadata {
        TraitMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path().clone(),
            trait_name: const_dyn_metadata.trait_name(),
            super_traits: const_dyn_metadata.super_traits().clone(),
            is_dyn_safe: const_dyn_metadata.is_dyn_safe().clone(),
            object_safety_notes: const_dyn_metadata.object_safety_notes().clone(),
        }
    }
}
pub trait TraitObjectDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: TraitObjectConstDynMetadata>(&self, const_dyn_metadata: &T) -> TraitObjectMetadata {
        TraitObjectMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path().clone(),
            trait_object_name: const_dyn_metadata.trait_object_name(),
        }
    }
}
