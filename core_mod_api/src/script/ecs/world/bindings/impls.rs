use bevy::prelude::Commands as BevyCommands;
use bevy::ecs::world::EntityWorldMut as BevyEntityWorldMut;
use rhai::{Dynamic, FnPtr, NativeCallContext, Shared};

use crate::script::{
    core::internals::{traits::ScopedAccessProvider, types::ScopedAccessHandle},
    ecs::{
        system::commands::bindings::types::Commands,
        world::{
            bindings::types::World,
            entity_ref::bindings::types::EntityWorldMut,
            internals::traits::WorldApi
        }
    }
};

impl WorldApi for Shared<World> {
    fn commands(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = self.world
            .write()
            .expect("World write-lock failed");

        let mut out = Dynamic::UNIT;

        world.write(|world| {
            let commands_raw_handle: ScopedAccessHandle<BevyCommands<'static, 'static>> = unsafe { world.start_access("commands", Box::new(())) };
            let commands_binding = Commands { commands: commands_raw_handle.clone() };
            let shared_commands = Shared::new(commands_binding);

            let (_returned_commands, output): (Shared<Commands>, Dynamic) =
                callback.call_within_context(&ctx, (shared_commands,))
                    .expect("Callback failed");

            unsafe { world.end_access(commands_raw_handle) };

            out = output;
        }).unwrap_or_else(|e| {
            panic!("World access failed: {}", e);
        });

        out
    }

    fn flush(&self) {
        let mut world = self.world
            .write()
            .expect("World write-lock failed");

        world.write(|world| {
            world.flush();
        }).unwrap_or_else(|e| {
            panic!("World access failed: {}", e);
        });
    }

    fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = self.world
            .write()
            .expect("World write-lock failed");

        let mut out = Dynamic::UNIT;

        world.write(|world| {
            let entity_world_mut_raw_handle: ScopedAccessHandle<BevyEntityWorldMut<'static>> = unsafe { world.start_access("spawn_empty", Box::new(())) };
            let entity_world_mut = EntityWorldMut { entity_world_mut: entity_world_mut_raw_handle.clone() };
            let shared_entity_world_mut = Shared::new(entity_world_mut);

            let (_returned_entity_world_mut, output): (Shared<EntityWorldMut>, Dynamic) =
                callback.call_within_context(&ctx, (shared_entity_world_mut,))
                    .expect("Callback failed");

            unsafe { world.end_access(entity_world_mut_raw_handle) };

            out = output;
        }).unwrap_or_else(|e| {
            panic!("World access failed: {}", e);
        });

        out
    }
}