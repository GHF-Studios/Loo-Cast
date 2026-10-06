//! Local-human device bindings and per-frame semantic input snapshot.
//!
//! Hardware state is sampled once after focus/cursor arbitration. Gameplay
//! systems consume `PlayerInputFrame` rather than inspecting buttons directly.
//!
//! ## Module map
//!
//! - `action`: Semantic actions sampled from bound devices.
//! - `bindings`: Runtime button bindings and console bind dispatch.
//! - `button`: Bindable device vocabulary and hardware button sampling.
//! - `frame`: One gameplay input snapshot sampled after focus and cursor arbitration.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use bevy::prelude::*;

mod action;
mod bindings;
mod button;
mod frame;

pub(crate) use action::PlayerAction;
pub(super) use bindings::dispatch_bound_console_commands;
pub(crate) use bindings::{PLAYER_BIND_TARGETS, PlayerInputBindings};
pub(crate) use button::BINDABLE_INPUT_NAMES;
pub(crate) use frame::PlayerInputFrame;
pub(super) use frame::sample_player_input;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum PlayerInputSet {
    Cursor,
    Sample,
}
