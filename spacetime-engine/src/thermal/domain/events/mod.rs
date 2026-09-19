//! Thermal domain messages.

use bevy::prelude::*;

/// Instant signed thermal energy transfer into one entity.
///
/// Positive values heat; negative values cool. The target may be either a
/// semantic entity or one of its manifestations; simulation canonicalizes it.
#[derive(Message, Debug, Clone, Copy)]
pub struct ThermalImpulse {
    pub target: Entity,
    pub energy_joules: f32,
}
