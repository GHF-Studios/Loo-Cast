use crate::bevy::prelude::Entity as BevyEntity;
use crate::bevy::ecs::system::Commands;
use crate::bevy::ecs::system::EntityCommands;
use crate::rhai_binding::value_semantics::access_traits::AccessCellProvider;
use crate::rhai_binding::value_semantics::access_cell::{AccessCell, Scoped};
use std::any::Any;


unsafe impl AccessCellProvider<EntityCommands<'static>> for Commands<'static, 'static> {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> AccessCell<Scoped, EntityCommands<'static>> {
        let entity_commands = match method {
            "spawn_empty" => {
                if !args.is::<()>() {
                    panic!("Unsupported arguments for method '{}' in AccessCellProvider<EntityCommands> for Commands", method);
                }

                self.spawn_empty()
            },
            _ => panic!("Unsupported method '{}' in AccessCellProvider<EntityCommands> for Commands", method),
        };

        // Erase lifetime(s)
        let entity_commands_static = unsafe {
            std::mem::transmute::<EntityCommands<'_>, EntityCommands<'static>>(entity_commands)
        };

        AccessCell::new(entity_commands_static)
    }

    unsafe fn end_access(&mut self, handle: AccessCell<Scoped, EntityCommands<'static>>) {
        let returned_entity_commands_static = handle.take();

        // Restore lifetime(s)
        let _returned_entity_commands = unsafe {
            std::mem::transmute::<EntityCommands<'static>, EntityCommands<'_>>(returned_entity_commands_static)
        };
    }
}

unsafe impl AccessCellProvider<Commands<'static, 'static>> for EntityCommands<'static> {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> AccessCell<Scoped, Commands<'static, 'static>> {
        if method != "commands" {
            panic!("Unsupported method '{}' in AccessCellProvider<Commands> for EntityCommands", method);
        }
        if !args.is::<()>() {
            panic!("Unsupported arguments for method '{}' in AccessCellProvider<Commands> for EntityCommands", method);
        }

        let commands = self.commands();
        
        // Erase lifetime(s)
        let commands_static = unsafe {
            std::mem::transmute::<Commands<'_, '_>, Commands<'static, 'static>>(commands)
        };

        AccessCell::new(commands_static)
    }

    unsafe fn end_access(&mut self, handle: AccessCell<Scoped, Commands<'static, 'static>>) {
        let returned_commands_static = handle.take();

        // Restore lifetime(s)
        let _returned_commands = unsafe {
            std::mem::transmute::<Commands<'static, 'static>, Commands<'_, '_>>(returned_commands_static)
        };
    }
}

unsafe impl AccessCellProvider<BevyEntity> for EntityCommands<'static> {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> AccessCell<Scoped, BevyEntity> {
        if method != "id" {
            panic!("Unsupported method '{}' in AccessCellProvider<BevyEntity> for EntityCommands", method);
        }
        if !args.is::<()>() {
            panic!("Unsupported arguments for method '{}' in AccessCellProvider<BevyEntity> for EntityCommands", method);
        }

        let id = self.id();

        AccessCell::new(id)
    }

    unsafe fn end_access(&mut self, handle: AccessCell<Scoped, BevyEntity>) {
        let _returned_id = handle.take();
    }
}
