//! Compatibility shim: legacy `reflection::traits` path.
//! Canonical home is `rhai_binding::value_semantics::trait_object`.

pub use crate::rhai_binding::value_semantics::trait_object::{
    DynamicTraitObject, StaticTraitObject, TraitObjectUseMutFn, TraitObjectUseOwnedFn, TraitObjectUseRefFn, TraitTypeEntry,
    TraitTypeKey, TraitTypeVTables,
};
