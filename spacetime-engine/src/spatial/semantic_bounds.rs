//! Reconstructible semantic envelope for generic spatial queries and observers.

use bevy::prelude::*;

/// A conservative non-authoritative spatial envelope supplied by a semantic
/// phenomenon for observer/query broadphase and distant representation.
///
/// The semantic phenomenon owns its true geometry. This bound is disposable
/// derived evidence: it can be refined without changing identity, physics,
/// canonical position, or realization Scale. Domains with changing extents
/// must republish their bound when their semantic geometry changes.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct UsfSemanticBounds {
    radius_metres: f64,
}

impl UsfSemanticBounds {
    /// Spherical conservative extent is the first supported bounded envelope.
    /// Non-spherical/unbounded phenomena can later provide different query
    /// envelopes without adding special cases to projection consumers.
    pub fn sphere(radius_metres: f64) -> Self {
        assert!(radius_metres.is_finite() && radius_metres > 0.0);
        Self { radius_metres }
    }

    pub const fn radius_metres(self) -> f64 {
        self.radius_metres
    }
}

