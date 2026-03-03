use rhai::{Dynamic, FnPtr, NativeCallContext, Shared};
use std::sync::TryLockError;

use crate::bevy::prelude::Entity as BevyEntity;
use crate::rhai_binding::value_semantics::access_traits::ScopedAccessProvider;
use crate::script::ecs::system::commands::{bindings::types::{Commands, EntityCommands}, internals::traits::{CommandsApi, EntityCommandsApi}};

impl CommandsApi for Shared<Commands> {
    fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut commands = match self.commands.0.try_write() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(_)) => panic!("Commands lock poisoned"),
            Err(TryLockError::WouldBlock) => panic!("Commands is already borrowed elsewhere"),
        };

        commands.write(|commands| {
            let entity_commands_raw_handle = unsafe { commands.start_access("spawn_empty", Box::new(())) };
            let entity_commands_binding = EntityCommands { entity_commands: entity_commands_raw_handle.clone() };
            let shared_entity_commands = Shared::new(entity_commands_binding);

            let output: Dynamic =
                callback.call_within_context(&ctx, (shared_entity_commands,))
                    .expect("Callback failed");

            unsafe { commands.end_access(entity_commands_raw_handle) };

            output
        }).unwrap_or_else(|e| {
            panic!("Commands access failed: {}", e);
        })
    }
}

impl EntityCommandsApi for Shared<EntityCommands> {
    fn commands(&self, ctx: NativeCallContext, f: FnPtr) -> Dynamic {
        let mut entity_commands = match self.entity_commands.0.try_write() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(_)) => panic!("EntityCommands lock poisoned"),
            Err(TryLockError::WouldBlock) => panic!("EntityCommands is already borrowed elsewhere"),
        };

        entity_commands.write(|entity_commands| {
            let commands_raw_handle = unsafe { entity_commands.start_access("commands", Box::new(())) };
            let commands_binding = Commands { commands: commands_raw_handle.clone() };
            let shared_commands = Shared::new(commands_binding);

            let output: Dynamic =
                f.call_within_context(&ctx, (shared_commands,))
                    .expect("Callback failed");

            unsafe { entity_commands.end_access(commands_raw_handle) };
            
            output
        }).unwrap_or_else(|e| {
            panic!("EntityCommands access failed: {}", e);
        })
    }

    fn id(&self) -> BevyEntity {
        let entity_commands = match self.entity_commands.0.try_read() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(_)) => panic!("EntityCommands lock poisoned"),
            Err(TryLockError::WouldBlock) => panic!("EntityCommands is already borrowed elsewhere"),
        };

        entity_commands.read(|entity_commands| {
            entity_commands.id()
        }).unwrap_or_else(|e| {
            panic!("EntityCommands access failed: {}", e);
        })
    }
}
