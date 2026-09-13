//! Screen-space developer UI.
//!
//! This layer consumes structured developer data. It never owns simulation state
//! and never feeds strings back into world-draw primitives.

mod inspector;

use bevy::prelude::*;

pub(super) fn configure(app: &mut App) {
    inspector::configure(app);
}
