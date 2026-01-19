use bevy::ecs::query::{QueryData, QueryFilter};
use bevy::ecs::system::{Commands, EntityCommands};
use bevy::prelude::{World, Query};
use std::sync::{Arc, RwLock};

use crate::script::core::internals::traits::ScopedAccessProvider;
use crate::script::core::internals::types::{ScopedAccess, ScopedAccessHandle};

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

        // restore lifetime and drop
        let _cmds = std::mem::transmute::<Commands<'static, 'static>, Commands<'_, '_>>(cmds_static);
    }
}

unsafe impl ScopedAccessProvider<EntityCommands<'static>> for World {
    unsafe fn start_access(&mut self) -> ScopedAccessHandle<EntityCommands<'static>> {
        let mut entity_cmds = self.entity_commands();

        // erase lifetime
        let entity_cmds_static = std::mem::transmute::<EntityCommands<'_>, EntityCommands<'static>>(entity_cmds);

        Arc::new(RwLock::new(ScopedAccess::new(entity_cmds_static)))
    }

    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<EntityCommands<'static>>) {
        let mut scoped = Arc::into_inner(handle)
            .expect("EntityCommands handle leaked or cloned")
            .into_inner()
            .expect("RwLock poisoned");
        
        let entity_cmds_static = scoped
            .invalidate()
            .expect("EntityCommands handle was already invalidated");

        // restore lifetime and drop
        let _entity_cmds = std::mem::transmute::<EntityCommands<'static>, EntityCommands<'_>>(entity_cmds_static);
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

        // restore lifetime and drop
        let _query = std::mem::transmute::<Query<'static, 'static, D, F>, Query<'_, '_ , D, F>>(query_static);
    }
}