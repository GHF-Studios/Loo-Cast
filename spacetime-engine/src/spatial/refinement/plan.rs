//! Current-relative multiscale refinement planning.
//!
//! A refinement plan is runtime policy over canonical space. It decides which
//! Scale Slices participate, how a bounded working-set footprint tapers from the
//! requested tip toward coarser ancestors, which parent precedes each step, and
//! how much residency halo/priority bias accompanies each step.
//!
//! It does not know what a capability realizes inside those scopes.

use bevy::prelude::Vec3;

use super::super::{
    SPATIAL_SCALE_MAX, SPATIAL_SCALE_MIN, SpatialScale, UsfChartMask,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfRefinementStep {
    scale: SpatialScale,
    parent_scale: Option<SpatialScale>,
    half_extent_native: Vec3,
    residency_half_extent_native: Vec3,
    priority: i32,
}

impl UsfRefinementStep {
    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub const fn parent_scale(self) -> Option<SpatialScale> {
        self.parent_scale
    }

    pub const fn half_extent_native(self) -> Vec3 {
        self.half_extent_native
    }

    pub const fn residency_half_extent_native(self) -> Vec3 {
        self.residency_half_extent_native
    }

    pub const fn priority(self) -> i32 {
        self.priority
    }
}

/// Generic current-relative refinement policy.
///
/// With no explicit tip request, only the source's current Scale Slice is
/// requested. With an explicit tip, every supported Scale Slice from that tip
/// toward coarser context participates. The tip footprint is projected upward
/// through decimal USF scale units, so the branch narrows toward coarse context
/// and widens again approaching the current/requested tip.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfRefinementPlan {
    source_scale: SpatialScale,
    requested_tip: Option<SpatialScale>,
    supported_slices: UsfChartMask,
    tip_half_extent_native: Vec3,
    residency_halo_native: Vec3,
    tip_priority: i32,
    coarser_priority_step: i32,
}

impl UsfRefinementPlan {
    pub fn new(
        source_scale: SpatialScale,
        requested_tip: Option<SpatialScale>,
        supported_slices: UsfChartMask,
        tip_half_extent_native: Vec3,
        tip_priority: i32,
    ) -> Self {
        Self {
            source_scale,
            requested_tip,
            supported_slices,
            tip_half_extent_native: sanitize_extent(tip_half_extent_native),
            residency_halo_native: Vec3::ZERO,
            tip_priority,
            coarser_priority_step: 0,
        }
    }

    pub const fn source_scale(self) -> SpatialScale {
        self.source_scale
    }

    pub const fn tip_scale(self) -> SpatialScale {
        match self.requested_tip {
            Some(scale) => scale,
            None => self.source_scale,
        }
    }

    pub const fn supported_slices(self) -> UsfChartMask {
        self.supported_slices
    }

    pub fn with_residency_halo_native(mut self, halo: Vec3) -> Self {
        self.residency_halo_native = sanitize_extent(halo);
        self
    }

    /// Adds `step` priority for each supported level coarser than the tip.
    ///
    /// Zero preserves equal priority across the branch. Positive values favor
    /// coarse context; negative values favor detail once parent readiness allows
    /// it to participate.
    pub const fn with_coarser_priority_step(mut self, step: i32) -> Self {
        self.coarser_priority_step = step;
        self
    }

    pub fn requests_scale(self, scale: SpatialScale) -> bool {
        if !self.supported_slices.contains(scale) {
            return false;
        }

        match self.requested_tip {
            Some(tip) => scale >= tip,
            None => scale == self.source_scale,
        }
    }

    pub fn step(self, scale: SpatialScale) -> Option<UsfRefinementStep> {
        if !self.requests_scale(scale) {
            return None;
        }

        let tip = self.tip_scale();
        let exponent_delta =
            i32::from(tip.exponent()) - i32::from(scale.exponent());
        let factor = 10.0_f32.powi(exponent_delta);
        let half_extent_native = self.tip_half_extent_native * factor;

        let coarser_levels =
            i32::from(scale.exponent()) - i32::from(tip.exponent());
        let priority_delta = self
            .coarser_priority_step
            .saturating_mul(coarser_levels.max(0));

        Some(UsfRefinementStep {
            scale,
            parent_scale: self.parent_scale(scale),
            half_extent_native,
            residency_half_extent_native: half_extent_native
                + self.residency_halo_native,
            priority: self.tip_priority.saturating_add(priority_delta),
        })
    }

    /// Immediately coarser participating Scale Slice for `scale`.
    pub fn parent_scale(self, scale: SpatialScale) -> Option<SpatialScale> {
        if !self.requests_scale(scale) {
            return None;
        }

        let start = scale.exponent().checked_add(1)?;
        for raw in start..=SPATIAL_SCALE_MAX {
            let candidate =
                SpatialScale::new(raw).expect("bounded USF refinement scale");
            if self.requests_scale(candidate) {
                return Some(candidate);
            }
        }
        None
    }

    /// Deterministic coarse-context -> requested-tip traversal.
    pub fn steps_coarse_to_fine(
        self,
    ) -> impl Iterator<Item = UsfRefinementStep> {
        (SPATIAL_SCALE_MIN..=SPATIAL_SCALE_MAX)
            .rev()
            .filter_map(move |raw| {
                let scale =
                    SpatialScale::new(raw).expect("bounded USF refinement scale");
                self.step(scale)
            })
    }
}

fn sanitize_extent(value: Vec3) -> Vec3 {
    Vec3::new(
        sanitize_axis(value.x),
        sanitize_axis(value.y),
        sanitize_axis(value.z),
    )
}

fn sanitize_axis(value: f32) -> f32 {
    if value.is_finite() {
        value.abs()
    } else {
        0.0
    }
}
