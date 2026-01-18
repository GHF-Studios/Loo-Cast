use bevy::ecs::query::{QueryData, QueryFilter};
use bevy::ecs::system::Commands;
use bevy::prelude::{World, Query};
use std::sync::{Arc, RwLock};

use crate::script::core::internals::types::{ScopedAccess, ScopedAccessHandle};

#[allow(clippy::missing_safety_doc)]
pub unsafe trait ScopedAccessProvider<TScoped> {
    unsafe fn start_access(&mut self) -> ScopedAccessHandle<TScoped>;
    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<TScoped>);
}

unsafe impl ScopedAccessProvider<Commands<'static, 'static>> for World {
    unsafe fn start_access(&mut self) -> ScopedAccessHandle<Commands<'static, 'static>> {
        let cmds = self.commands();
        
        // erase lifetime
        let cmds_static = std::mem::transmute::<Commands<'_, '_>, Commands<'static, 'static>>(cmds);

        Arc::new(RwLock::new(ScopedAccess::new(cmds_static)))
    }

    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<Commands<'static, 'static>>) {
        let mut scoped = Arc::into_inner(handle)
            .expect("Commands handle leaked or cloned")
            .into_inner()
            .expect("RwLock poisoned");
        
        let cmds_static = scoped
            .invalidate()
            .expect("Commands handle was already invalidated");

        // restore lifetime and drop T
        let _ = std::mem::transmute::<Commands<'static, 'static>, Commands<'_, '_>>(cmds_static);
    }
}

unsafe impl<D: QueryData, F: QueryFilter> ScopedAccessProvider<Query<'static, 'static, D, F>> for World {
    unsafe fn start_access(&mut self) -> ScopedAccessHandle<Query<'static, 'static, D, F>> {
        let mut query_state = self.query_filtered::<D, F>();
        let query = query_state.query_mut(self);

        // erase lifetime
        let query_static = std::mem::transmute::<Query<'_, '_ , D, F>, Query<'static, 'static, D, F>>(query);

        Arc::new(RwLock::new(ScopedAccess::new(query_static)))
    }

    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<Query<'static, 'static, D, F>>) {
        let mut scoped = Arc::into_inner(handle)
            .expect("Query handle leaked or cloned")
            .into_inner()
            .expect("RwLock poisoned");

        let query_static = scoped
            .invalidate()
            .expect("Query handle was already invalidated");

        // restore lifetime and drop T
        let _ = std::mem::transmute::<Query<'static, 'static, D, F>, Query<'_, '_ , D, F>>(query_static);
    }
}