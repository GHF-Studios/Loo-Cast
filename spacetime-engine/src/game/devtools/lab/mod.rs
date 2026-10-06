//! Live developer override/preset composition.
//!
//!
//! This layer composes existing typed runtime-variable adapters. It does not
//! become a second config/cvar authority. Presets are reversible transactions:
//! they capture the pre-preset baseline, apply through validated setters, and
//! rebuild effective values when presets are removed.

use std::collections::{BTreeMap, BTreeSet};

use bevy::prelude::*;

use crate::console::{
    AppConsoleExt, ConsoleArgumentCompletion, ConsoleCommandInvocation, ConsoleCommandResult,
    ConsoleCommandSpec, RuntimeVariableBinding, RuntimeVariableRegistry,
};

mod commands;
mod model;
mod transaction;

pub(crate) use commands::configure;
use model::FREECAM_PRESET;
pub(crate) use model::{
    DeveloperPresetRegistry, DeveloperPresetSpec, DeveloperPresetState,
};
pub(crate) use transaction::{apply_preset, clear_preset, preset_active};

fn normalize(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace('-', "_")
}
