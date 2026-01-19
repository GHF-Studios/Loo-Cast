use bevy::prelude::{Commands as BevyCommands, EntityCommands as BevyEntityCommands};
use rhai::{Dynamic, FnPtr, NativeCallContext};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::script::{core::internals::{traits::ScopedAccessProvider, types::ScopedAccessHandle}, ecs::entity::bindings::types::EntityId};

#[derive(Clone)]
#[repr(transparent)]
pub struct Commands {
    pub(crate) commands: ScopedAccessHandle<BevyCommands<'static, 'static>>
}
impl Commands {
    pub fn entity_commands(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut commands = self.commands
            .write()
            .expect("Commands write-lock failed");

        let mut out = Dynamic::UNIT;

        commands.write(|commands| {
            let raw_handle = unsafe { commands.start_access() };
            let entity_commands = EntityCommands { entity_commands: raw_handle.clone()};

            let (_entity_commands, result): (EntityCommands, Dynamic) =
                callback.call_within_context(&ctx, (entity_commands,))
                    .expect("Callback failed");

            unsafe { commands.end_access(raw_handle) };

            out = result;
        }).unwrap_or_else(|e| {
            panic!("Commands access failed: {}", e);
        });

        out
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityCommands {
    entity_commands: ScopedAccessHandle<bevy::prelude::EntityCommands<'static>>
}
impl EntityCommands {
    pub fn commands(&mut self, ctx: NativeCallContext, f: FnPtr) -> Dynamic {
        let mut entity_commands = self.entity_commands
            .write()
            .expect("EntityCommands write-lock failed");

        let mut out = Dynamic::UNIT;

        entity_commands.write(|entity_commands| {
            let raw_handle = unsafe { entity_commands.start_access() };
            let commands = Commands { commands: raw_handle.clone()};

            let (_commands, result): (Commands, Dynamic) =
                f.call_within_context(&ctx, (commands,))
                    .expect("Callback failed");

            unsafe { entity_commands.end_access(raw_handle) };

            out = result;
        }).unwrap_or_else(|e| {
            panic!("EntityCommands access failed: {}", e);
        });

        out
    }
}
