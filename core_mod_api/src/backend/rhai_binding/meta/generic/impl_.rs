use crate::{
    rhai_binding::{
        meta::{
            generic::abstract_primitive::ConstDynMetadata,
            monomorphized::impl_::{InherentImplMetadata, TraitImplMetadata},
        },
        path::{
            function_path::{ConstructorFunctionPath, ItemAssociatedFunctionPath, MethodFunctionPath},
            impl_path::{InherentImplPath, TraitImplPath},
        },
    },
    utils::clone_lazy::CloneLazy,
};

pub const trait InherentImplConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<InherentImplPath>;
    fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>>;
    fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>>;
    fn item_associated_functions(&self) -> CloneLazy<Vec<ItemAssociatedFunctionPath>>;
}
pub const trait TraitImplConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<TraitImplPath>;
    fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>>;
    fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>>;
    fn item_associated_functions(&self) -> CloneLazy<Vec<ItemAssociatedFunctionPath>>;
}

pub trait InherentImplDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: InherentImplConstDynMetadata>(&self, const_dyn_metadata: &T) -> InherentImplMetadata {
        InherentImplMetadata {
            id_path: const_dyn_metadata.id_path(),
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),

            constructor_functions: const_dyn_metadata.constructor_functions().clone(),
            method_functions: const_dyn_metadata.method_functions().clone(),
            item_associated_functions: const_dyn_metadata.item_associated_functions().clone(),
        }
    }
}
pub trait TraitImplDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: TraitImplConstDynMetadata>(&self, const_dyn_metadata: &T) -> TraitImplMetadata {
        TraitImplMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path(),

            constructor_functions: const_dyn_metadata.constructor_functions().clone(),
            method_functions: const_dyn_metadata.method_functions().clone(),
            item_associated_functions: const_dyn_metadata.item_associated_functions().clone(),
        }
    }
}
