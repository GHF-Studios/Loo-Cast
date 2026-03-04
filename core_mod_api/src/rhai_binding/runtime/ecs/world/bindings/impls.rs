use rhai::{Dynamic, FnPtr, NativeCallContext, Shared};

use crate::bevy::ecs::world::EntityWorldMut as BevyEntityWorldMut;
use crate::bevy::prelude::Commands as BevyCommands;
use crate::rhai_binding::value_semantics::access_traits::AccessCellProvider;

use crate::rhai_binding::runtime::ecs::{
    bundle::internals::trait_objects::BundleTraitObject,
    system::query::bindings::types::{Query, QueryData, QueryFilter},
    system::commands::bindings::types::Commands,
    world::{
        bindings::types::World,
        entity_ref::bindings::types::EntityWorldMut,
        internals::{
            access_requests::{
                WorldQueryRequest, WorldSpawnSingleRequest, WriteProbeMessageRequest, WORLD_ACCESS_METHOD_DRAIN_PROBE_MESSAGES,
                WORLD_ACCESS_METHOD_QUERY, WORLD_ACCESS_METHOD_SPAWN_SINGLE, WORLD_ACCESS_METHOD_WRITE_PROBE_MESSAGE,
            },
            traits::WorldApi,
        },
    },
};
use crate::rhai_binding::runtime::std::iter::bindings::types::StringIter;

impl WorldApi for Shared<World> {
    fn commands(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = self.world.start_write();

        let commands_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            BevyCommands<'static, 'static>,
        > = unsafe { world.start_access("commands", Box::new(())) };
        let commands_binding = Commands {
            commands: commands_raw_handle.clone(),
        };
        let shared_commands = Shared::new(commands_binding);

        let output = callback.call_within_context::<Dynamic>(&ctx, (shared_commands.clone(),));

        drop(shared_commands);
        unsafe { world.end_access(commands_raw_handle) };
        self.world.end_write(world);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }

    fn flush(&self) {
        let mut world = self.world.start_write();
        world.flush();
        self.world.end_write(world);
    }

    fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = self.world.start_write();

        let entity_world_mut_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            BevyEntityWorldMut<'static>,
        > = unsafe { world.start_access("spawn_empty", Box::new(())) };
        let entity_world_mut = EntityWorldMut {
            entity_world_mut: entity_world_mut_raw_handle.clone(),
        };
        let shared_entity_world_mut = Shared::new(entity_world_mut);

        let output = callback.call_within_context::<Dynamic>(&ctx, (shared_entity_world_mut.clone(),));

        drop(shared_entity_world_mut);
        unsafe { world.end_access(entity_world_mut_raw_handle) };
        self.world.end_write(world);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }

    fn spawn_single(&self, bundle: BundleTraitObject, ctx: NativeCallContext, callback: FnPtr) -> Dynamic {
        let mut world = self.world.start_write();
        let spawn_request = WorldSpawnSingleRequest::new(bundle);

        let entity_world_mut_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            BevyEntityWorldMut<'static>,
        > = unsafe { world.start_access(WORLD_ACCESS_METHOD_SPAWN_SINGLE, Box::new(spawn_request)) };
        let entity_world_mut = EntityWorldMut {
            entity_world_mut: entity_world_mut_raw_handle.clone(),
        };
        let shared_entity_world_mut = Shared::new(entity_world_mut);

        let output = callback.call_within_context::<Dynamic>(&ctx, (shared_entity_world_mut.clone(),));

        drop(shared_entity_world_mut);
        unsafe { world.end_access(entity_world_mut_raw_handle) };
        self.world.end_write(world);

        output.unwrap_or_else(|e| {
            panic!("Callback failed: {e}");
        })
    }

    fn query(&self, data: QueryData) -> Query {
        self.query_filtered(data, QueryFilter::none())
    }

    fn query_filtered(&self, data: QueryData, filter: QueryFilter) -> Query {
        let mut world = self.world.start_write();
        let query_request = WorldQueryRequest::new(data, filter);

        let query_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            Query,
        > = unsafe { world.start_access(WORLD_ACCESS_METHOD_QUERY, Box::new(query_request)) };
        let query = query_raw_handle.take();
        unsafe { world.end_access(query_raw_handle) };

        self.world.end_write(world);

        query
    }

    fn write_probe_message(&self, payload: rhai::ImmutableString) {
        let mut world = self.world.start_write();
        let write_request = WriteProbeMessageRequest::new(payload.to_string());
        let write_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            (),
        > = unsafe { world.start_access(WORLD_ACCESS_METHOD_WRITE_PROBE_MESSAGE, Box::new(write_request)) };
        unsafe { world.end_access(write_raw_handle) };

        self.world.end_write(world);
    }

    fn drain_probe_messages(&self) -> StringIter {
        let mut world = self.world.start_write();
        let message_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            StringIter,
        > = unsafe { world.start_access(WORLD_ACCESS_METHOD_DRAIN_PROBE_MESSAGES, Box::new(())) };
        let messages = message_raw_handle.take();
        unsafe { world.end_access(message_raw_handle) };

        self.world.end_write(world);

        messages
    }
}
