use rhai::{Dynamic, FnPtr, NativeCallContext, Shared};
use std::sync::TryLockError;

use crate::bevy::prelude::Commands as BevyCommands;
use crate::bevy::ecs::world::EntityWorldMut as BevyEntityWorldMut;
use crate::reflection::internals::managed_traits::BundleTraitObject;
use crate::rhai_binding::value_semantics::access_traits::ScopedAccessProvider;
use crate::rhai_binding::value_semantics::scoped_access::ScopedAccessHandle;

use crate::script::{
    ecs::{
        bundle::bindings::types::Bundle, system::commands::bindings::types::Commands, world::{
            bindings::types::World,
            entity_ref::bindings::types::EntityWorldMut,
            internals::traits::WorldApi
        }
    }
};

impl WorldApi for Shared<World> {
    fn commands(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = match self.world.0.try_write() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(_)) => panic!("World lock poisoned"),
            Err(TryLockError::WouldBlock) => panic!("World is already borrowed elsewhere"),
        };

        world.write(|world| {
            let commands_raw_handle: ScopedAccessHandle<BevyCommands<'static, 'static>> = unsafe { world.start_access("commands", Box::new(())) };
            let commands_binding = Commands { commands: commands_raw_handle.clone() };
            let shared_commands = Shared::new(commands_binding);

            let output: Dynamic =
                callback.call_within_context(&ctx, (shared_commands,))
                    .expect("Callback failed");

            unsafe { world.end_access(commands_raw_handle) };

            output
        }).unwrap_or_else(|e| {
            panic!("World access failed: {}", e);
        })
    }

    fn flush(&self) {
        let mut world = match self.world.0.try_write() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(_)) => panic!("World lock poisoned"),
            Err(TryLockError::WouldBlock) => panic!("World is already borrowed elsewhere"),
        };

        world.write(|world| {
            world.flush();
        }).unwrap_or_else(|e| {
            panic!("World access failed: {}", e);
        })
    }

    fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = match self.world.0.try_write() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(_)) => panic!("World lock poisoned"),
            Err(TryLockError::WouldBlock) => panic!("World is already borrowed elsewhere"),
        };

        world.write(|world| {
            let entity_world_mut_raw_handle: ScopedAccessHandle<BevyEntityWorldMut<'static>> = unsafe { world.start_access("spawn_empty", Box::new(())) };
            let entity_world_mut = EntityWorldMut { entity_world_mut: entity_world_mut_raw_handle.clone() };
            let shared_entity_world_mut = Shared::new(entity_world_mut);

            let output: Dynamic =
                callback.call_within_context(&ctx, (shared_entity_world_mut,))
                    .expect("Callback failed");

            unsafe { world.end_access(entity_world_mut_raw_handle) };

            output
        }).unwrap_or_else(|e| {
            panic!("World access failed: {}", e);
        })
    }

    fn spawn_single(&self, bundle: Shared<BundleTraitObject>, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = match self.world.0.try_write() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(_)) => panic!("World lock poisoned"),
            Err(TryLockError::WouldBlock) => panic!("World is already borrowed elsewhere"),
        };

        world.write(|world| {
            let entity_world_mut_raw_handle: ScopedAccessHandle<BevyEntityWorldMut<'static>> = unsafe { world.start_access("spawn", Box::new(bundle)) };
            let entity_world_mut = EntityWorldMut { entity_world_mut: entity_world_mut_raw_handle.clone() };
            let shared_entity_world_mut = Shared::new(entity_world_mut);

            let output: Dynamic =
                callback.call_within_context(&ctx, (shared_entity_world_mut,))
                    .expect("Callback failed");

            unsafe { world.end_access(entity_world_mut_raw_handle) };

            output
        }).unwrap_or_else(|e| {
            panic!("World access failed: {}", e);
        })
    }
}
