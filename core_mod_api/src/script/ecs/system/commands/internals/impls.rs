use bevy::ecs::system::Commands;
use bevy::ecs::system::EntityCommands;
use std::sync::{Arc, RwLock};

use crate::script::core::internals::traits::ScopedAccessProvider;
use crate::script::core::internals::types::{ScopedAccess, ScopedAccessHandle};

unsafe impl ScopedAccessProvider<EntityCommands<'static>> for Commands<'static, 'static> {
    unsafe fn start_access(&mut self) -> ScopedAccessHandle<EntityCommands<'static>> {
        let entity_commands = self.spawn_empty(); // TODO: This is shit, fix it by properly implementing a vtable with support for parameters for the ScopedAccessProvider trait

        // erase lifetime
        let entity_commands_static = std::mem::transmute::<EntityCommands<'_>, EntityCommands<'static>>(entity_commands);

        Arc::new(RwLock::new(ScopedAccess::new(entity_commands_static)))
    }

    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<EntityCommands<'static>>) {
        let mut entity_commands_raw_scoped = Arc::into_inner(handle)
            .expect("EntityCommands handle leaked or cloned")
            .into_inner()
            .expect("RwLock poisoned");
        
        let returned_entity_commands_static = entity_commands_raw_scoped
            .invalidate()
            .expect("EntityCommands handle was already invalidated");

        // restore lifetime and drop
        let _returned_entity_commands = std::mem::transmute::<EntityCommands<'static>, EntityCommands<'_>>(returned_entity_commands_static);
    }
}

unsafe impl ScopedAccessProvider<Commands<'static, 'static>> for EntityCommands<'static> {
    unsafe fn start_access(&mut self) -> ScopedAccessHandle<Commands<'static, 'static>> {
        let commands = self.commands();
        
        // erase lifetime
        let commands_static = std::mem::transmute::<Commands<'_, '_>, Commands<'static, 'static>>(commands);

        Arc::new(RwLock::new(ScopedAccess::new(commands_static)))
    }

    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<Commands<'static, 'static>>) {
        let mut commands_raw_scoped = Arc::into_inner(handle)
            .expect("Commands handle leaked or cloned")
            .into_inner()
            .expect("RwLock poisoned");
        
        let returned_commands_static = commands_raw_scoped
            .invalidate()
            .expect("Commands handle was already invalidated");

        // restore lifetime and drop
        let _returned_commands = std::mem::transmute::<Commands<'static, 'static>, Commands<'_, '_>>(returned_commands_static);
    }
}