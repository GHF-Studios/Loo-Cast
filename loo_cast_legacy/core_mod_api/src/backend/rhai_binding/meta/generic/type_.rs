use rhai::ImmutableString;

use crate::{
    rhai_binding::{
        meta::{generic::abstract_primitive::ConstDynMetadata, monomorphized::type_::TypeMetadata},
        path::{function_path::MethodFunctionPath, trait_path::TraitPath, type_path::TypePath},
        value_semantics::modes::TypeValueSemantics,
    },
    utils::{clone_closure::CloneClosure, clone_lazy::CloneLazy},
};

/// I think this is outdated, and the entire Type shit is not yet adapted to the new reflection paradigm,
/// AKA there is no metadata to describe the different possible variants of a Type yet
pub const trait TypeConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<TypePath>;
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)>;
    fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>>;
    fn value_semantics(&self) -> CloneLazy<TypeValueSemantics>;
    fn generic_definition_id(&self) -> CloneLazy<Option<ImmutableString>>;
    fn generic_param_names(&self) -> CloneLazy<Vec<ImmutableString>>;
    fn generic_param_trait_bounds(&self) -> CloneLazy<Vec<Vec<TraitPath>>>;
    fn generic_instantiation_args(&self) -> CloneLazy<Vec<Vec<TypePath>>>;
}
// pub const trait TypeOwnConstDynMetadata: TypeConstDynMetadata {}
// pub const trait TypeCloneConstDynMetadata: TypeConstDynMetadata {}
// pub const trait TypePersistentRefConstDynMetadata: TypeConstDynMetadata {}
// pub const trait TypePersistentMutConstDynMetadata: TypeConstDynMetadata {}
// /// Like a PersistentRef, but backs a rust-native immutable borrow *with* lifetimes, aka it implements runtime-checks against use-after-free's and aliasing issues;
// pub const trait TypeScopedRefConstDynMetadata: TypeConstDynMetadata {}
// pub const trait TypeScopedMutConstDynMetadata: TypeConstDynMetadata {}

pub trait TypeDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: TypeConstDynMetadata>(&self, const_dyn_metadata: &T) -> TypeMetadata {
        TypeMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path().clone(),
            registrator: const_dyn_metadata.clone().registrator(),

            method_functions: const_dyn_metadata.method_functions().clone(),
            value_semantics: const_dyn_metadata.value_semantics().clone(),
            generic_definition_id: const_dyn_metadata.generic_definition_id().clone(),
            generic_param_names: const_dyn_metadata.generic_param_names().clone(),
            generic_param_trait_bounds: const_dyn_metadata.generic_param_trait_bounds().clone(),
            generic_instantiation_args: const_dyn_metadata.generic_instantiation_args().clone(),
        }
    }
}
