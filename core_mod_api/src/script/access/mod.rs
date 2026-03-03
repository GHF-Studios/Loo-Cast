//! Compatibility shim: legacy `script::access` path.
//! Canonical home is `rhai_binding::value_semantics::scoped_access`.

pub use crate::rhai_binding::value_semantics::scoped_access::{
    ScopedAccess, ScopedAccessHandle, ScopedAccessHandleExt, ScopedAccessReadGuard, ScopedAccessWriteGuard,
};
