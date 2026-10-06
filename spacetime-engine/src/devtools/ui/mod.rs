//! Screen-space developer UI.
//!
//! This layer consumes structured developer data. It never owns simulation state
//! and never feeds strings back into world-draw primitives.
//!
//! ## Module map
//!
//! - `focus_badge`: Optional single screen-space badge for the current world focus.
//! - `inspector`: Compact Bevy-UI semantic Inspector for the canonical tooling focus.
//! - `tools`: Tiny flat Developer Tools palette.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

mod focus_badge;
pub(super) mod inspector;
mod tools;

use bevy::prelude::*;

pub(super) fn configure(app: &mut App) {
    focus_badge::configure(app);
    inspector::configure(app);
    tools::configure(app);
}
