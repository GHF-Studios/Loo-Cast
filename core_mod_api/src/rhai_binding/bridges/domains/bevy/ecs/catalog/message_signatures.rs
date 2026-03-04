//! Compile-time message signature catalog.
//!
//! Reserve this module for message reader/writer/drain operation signatures
//! that require compile-time registration.

use crate::bevy::ecs::message::Messages;
use crate::bevy::prelude::World as BevyWorld;
use crate::rhai_binding::runtime::ecs::message::bindings::types::ScriptProbeMessage;
use crate::rhai_binding::runtime::ecs::dispatch_policy::{
    submit_message_drain_dispatch_entry, submit_message_write_dispatch_entry,
};

pub const MESSAGE_SIG__SCRIPT_PROBE__WRITE: &str = "MESSAGE_SIG__SCRIPT_PROBE__WRITE";
pub const MESSAGE_SIG__SCRIPT_PROBE__DRAIN: &str = "MESSAGE_SIG__SCRIPT_PROBE__DRAIN";

pub const TYPE_PATH__SCRIPT_PROBE_MESSAGE: &str = "core_mod_api::rhai_binding::runtime::ecs::message::bindings::types::ScriptProbeMessage";

fn dispatch_message_sig_script_probe_write(world: &mut BevyWorld, payload: String) {
    let message = ScriptProbeMessage { payload };
    if world.write_message(message).is_none() {
        panic!("ScriptProbeMessage writer unavailable. Ensure RhaiEnginePlugin registered add_message::<ScriptProbeMessage>()");
    }
}

fn dispatch_message_sig_script_probe_drain(world: &mut BevyWorld) -> Vec<String> {
    let Some(mut messages) = world.get_resource_mut::<Messages<ScriptProbeMessage>>() else {
        panic!("ScriptProbeMessage storage is unavailable. Ensure RhaiEnginePlugin registered add_message::<ScriptProbeMessage>()");
    };

    messages.drain().map(|message| message.payload).collect()
}

submit_message_write_dispatch_entry!(
    signature_id = MESSAGE_SIG__SCRIPT_PROBE__WRITE,
    message_type_id = TYPE_PATH__SCRIPT_PROBE_MESSAGE,
    dispatch = dispatch_message_sig_script_probe_write,
);

submit_message_drain_dispatch_entry!(
    signature_id = MESSAGE_SIG__SCRIPT_PROBE__DRAIN,
    message_type_id = TYPE_PATH__SCRIPT_PROBE_MESSAGE,
    dispatch = dispatch_message_sig_script_probe_drain,
);
