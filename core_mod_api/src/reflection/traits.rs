//! Reflection-facing trait-object aliases.
//! Canonical definitions live in `rhai_binding::value_semantics::trait_object`.

pub use crate::rhai_binding::value_semantics::trait_object::{
    DynamicTraitObject, StaticTraitObject, TraitObjectUseMutFn, TraitObjectUseOwnedFn, TraitObjectUseRefFn, TraitTypeEntry,
    TraitTypeKey, TraitTypeVTables,
};
