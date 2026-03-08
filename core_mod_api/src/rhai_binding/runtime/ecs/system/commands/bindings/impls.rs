use rhai::{Dynamic, FnPtr, NativeCallContext};

use crate::bevy::prelude::Entity as BevyEntity;
use crate::rhai_binding::runtime::ecs::system::commands::{
    bindings::types::{Commands, EntityCommands},
    internals::traits::{CommandsApi, EntityCommandsApi},
};
use crate::rhai_binding::value_semantics::access_traits::AccessCellProvider;

impl CommandsApi for Commands {
    fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut commands = self.commands.start_write();

        let entity_commands_raw_handle = unsafe { commands.start_access("spawn_empty", Box::new(())) };
        let entity_commands_binding = EntityCommands {
            entity_commands: entity_commands_raw_handle.clone(),
        };
        let output = callback.call_within_context::<Dynamic>(&ctx, (entity_commands_binding,));
        unsafe { commands.end_access(entity_commands_raw_handle) };
        self.commands.end_write(commands);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }
}

impl EntityCommandsApi for EntityCommands {
    fn commands(&self, ctx: NativeCallContext, f: FnPtr) -> Dynamic {
        let mut entity_commands = self.entity_commands.start_write();

        let commands_raw_handle = unsafe { entity_commands.start_access("commands", Box::new(())) };
        let commands_binding = Commands {
            commands: commands_raw_handle.clone(),
        };
        let output = f.call_within_context::<Dynamic>(&ctx, (commands_binding,));
        unsafe { entity_commands.end_access(commands_raw_handle) };
        self.entity_commands.end_write(entity_commands);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }

    fn id(&self) -> BevyEntity {
        let entity_commands = self.entity_commands.start_read();
        let id = entity_commands.id();
        self.entity_commands.end_read(entity_commands);

        id
    }
}
