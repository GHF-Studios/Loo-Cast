use bevy::prelude::Entity as BevyEntity;
use rhai::{Dynamic, FnPtr, NativeCallContext, Shared};

use crate::script::{
    core::internals::traits::ScopedAccessProvider,
    ecs::{
        system::commands::{bindings::types::{Commands, EntityCommands}, internals::traits::{CommandsApi, EntityCommandsApi}},
    }
};

impl CommandsApi for Shared<Commands> {
    fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut commands = self.commands
            .write()
            .expect("Commands write-lock failed");

        let mut out = Dynamic::UNIT;

        commands.write(|commands| {
            let entity_commands_raw_handle = unsafe { commands.start_access("spawn_empty", Box::new(())) };
            let entity_commands_binding = EntityCommands { entity_commands: entity_commands_raw_handle.clone() };
            let shared_entity_commands = Shared::new(entity_commands_binding);

            let (_returned_entity_commands, output): (Shared<EntityCommands>, Dynamic) =
                callback.call_within_context(&ctx, (shared_entity_commands,))
                    .expect("Callback failed");

            unsafe { commands.end_access(entity_commands_raw_handle) };

            out = output;
        }).unwrap_or_else(|e| {
            panic!("Commands access failed: {}", e);
        });

        out
    }
}

impl EntityCommandsApi for Shared<EntityCommands> {
    fn commands(&self, ctx: NativeCallContext, f: FnPtr) -> Dynamic {
        let mut entity_commands = self.entity_commands
            .write()
            .expect("EntityCommands write-lock failed");

        let mut out = Dynamic::UNIT;

        entity_commands.write(|entity_commands| {
            let commands_raw_handle = unsafe { entity_commands.start_access("commands", Box::new(())) };
            let commands_binding = Commands { commands: commands_raw_handle.clone() };
            let shared_commands = Shared::new(commands_binding);

            let (_returned_commands, output): (Shared<Commands>, Dynamic) =
                f.call_within_context(&ctx, (shared_commands,))
                    .expect("Callback failed");

            unsafe { entity_commands.end_access(commands_raw_handle) };
            
            out = output;
        }).unwrap_or_else(|e| {
            panic!("EntityCommands access failed: {}", e);
        });

        out
    }

    fn id(&self) -> BevyEntity {
        let entity_commands = self.entity_commands
            .read()
            .expect("EntityCommands read-lock failed");

        let id = entity_commands.read(|entity_commands| {
            entity_commands.id()
        }).unwrap_or_else(|e| {
            panic!("EntityCommands access failed: {}", e);
        });

        id
    }
}