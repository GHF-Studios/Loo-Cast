use crate::{
    rhai_binding::{
        meta::{
            generic::abstract_primitive::ConstDynMetadata,
            monomorphized::module::{TopLevelModuleMetadata, SubModuleMetadata, TypeBindingModuleMetadata}
        },
        path::{
            function_path::*, impl_path::*, module_path::*, trait_path::*, type_path::*
        }
    },
    utils::clone_lazy::CloneLazy
};


/// # Abstract
/// Should not be implemented itself, but via a superset-trait,
/// namely `TopLevelModuleConstDynMetadata`, `SubModuleConstDynMetadata` and/or `TypeBindingModuleConstDynMetadata`.
pub const trait NativeModuleConstDynMetadata: ConstDynMetadata {
    fn traits(&self) -> CloneLazy<Vec<TraitPath>>;
    fn types(&self) -> CloneLazy<Vec<TypePath>>;
    fn inherent_impls(&self) -> CloneLazy<Vec<InherentImplPath>>;
    fn trait_impls(&self) -> CloneLazy<Vec<TraitImplPath>>;
}
pub const trait TopLevelModuleConstDynMetadata: NativeModuleConstDynMetadata {
    fn id_path(&self) -> CloneLazy<TopLevelModulePath>;
    fn sub_modules(&self) -> CloneLazy<Vec<SubModulePath>>;
    fn type_binding_modules(&self) -> CloneLazy<Vec<TypeBindingModulePath>>;
    fn module_associated_functions(&self) -> CloneLazy<Vec<ModuleAssociatedFunctionPath>>;
}
pub const trait SubModuleConstDynMetadata: NativeModuleConstDynMetadata {
    fn id_path(&self) -> CloneLazy<SubModulePath>;
    fn sub_modules(&self) -> CloneLazy<Vec<SubModulePath>>;
    fn type_binding_modules(&self) -> CloneLazy<Vec<TypeBindingModulePath>>;
    fn module_associated_functions(&self) -> CloneLazy<Vec<ModuleAssociatedFunctionPath>>;
}
pub const trait TypeBindingModuleConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<TypeBindingModulePath>;
    fn item_associated_functions(&self) -> CloneLazy<Vec<ItemAssociatedFunctionPath>>;
    fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>>;
    fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>>;
}

pub trait TopLevelModuleDynamicTypedMetadata: TopLevelModuleConstDynMetadata {
    fn from_comptime_to_runtime<T: TopLevelModuleConstDynMetadata>(&self, const_dyn_metadata: &T) -> TopLevelModuleMetadata {
        TopLevelModuleMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path().clone(),

            traits: const_dyn_metadata.traits().clone(),
            types: const_dyn_metadata.types().clone(),
            inherent_impls: const_dyn_metadata.inherent_impls().clone(),
            trait_impls: const_dyn_metadata.trait_impls().clone(),

            sub_modules: const_dyn_metadata.sub_modules().clone(),
            type_binding_modules: const_dyn_metadata.type_binding_modules().clone(),
            module_associated_functions: const_dyn_metadata.module_associated_functions().clone(),
        }
    }
}
pub trait SubModuleDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: SubModuleConstDynMetadata>(&self, const_dyn_metadata: &T) -> SubModuleMetadata {
        SubModuleMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path(),

            traits: const_dyn_metadata.traits().clone(),
            types: const_dyn_metadata.types().clone(),
            inherent_impls: const_dyn_metadata.inherent_impls().clone(),
            trait_impls: const_dyn_metadata.trait_impls().clone(),

            sub_modules: const_dyn_metadata.sub_modules().clone(),
            type_binding_modules: const_dyn_metadata.type_binding_modules().clone(),
            module_associated_functions: const_dyn_metadata.module_associated_functions().clone(),
        }
    }
}
pub trait TypeBindingModuleDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: TypeBindingModuleConstDynMetadata>(&self, const_dyn_metadata: &T) -> TypeBindingModuleMetadata {
        TypeBindingModuleMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path().clone(),

            item_associated_functions: const_dyn_metadata.item_associated_functions().clone(),
            constructor_functions: const_dyn_metadata.constructor_functions().clone(),
            method_functions: const_dyn_metadata.method_functions().clone(),
        }
    }
}