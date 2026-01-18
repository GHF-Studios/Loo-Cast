use bevy::ecs::query::{QueryData, QueryFilter};
use bevy::ecs::system::Commands;
use bevy::prelude::{World, Query};
use std::sync::{Arc, RwLock};

use crate::script::core::internals::types::{ScopedAccess, ScopedAccessHandle};

pub trait ScopedAccessProvider0<T> {
    /// Create a runtime-scoped access handle.
    fn start_access(&mut self) -> ScopedAccessHandle<T>;

    /// Reclaim the value after Rhai is done.
    /// Must fail hard if the handle was misused.
    fn end_access(&mut self, handle: ScopedAccessHandle<T>) -> T;
}

/// # Safety
/// The implementor must ensure that Raw is the same fundamental type as Scoped, but with all lifetimes transmuted to 'static.
pub unsafe trait ScopedAccessProvider1<TRaw, TScoped>
where
    for<'a> TRaw: 'a,
    TScoped: 'static,
{
    /// Create a runtime-scoped access handle.
    /// Internally may transmute lifetimes to 'static.
    /// # Safety
    /// Refer to the trait-level safety doc.
    unsafe fn start_access(&mut self) -> ScopedAccessHandle<TScoped>;

    /// Reclaim the value after Rhai is done.
    /// Must fail hard if the handle was misused.
    /// # Safety
    /// Refer to the trait-level safety doc.
    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<TScoped>) -> TRaw;
}

/// # Safety
/// The implementor must ensure that Raw is the same fundamental type as Scoped, but with all lifetimes transmuted to 'static.
pub unsafe trait ScopedAccessProvider2<TRaw, TScoped>
where
    for<'a, 'b> TRaw: 'a + 'b,
    TScoped: 'static,
{
    /// Create a runtime-scoped access handle.
    /// Internally may transmute lifetimes to 'static.
    /// # Safety
    /// Refer to the trait-level safety doc.
    unsafe fn start_access(&mut self) -> ScopedAccessHandle<TScoped>;

    /// Reclaim the value after Rhai is done.
    /// Must fail hard if the handle was misused.
    /// # Safety
    /// Refer to the trait-level safety doc.
    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<TScoped>) -> TRaw;
}

unsafe impl<'w, 's> ScopedAccessProvider2<Commands<'w, 's>, Commands<'static, 'static>> for World
{
    unsafe fn start_access(&mut self) -> ScopedAccessHandle<Commands<'static, 'static>> {
        let cmds: Commands<'_, '_> = self.commands();

        // erase lifetime
        let cmds_static: Commands<'static, 'static> =
            std::mem::transmute::<Commands<'_, '_>, Commands<'static, 'static>>(cmds);

        Arc::new(RwLock::new(ScopedAccess::new(cmds_static)))
    }

    unsafe fn end_access(
        &mut self,
        handle: ScopedAccessHandle<Commands<'static, 'static>>,
    ) -> Commands<'w, 's> {
        let mut scoped = Arc::into_inner(handle)
            .expect("Commands handle leaked or cloned")
            .into_inner()
            .expect("RwLock poisoned");

        let cmds_static = scoped
            .invalidate()
            .expect("Commands handle was already invalidated");

        // restore lifetime
        std::mem::transmute::<Commands<'static, 'static>, Commands<'_, '_>>(cmds_static)
    }
}

unsafe impl<'w, 's, D: QueryData, F: QueryFilter> ScopedAccessProvider2<Query<'w, 's, D, F>, Query<'static, 'static, D, F>> for World
{
    unsafe fn start_access(&mut self) -> ScopedAccessHandle<Query<'static, 'static, D, F>> {
        let query: Query<'_, '_ , D, F> = todo!(); // self.query::<D, F>();

        // erase lifetime
        let query_static: Query<'static, 'static, D, F> =
            std::mem::transmute::<Query<'_, '_ , D, F>, Query<'static, 'static, D, F>>(query);

        Arc::new(RwLock::new(ScopedAccess::new(query_static)))
    }

    unsafe fn end_access(
        &mut self,
        handle: ScopedAccessHandle<Query<'static, 'static, D, F>>,
    ) -> Query<'w, 's, D, F> {
        let mut scoped = Arc::into_inner(handle)
            .expect("Query handle leaked or cloned")
            .into_inner()
            .expect("RwLock poisoned");

        let query_static = scoped
            .invalidate()
            .expect("Query handle was already invalidated");

        // restore lifetime
        std::mem::transmute::<Query<'static, 'static, D, F>, Query<'_, '_ , D, F>>(query_static)
    }
}