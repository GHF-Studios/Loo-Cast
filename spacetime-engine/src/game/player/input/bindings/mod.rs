//! Runtime button bindings and console bind dispatch.
//!
//! Bindings translate hardware buttons to semantic player actions or explicit
//! console commands. They do not decide whether gameplay currently has focus.

use super::{
    PlayerAction,
    button::{PlayerInputButton, key_label},
};

mod dispatch;
mod model;
mod registry;

pub(in crate::game::player) use dispatch::dispatch_bound_console_commands;
pub(crate) use model::PlayerInputBindings;
pub(crate) use registry::PLAYER_BIND_TARGETS;
use registry::{DEFAULT_PLAYER_BINDINGS, bind_target_actions};
