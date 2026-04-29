//! Concrete runtime metadata for generic definitions/instantiations.
//!
//! These structs are inserted into the runtime binding graph and are consumed
//! by tooling/logging/validation. They are intentionally canonical (full-path
//! IDs, explicit bounds, explicit type argument lists).

use rhai::ImmutableString;

use crate::{
    rhai_binding::{
        meta::generic::{
            abstract_primitive::ConstDynMetadata,
            generic_::{GenericDefinitionConstDynMetadata, GenericInstantiationConstDynMetadata, GenericOwnerKind},
        },
        path::{trait_path::TraitPath, type_path::TypePath},
        value_semantics::modes::TypeValueSemantics,
    },
    utils::clone_lazy::CloneLazy,
};

#[derive(Clone)]
pub struct GenericDefinitionMetadata {
    pub id: CloneLazy<ImmutableString>,
    pub owner_kind: CloneLazy<GenericOwnerKind>,
    pub params: CloneLazy<Vec<ImmutableString>>,
    pub param_trait_bounds: CloneLazy<Vec<Vec<TraitPath>>>,
    pub notes: CloneLazy<Vec<ImmutableString>>,
    pub raw_rust_module_path: &'static str,
}
impl ConstDynMetadata for GenericDefinitionMetadata {
    fn raw_rust_module_path(&self) -> &'static str {
        self.raw_rust_module_path
    }
}
impl GenericDefinitionConstDynMetadata for GenericDefinitionMetadata {
    fn id(&self) -> CloneLazy<ImmutableString> {
        self.id.clone()
    }
    fn owner_kind(&self) -> CloneLazy<GenericOwnerKind> {
        self.owner_kind.clone()
    }
    fn params(&self) -> CloneLazy<Vec<ImmutableString>> {
        self.params.clone()
    }
    fn param_trait_bounds(&self) -> CloneLazy<Vec<Vec<TraitPath>>> {
        self.param_trait_bounds.clone()
    }
    fn notes(&self) -> CloneLazy<Vec<ImmutableString>> {
        self.notes.clone()
    }
}

#[derive(Clone)]
pub struct GenericInstantiationMetadata {
    pub id: CloneLazy<ImmutableString>,
    pub generic_id: CloneLazy<ImmutableString>,
    pub type_arguments: CloneLazy<Vec<TypePath>>,
    pub concrete_item_path: CloneLazy<ImmutableString>,
    pub value_semantics: CloneLazy<Option<TypeValueSemantics>>,
    pub raw_rust_module_path: &'static str,
}
impl ConstDynMetadata for GenericInstantiationMetadata {
    fn raw_rust_module_path(&self) -> &'static str {
        self.raw_rust_module_path
    }
}
impl GenericInstantiationConstDynMetadata for GenericInstantiationMetadata {
    fn id(&self) -> CloneLazy<ImmutableString> {
        self.id.clone()
    }
    fn generic_id(&self) -> CloneLazy<ImmutableString> {
        self.generic_id.clone()
    }
    fn type_arguments(&self) -> CloneLazy<Vec<TypePath>> {
        self.type_arguments.clone()
    }
    fn concrete_item_path(&self) -> CloneLazy<ImmutableString> {
        self.concrete_item_path.clone()
    }
    fn value_semantics(&self) -> CloneLazy<Option<TypeValueSemantics>> {
        self.value_semantics.clone()
    }
}
