use rhai::{Dynamic, FnPtr, NativeCallContext, Shared};

use crate::bevy::ecs::message::Messages;
use crate::bevy::ecs::query::With;
use crate::bevy::ecs::world::EntityWorldMut as BevyEntityWorldMut;
use crate::bevy::prelude::Commands as BevyCommands;
use crate::bevy::prelude::Entity as BevyEntity;
use crate::player::components::Player;
use crate::rhai_binding::value_semantics::access_traits::AccessCellProvider;

use crate::script::ecs::{
    bundle::internals::trait_objects::BundleTraitObject,
    messages::bindings::types::{MessageBatch, ScriptProbeMessage},
    query::bindings::types::{Query, QueryData, QueryDataKind, QueryFilter, QueryFilterKind},
    system::commands::bindings::types::Commands,
    world::{bindings::types::World, entity_ref::bindings::types::EntityWorldMut, internals::traits::WorldApi},
};

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

        let entity_world_mut_raw_handle: crate::rhai_binding::value_semantics::access_cell::AccessCell<
            crate::rhai_binding::value_semantics::access_cell::Scoped,
            BevyEntityWorldMut<'static>,
        > = unsafe { world.start_access("spawn", Box::new(bundle)) };
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

        let entities = match (data.kind, filter.kind) {
            (QueryDataKind::Entities, QueryFilterKind::None) => {
                let mut query = world.query::<BevyEntity>();
                query.iter(&*world).collect()
            }
            (QueryDataKind::Entities, QueryFilterKind::WithPlayer) => {
                let mut query = world.query_filtered::<BevyEntity, With<Player>>();
                query.iter(&*world).collect()
            }
        };

        self.world.end_write(world);

        Query { entities }
    }

    fn write_probe_message(&self, payload: rhai::ImmutableString) {
        let mut world = self.world.start_write();
        let message = ScriptProbeMessage { payload: payload.to_string() };

        if world.write_message(message).is_none() {
            self.world.end_write(world);
            panic!("ScriptProbeMessage writer unavailable. Ensure RhaiEnginePlugin registered add_message::<ScriptProbeMessage>()");
        }

        self.world.end_write(world);
    }

    fn read_probe_messages(&self) -> MessageBatch {
        let mut world = self.world.start_write();

        let payloads = {
            let Some(mut messages) = world.get_resource_mut::<Messages<ScriptProbeMessage>>() else {
                self.world.end_write(world);
                panic!("ScriptProbeMessage storage is unavailable. Ensure RhaiEnginePlugin registered add_message::<ScriptProbeMessage>()");
            };
            messages.drain().map(|message| message.payload).collect()
        };

        self.world.end_write(world);

        MessageBatch { payloads }
    }
}
