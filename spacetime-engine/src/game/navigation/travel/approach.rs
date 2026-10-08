//! Readiness for capability refinement during approach.

use crate::spatial::SpatialScale;
use bevy::prelude::*;

/// Semantic progress through future capability refinement.
///
/// This state owns readiness/refinement only. Interaction Scale is controlled
/// elsewhere by the controlled manifestation's explicit Scale affinity.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ApproachRefinementState {
    pub active: bool,
    pub minimum_scale: SpatialScale,
    pub realization_target_scale: SpatialScale,
}

impl Default for ApproachRefinementState {
    fn default() -> Self {
        Self {
            active: false,
            minimum_scale: SpatialScale::MAX,
            realization_target_scale: SpatialScale::MAX,
        }
    }
}
