//! Resolved hard-body context used by travel policy.

use crate::spatial::{SpatialScale, UsfPosition};
use bevy::prelude::*;

/// One resolved primary hard body for the current navigation subject.
///
/// This is travel geometry and identity only. Physical fields such as gravity
/// are queried from their own domains and must not be smuggled through
/// navigation state.
#[derive(Component, Debug, Clone, Copy)]
pub struct PrimaryBodyContext {
    entity: Option<Entity>,
    center: UsfPosition,
    radius_metres: f64,
    reference_scale: SpatialScale,
    center_distance_metres: f64,
    clearance_metres: f64,
}

impl Default for PrimaryBodyContext {
    fn default() -> Self {
        Self {
            entity: None,
            center: UsfPosition::zero(SpatialScale::MAX),
            radius_metres: 0.0,
            reference_scale: SpatialScale::MAX,
            center_distance_metres: f64::INFINITY,
            clearance_metres: f64::INFINITY,
        }
    }
}

impl PrimaryBodyContext {
    pub fn resolved(
        entity: Entity,
        center: UsfPosition,
        radius_metres: f64,
        reference_scale: SpatialScale,
        center_distance_metres: f64,
        clearance_metres: f64,
    ) -> Self {
        Self {
            entity: Some(entity),
            center,
            radius_metres,
            reference_scale,
            center_distance_metres,
            clearance_metres,
        }
    }

    pub const fn entity(self) -> Option<Entity> {
        self.entity
    }

    pub const fn center(self) -> UsfPosition {
        self.center
    }

    pub const fn radius_metres(self) -> f64 {
        self.radius_metres
    }

    pub const fn reference_scale(self) -> SpatialScale {
        self.reference_scale
    }

    pub const fn center_distance_metres(self) -> f64 {
        self.center_distance_metres
    }

    pub const fn clearance_metres(self) -> f64 {
        self.clearance_metres
    }

    pub const fn is_resolved(self) -> bool {
        self.entity.is_some()
    }
}
