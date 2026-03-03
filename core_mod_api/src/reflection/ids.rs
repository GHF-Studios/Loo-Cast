//! Compatibility shim: legacy `reflection::ids` path.
//! Canonical home is `rhai_binding::value_semantics::ids`.

pub use crate::rhai_binding::value_semantics::ids::{
    DynamicTraitId, ModuleId, StaticTraitId, TypeId,
};
