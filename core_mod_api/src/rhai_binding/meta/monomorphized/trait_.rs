use rhai::ImmutableString;

use crate::{
    rhai_binding::{
        meta::{
            abstract_::trait_identity::{DynGetTraitName, DynGetTraitObjectName},
            generic::{
                abstract_primitive::ConstDynMetadata,
                trait_::{TraitConstDynMetadata, TraitObjectConstDynMetadata},
            },
        },
        path::trait_path::TraitPath,
    },
    utils::clone_lazy::CloneLazy,
};

#[derive(Clone)]
pub struct TraitMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<TraitPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    pub trait_name: &'static str,
    pub super_traits: CloneLazy<Vec<TraitPath>>,
    pub is_dyn_safe: CloneLazy<bool>,
    pub object_safety_notes: CloneLazy<Vec<ImmutableString>>,
}
impl ConstDynMetadata for TraitMetadata {
    fn raw_rust_module_path(&self) -> &'static str {
        self.raw_rust_module_path
    }
}
impl DynGetTraitName for TraitMetadata {
    fn trait_name(&self) -> &'static str {
        self.trait_name
    }
}
impl TraitConstDynMetadata for TraitMetadata {
    fn id_path(&self) -> CloneLazy<TraitPath> {
        self.id_path.clone()
    }
    fn super_traits(&self) -> CloneLazy<Vec<TraitPath>> {
        self.super_traits.clone()
    }
    fn is_dyn_safe(&self) -> CloneLazy<bool> {
        self.is_dyn_safe.clone()
    }
    fn object_safety_notes(&self) -> CloneLazy<Vec<ImmutableString>> {
        self.object_safety_notes.clone()
    }
}
#[derive(Clone)]
pub struct TraitObjectMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<TraitPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    pub trait_object_name: &'static str,
}
impl ConstDynMetadata for TraitObjectMetadata {
    fn raw_rust_module_path(&self) -> &'static str {
        self.raw_rust_module_path
    }
}
impl DynGetTraitObjectName for TraitObjectMetadata {
    fn trait_object_name(&self) -> &'static str {
        self.trait_object_name
    }
}
impl TraitObjectConstDynMetadata for TraitObjectMetadata {
    fn id_path(&self) -> CloneLazy<TraitPath> {
        self.id_path.clone()
    }
}
