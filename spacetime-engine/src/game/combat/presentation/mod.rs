//! Derived combat presentation.

mod health;

use bevy::prelude::*;

pub(super) fn configure(app: &mut App) {
    health::configure(app);
}
