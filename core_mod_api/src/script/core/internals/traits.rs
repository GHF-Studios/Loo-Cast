#![allow(clippy::missing_safety_doc)]

use crate::script::core::internals::types::ScopedAccessHandle;

pub(crate) unsafe trait ScopedAccessProvider<TScoped> {
    unsafe fn start_access(&mut self) -> ScopedAccessHandle<TScoped>;
    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<TScoped>);
}
