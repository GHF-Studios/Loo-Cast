use crate::{rhai_binding::{meta::{abstract_::trait_identity::{DynGetTraitName, DynGetTraitObjectName}, generic::abstract_primitive::ConstDynMetadata, monomorphized::trait_::{TraitMetadata, TraitObjectMetadata}}, path::trait_path::TraitPath}, utils::clone_lazy::CloneLazy};


pub const trait TraitConstDynMetadata: ConstDynMetadata + DynGetTraitName {
    fn id_path(&self) -> CloneLazy<TraitPath>;
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
