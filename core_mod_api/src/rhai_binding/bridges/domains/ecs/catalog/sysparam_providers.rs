use std::any::Any;

use crate::bevy::ecs::message::Messages;
use crate::bevy::ecs::system::Commands;
use crate::bevy::ecs::system::EntityCommands;
use crate::bevy::ecs::world::EntityWorldMut;
use crate::bevy::prelude::Entity as BevyEntity;
use crate::bevy::prelude::World;
use crate::player::bundles::PlayerBundle;
use crate::rhai_binding::meta::abstract_::trait_identity::ToTraitObject;
use crate::rhai_binding::value_semantics::access_cell::{AccessCell, Persistent, Scoped};
use crate::rhai_binding::value_semantics::access_traits::AccessCellProvider;
use crate::rhai_binding::value_semantics::modes::{GetTypeValueSemantics, TypeValueSemantics};
use crate::rhai_binding::runtime::ecs::bundle::internals::trait_objects::{BundleTrait, BundleTraitObject};
use crate::rhai_binding::runtime::ecs::messages::bindings::types::ScriptProbeMessage;
use crate::rhai_binding::runtime::ecs::query::{bindings::types::Query, internals::statics::resolve_query_dispatch};
use crate::rhai_binding::runtime::ecs::world::internals::access_requests::{
    WorldQueryRequest, WriteProbeMessageRequest, WORLD_ACCESS_METHOD_DRAIN_PROBE_MESSAGES, WORLD_ACCESS_METHOD_QUERY,
    WORLD_ACCESS_METHOD_WRITE_PROBE_MESSAGE,
};
use crate::rhai_binding::runtime::rust::iter::bindings::types::StringIter;

unsafe impl AccessCellProvider<Commands<'static, 'static>> for World {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> AccessCell<Scoped, Commands<'static, 'static>> {
        if method != "commands" {
            panic!("Unsupported method '{}' in AccessCellProvider<Commands> for World", method);
        }
        if !args.is::<()>() {
            panic!("Unsupported arguments for method '{}' in AccessCellProvider<Commands> for World", method);
        }

        let commands = self.commands();

        // Erase lifetime(s)
        let commands_static = unsafe { std::mem::transmute::<Commands<'_, '_>, Commands<'static, 'static>>(commands) };

        AccessCell::new(commands_static)
    }

    unsafe fn end_access(&mut self, handle: AccessCell<Scoped, Commands<'static, 'static>>) {
        let returned_static_commands = handle.take();

        // Restore lifetime(s)
        let _returned_commands = unsafe { std::mem::transmute::<Commands<'static, 'static>, Commands<'_, '_>>(returned_static_commands) };
    }
}

unsafe impl AccessCellProvider<EntityWorldMut<'static>> for World {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> AccessCell<Scoped, EntityWorldMut<'static>> {
        let entity_world_mut = match method {
            "spawn_empty" => {
                if !args.is::<()>() {
                    panic!("Unsupported arguments for method '{}' in AccessCellProvider<EntityWorldMut> for World", method);
                }

                self.spawn_empty()
            }
            "spawn" => {
                let Ok(bundle) = args.downcast::<BundleTraitObject>() else {
                    panic!("Unsupported arguments for method '{}' in AccessCellProvider<EntityWorldMut> for World", method);
                };
                let mut ent = self.spawn_empty();
                let bundle = *bundle;
                match <PlayerBundle as GetTypeValueSemantics>::VALUE_SEMANTICS {
                    TypeValueSemantics::ScopedMut => {
                        let bundle: AccessCell<Scoped, PlayerBundle> = ToTraitObject::<BundleTrait>::cast_from(bundle.0);
                        ent.insert(bundle.take());
                    }
                    TypeValueSemantics::Owned => {
                        let bundle: AccessCell<Persistent, PlayerBundle> = ToTraitObject::<BundleTrait>::cast_from(bundle.0);
                        ent.insert(bundle.take());
                    }
                    TypeValueSemantics::Clone
                    | TypeValueSemantics::Ref
                    | TypeValueSemantics::Mut
                    | TypeValueSemantics::ScopedOwned
                    | TypeValueSemantics::ScopedRef => {
                        panic!("World::spawn currently supports PlayerBundle semantics: owned | scoped_mut")
                    }
                }
                ent
            }
            _ => panic!("Unsupported method '{}' in AccessCellProvider<EntityWorldMut> for World", method),
        };

        // Erase lifetime(s)
        let entity_world_mut_static = unsafe { std::mem::transmute::<EntityWorldMut<'_>, EntityWorldMut<'static>>(entity_world_mut) };

        AccessCell::new(entity_world_mut_static)
    }

    unsafe fn end_access(&mut self, handle: AccessCell<Scoped, EntityWorldMut<'static>>) {
        let returned_static_entity_world_mut = handle.take();

        // Restore lifetime(s)
        let _returned_entity_world_mut =
            unsafe { std::mem::transmute::<EntityWorldMut<'static>, EntityWorldMut<'_>>(returned_static_entity_world_mut) };
    }
}

unsafe impl AccessCellProvider<Query> for World {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> AccessCell<Scoped, Query> {
        if method != WORLD_ACCESS_METHOD_QUERY {
            panic!("Unsupported method '{}' in AccessCellProvider<Query> for World", method);
        }
        let Ok(request) = args.downcast::<WorldQueryRequest>() else {
            panic!("Unsupported arguments for method '{}' in AccessCellProvider<Query> for World", method);
        };
        let request = *request;

        let data_key = request.data.dispatch_key();
        let filter_key = request.filter.dispatch_key();
        let dispatch = resolve_query_dispatch(data_key.as_str(), filter_key.as_str());
        let query = dispatch(self);

        AccessCell::new(query)
    }

    unsafe fn end_access(&mut self, handle: AccessCell<Scoped, Query>) {
        let _handle = handle;
    }
}

unsafe impl AccessCellProvider<()> for World {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> AccessCell<Scoped, ()> {
        if method != WORLD_ACCESS_METHOD_WRITE_PROBE_MESSAGE {
            panic!("Unsupported method '{}' in AccessCellProvider<()> for World", method);
        }
        let Ok(request) = args.downcast::<WriteProbeMessageRequest>() else {
            panic!("Unsupported arguments for method '{}' in AccessCellProvider<()> for World", method);
        };
        let request = *request;
        let message = ScriptProbeMessage {
            payload: request.payload,
        };

        if self.write_message(message).is_none() {
            panic!("ScriptProbeMessage writer unavailable. Ensure RhaiEnginePlugin registered add_message::<ScriptProbeMessage>()");
        }

        AccessCell::new(())
    }

    unsafe fn end_access(&mut self, handle: AccessCell<Scoped, ()>) {
        let _handle = handle;
    }
}

unsafe impl AccessCellProvider<StringIter> for World {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> AccessCell<Scoped, StringIter> {
        if method != WORLD_ACCESS_METHOD_DRAIN_PROBE_MESSAGES {
            panic!("Unsupported method '{}' in AccessCellProvider<StringIter> for World", method);
        }
        if !args.is::<()>() {
            panic!("Unsupported arguments for method '{}' in AccessCellProvider<StringIter> for World", method);
        }

        let payloads = {
            let Some(mut messages) = self.get_resource_mut::<Messages<ScriptProbeMessage>>() else {
                panic!("ScriptProbeMessage storage is unavailable. Ensure RhaiEnginePlugin registered add_message::<ScriptProbeMessage>()");
            };
            messages.drain().map(|message| message.payload).collect()
        };

        AccessCell::new(StringIter::from_values(payloads))
    }

    unsafe fn end_access(&mut self, handle: AccessCell<Scoped, StringIter>) {
        let _handle = handle;
    }
}

unsafe impl AccessCellProvider<EntityCommands<'static>> for Commands<'static, 'static> {
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> AccessCell<Scoped, EntityCommands<'static>> {
        let entity_commands = match method {
            "spawn_empty" => {
                if !args.is::<()>() {
                    panic!("Unsupported arguments for method '{}' in AccessCellProvider<EntityCommands> for Commands", method);
                }

                self.spawn_empty()
            }
            _ => panic!("Unsupported method '{}' in AccessCellProvider<EntityCommands> for Commands", method),
        };

        // Erase lifetime(s)
        let entity_commands_static = unsafe { std::mem::transmute::<EntityCommands<'_>, EntityCommands<'static>>(entity_commands) };

        AccessCell::new(entity_commands_static)
    }

    unsafe fn end_access(&mut self, handle: AccessCell<Scoped, EntityCommands<'static>>) {
        let returned_entity_commands_static = handle.take();

        // Restore lifetime(s)
        let _returned_entity_commands =
            unsafe { std::mem::transmute::<EntityCommands<'static>, EntityCommands<'_>>(returned_entity_commands_static) };
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
        let commands_static = unsafe { std::mem::transmute::<Commands<'_, '_>, Commands<'static, 'static>>(commands) };

        AccessCell::new(commands_static)
    }

    unsafe fn end_access(&mut self, handle: AccessCell<Scoped, Commands<'static, 'static>>) {
        let returned_commands_static = handle.take();

        // Restore lifetime(s)
        let _returned_commands = unsafe { std::mem::transmute::<Commands<'static, 'static>, Commands<'_, '_>>(returned_commands_static) };
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
