//! View presentation policy and navigation health snapshot.

use crate::spatial::SpatialScale;
use bevy::prelude::*;

/// View-owned automatic presentation policy for semantic navigation.
///
/// The minimum is a content/realizer capability boundary for this view profile,
/// not a privileged USF floor. The current game defaults to S0 because the
/// present macro terrain and human-scale content are authored through metres.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct NavigationPresentationProfile {
    pub minimum_scale: SpatialScale,
    pub maximum_scale: SpatialScale,
    pub response_decades_per_second: f32,
    pub maximum_manual_bias_decades: f32,
}

impl Default for NavigationPresentationProfile {
    fn default() -> Self {
        Self {
            minimum_scale: SpatialScale::ZERO,
            maximum_scale: SpatialScale::MAX,
            response_decades_per_second: 10.0,
            maximum_manual_bias_decades: 8.0,
        }
    }
}

/// Persistent state of the automatic presentation planner.
///
/// `manual_bias_decades` is an offset on semantic automatic scale, so manual
/// zoom and automatic navigation compose instead of racing over view state.
#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct NavigationPresentationState {
    initialized: bool,
    automatic_target_exponent: f32,
    effective_target_exponent: f32,
    manual_bias_decades: f32,
}

impl Default for NavigationPresentationState {
    fn default() -> Self {
        Self {
            initialized: false,
            automatic_target_exponent: SpatialScale::MAX.exponent() as f32,
            effective_target_exponent: SpatialScale::MAX.exponent() as f32,
            manual_bias_decades: 0.0,
        }
    }
}

impl NavigationPresentationState {
    pub const fn initialized(self) -> bool {
        self.initialized
    }
    pub const fn automatic_target_exponent(self) -> f32 {
        self.automatic_target_exponent
    }
    pub const fn effective_target_exponent(self) -> f32 {
        self.effective_target_exponent
    }
    pub const fn manual_bias_decades(self) -> f32 {
        self.manual_bias_decades
    }

    pub fn add_manual_bias(&mut self, delta: f32) {
        if delta.is_finite() {
            self.manual_bias_decades += delta;
        }
    }

    /// Resolve the view-owned target; manual bias never selects an interaction
    /// Scale or changes the subject's semantic navigation state.
    pub(super) fn resolve_target(
        &mut self,
        automatic: f32,
        profile: &NavigationPresentationProfile,
    ) -> f32 {
        self.automatic_target_exponent = automatic;
        let bias_limit = profile.maximum_manual_bias_decades.max(0.0);
        self.manual_bias_decades = self.manual_bias_decades.clamp(-bias_limit, bias_limit);
        self.effective_target_exponent = (automatic + self.manual_bias_decades).clamp(
            profile.minimum_scale.exponent() as f32,
            profile.maximum_scale.exponent() as f32,
        );
        self.effective_target_exponent
    }

    pub(super) fn mark_initialized(&mut self) {
        self.initialized = true;
    }
}

/// Compact end-to-end navigation/presentation telemetry.
#[derive(Resource, Debug, Clone, Copy)]
pub struct NavigationAudit {
    pub healthy: bool,
    pub subject: Option<Entity>,
    pub subject_scale: Option<SpatialScale>,
    pub primary_body: Option<Entity>,
    pub primary_clearance_metres: Option<f64>,
    pub navigation_source_scale: Option<SpatialScale>,
    pub characteristic_length_metres: f64,
    pub approach_active: bool,
    pub realization_target_scale: Option<SpatialScale>,
    pub view_exponent: f32,
    pub presentation_target_exponent: f32,
}

impl Default for NavigationAudit {
    fn default() -> Self {
        Self {
            healthy: false,
            subject: None,
            subject_scale: None,
            primary_body: None,
            primary_clearance_metres: None,
            navigation_source_scale: None,
            characteristic_length_metres: 0.0,
            approach_active: false,
            realization_target_scale: None,
            view_exponent: SpatialScale::MAX.exponent() as f32,
            presentation_target_exponent: SpatialScale::MAX.exponent() as f32,
        }
    }
}
