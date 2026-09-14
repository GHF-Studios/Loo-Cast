//! Screen-space developer UI.
//!
//! This layer consumes structured developer data. It never owns simulation state
//! and never feeds strings back into world-draw primitives.

mod focus_badge;
pub(super) mod inspector;
mod tools;

use bevy::prelude::*;

pub(super) fn configure(app: &mut App) {
    focus_badge::configure(app);
    inspector::configure(app);
    tools::configure(app);
}
