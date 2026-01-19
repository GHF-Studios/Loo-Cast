use bevy::prelude::Entity as BevyEntity;
use bevy::ecs::system::Commands;
use bevy::ecs::system::EntityCommands;
use std::any::Any;
use std::sync::{Arc, RwLock};

use crate::script::core::internals::traits::ScopedAccessProvider;
use crate::script::core::internals::types::{ScopedAccess, ScopedAccessHandle};

unsafe impl ScopedAccessProvider<EntityCommands<'static>> for Commands<'static, 'static> {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> ScopedAccessHandle<EntityCommands<'static>> {
        let entity_commands = match method {
            "spawn_empty" => {
                if !args.is::<()>() {
                    panic!("Unsupported arguments for method '{}' in ScopedAccessProvider<EntityCommands> for Commands", method);
                }

                self.spawn_empty()
            },
            _ => panic!("Unsupported method '{}' in ScopedAccessProvider<EntityCommands> for Commands", method),
        };

        // Erase lifetime(s)
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

        // Restore lifetime(s)
        let _returned_entity_commands = std::mem::transmute::<EntityCommands<'static>, EntityCommands<'_>>(returned_entity_commands_static);
    }
}

unsafe impl ScopedAccessProvider<Commands<'static, 'static>> for EntityCommands<'static> {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> ScopedAccessHandle<Commands<'static, 'static>> {
        if method != "commands" {
            panic!("Unsupported method '{}' in ScopedAccessProvider<Commands> for EntityCommands", method);
        }
        if !args.is::<()>() {
            panic!("Unsupported arguments for method '{}' in ScopedAccessProvider<Commands> for EntityCommands", method);
        }

        let commands = self.commands();
        
        // Erase lifetime(s)
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

        // Restore lifetime(s)
        let _returned_commands = std::mem::transmute::<Commands<'static, 'static>, Commands<'_, '_>>(returned_commands_static);
    }
}

unsafe impl ScopedAccessProvider<BevyEntity> for EntityCommands<'static> {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> ScopedAccessHandle<BevyEntity> {
        if method != "id" {
            panic!("Unsupported method '{}' in ScopedAccessProvider<BevyEntity> for EntityCommands", method);
        }
        if !args.is::<()>() {
            panic!("Unsupported arguments for method '{}' in ScopedAccessProvider<BevyEntity> for EntityCommands", method);
        }

        let id = self.id();

        Arc::new(RwLock::new(ScopedAccess::new(id)))
    }

    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<BevyEntity>) {
        let mut id_raw_scoped = Arc::into_inner(handle)
            .expect("BevyEntity handle leaked or cloned")
            .into_inner()
            .expect("RwLock poisoned");
        
        let _returned_id = id_raw_scoped
            .invalidate()
            .expect("BevyEntity handle was already invalidated");
    }
}