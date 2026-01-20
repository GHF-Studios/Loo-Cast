#![allow(clippy::missing_safety_doc)]

use std::any::Any;

use crate::script::core::internals::types::ScopedAccessHandle;

pub(crate) trait AccessProvider<TScoped> {
    fn access(&mut self, method: &str, args: Box<dyn Any>) -> TScoped;
}

pub(crate) unsafe trait ScopedAccessProvider<TScoped> {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> ScopedAccessHandle<TScoped>;
    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<TScoped>);
}
