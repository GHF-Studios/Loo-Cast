//! Shared semantic travel-envelope policy.
//!
//! These thresholds are gameplay handoff semantics in canonical metre units.
//! Travel telemetry and Cruise braking share them so the two cannot drift.

const PLANETARY_HANDOFF_RADIUS_FRACTION: f64 = 0.12;
const PLANETARY_HANDOFF_MIN_METRES: f64 = 20_000.0;
const PLANETARY_HANDOFF_MAX_METRES: f64 = 750_000.0;
const CRITICAL_DROPOUT_FRACTION: f64 = 0.25;

pub(super) fn planetary_handoff_clearance(radius_metres: f64) -> f64 {
    (radius_metres * PLANETARY_HANDOFF_RADIUS_FRACTION)
        .clamp(PLANETARY_HANDOFF_MIN_METRES, PLANETARY_HANDOFF_MAX_METRES)
}

pub(super) fn critical_dropout_clearance(radius_metres: f64) -> f64 {
    planetary_handoff_clearance(radius_metres) * CRITICAL_DROPOUT_FRACTION
}
