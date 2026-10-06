//! Game-owned adapters for typed runtime console variables.
//!
//! Each group writes through its existing policy owner. Console state never
//! becomes an alternate source of simulation truth.
//!
//! ## Module map
//!
//! - `character`: Controlled character movement bindings.
//! - `freecam`: Detached camera controls and their presentation-only demand policy.
//! - `lab`: Developer locomotion override binding.
//! - `voxel`: Engine config override bindings for voxel work budgets.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

use crate::console::{AppConsoleExt, ConsoleArgumentCompletion, ConsoleCommandSpec};
use bevy::prelude::*;

mod character;
mod freecam;
mod lab;
mod voxel;

pub(super) fn configure(app: &mut App) {
    voxel::register(app);
    character::register(app);
    freecam::register(app);
    lab::register(app);

    app.register_console_command_with_completion(
        ConsoleCommandSpec {
            name: "debug freecam",
            aliases: &["freecam"],
            usage: "debug freecam [on|off|toggle]",
            summary: "Detach/restore the local debug camera without moving gameplay authority.",
        },
        &[ConsoleArgumentCompletion::Static(&["on", "off", "toggle"])],
        freecam::debug_freecam_command,
    );
}

pub(super) fn parse_usize(raw: &str, field: &str, minimum: usize) -> Result<usize, String> {
    let value = raw
        .parse::<usize>()
        .map_err(|_| format!("{field} must be an unsigned integer"))?;
    if value < minimum {
        return Err(format!("{field} must be >= {minimum}"));
    }
    Ok(value)
}

pub(super) fn parse_nonnegative_f32(raw: &str, field: &str) -> Result<f32, String> {
    let value = raw
        .parse::<f32>()
        .map_err(|_| format!("{field} must be a number"))?;
    if !value.is_finite() || value < 0.0 {
        return Err(format!("{field} must be finite and non-negative"));
    }
    Ok(value)
}

pub(super) fn parse_positive_f32(raw: &str, field: &str) -> Result<f32, String> {
    let value = raw
        .parse::<f32>()
        .map_err(|_| format!("{field} must be a number"))?;
    if !value.is_finite() || value <= 0.0 {
        return Err(format!("{field} must be finite and > 0"));
    }
    Ok(value)
}

pub(super) fn parse_bool(raw: &str, field: &str) -> Result<bool, String> {
    match raw.to_ascii_lowercase().as_str() {
        "true" | "on" | "1" => Ok(true),
        "false" | "off" | "0" => Ok(false),
        _ => Err(format!("{field} must be true/false or on/off")),
    }
}
