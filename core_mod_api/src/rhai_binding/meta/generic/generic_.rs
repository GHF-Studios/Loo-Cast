use rhai::ImmutableString;

use crate::{
    rhai_binding::{
        meta::{
            generic::abstract_primitive::ConstDynMetadata,
            monomorphized::generic_::{GenericDefinitionMetadata, GenericInstantiationMetadata},
        },
        path::{trait_path::TraitPath, type_path::TypePath},
        value_semantics::modes::TypeValueSemantics,
    },
    utils::clone_lazy::CloneLazy,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GenericOwnerKind {
    Type,
    Function,
    Method,
}

pub const trait GenericDefinitionConstDynMetadata: ConstDynMetadata {
    fn id(&self) -> CloneLazy<ImmutableString>;
    fn owner_kind(&self) -> CloneLazy<GenericOwnerKind>;
    fn params(&self) -> CloneLazy<Vec<ImmutableString>>;
    fn param_trait_bounds(&self) -> CloneLazy<Vec<Vec<TraitPath>>>;
    fn notes(&self) -> CloneLazy<Vec<ImmutableString>>;
}

pub trait GenericDefinitionDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: GenericDefinitionConstDynMetadata>(
        &self,
        const_dyn_metadata: &T,
    ) -> GenericDefinitionMetadata {
        GenericDefinitionMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id: const_dyn_metadata.id().clone(),
            owner_kind: const_dyn_metadata.owner_kind().clone(),
            params: const_dyn_metadata.params().clone(),
            param_trait_bounds: const_dyn_metadata.param_trait_bounds().clone(),
            notes: const_dyn_metadata.notes().clone(),
        }
    }
}

pub const trait GenericInstantiationConstDynMetadata: ConstDynMetadata {
    fn id(&self) -> CloneLazy<ImmutableString>;
    fn generic_id(&self) -> CloneLazy<ImmutableString>;
    fn type_arguments(&self) -> CloneLazy<Vec<TypePath>>;
    fn concrete_item_path(&self) -> CloneLazy<ImmutableString>;
    fn value_semantics(&self) -> CloneLazy<Option<TypeValueSemantics>>;
}

pub trait GenericInstantiationDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: GenericInstantiationConstDynMetadata>(
        &self,
        const_dyn_metadata: &T,
    ) -> GenericInstantiationMetadata {
        GenericInstantiationMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id: const_dyn_metadata.id().clone(),
            generic_id: const_dyn_metadata.generic_id().clone(),
            type_arguments: const_dyn_metadata.type_arguments().clone(),
            concrete_item_path: const_dyn_metadata.concrete_item_path().clone(),
            value_semantics: const_dyn_metadata.value_semantics().clone(),
        }
    }
}
