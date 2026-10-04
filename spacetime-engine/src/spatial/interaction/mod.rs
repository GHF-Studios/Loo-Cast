//! Controlled-subject interaction focus over the persistent Scale Slice stack.

use bevy::prelude::*;

use super::{SpatialScale, UsfScaleRoleMask};

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct UsfInteractionProjection;

/// Authored interaction Scale affinity of one controllable runtime manifestation.
///
/// This is semantic/control policy, not visual LOD and not an automatic
/// distance/clearance heuristic. Capability coverage may delay entry into this
/// Scale, but it never chooses a different target.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct UsfInteractionScaleAffinity {
    scale: SpatialScale,
    required_roles: UsfScaleRoleMask,
    coverage_radius_native: f32,
}

impl UsfInteractionScaleAffinity {
    pub const fn new(scale: SpatialScale) -> Self {
        Self {
            scale,
            required_roles: UsfScaleRoleMask::NONE,
            coverage_radius_native: 0.0,
        }
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub const fn required_roles(self) -> UsfScaleRoleMask {
        self.required_roles
    }

    pub const fn coverage_radius_native(self) -> f32 {
        self.coverage_radius_native
    }

    pub const fn requiring(mut self, roles: UsfScaleRoleMask) -> Self {
        self.required_roles = roles;
        self
    }

    pub fn with_coverage_radius_native(mut self, radius_native: f32) -> Self {
        self.coverage_radius_native =
            if radius_native.is_finite() { radius_native.max(0.0) } else { 0.0 };
        self
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct UsfPrimaryInteractionSlice {
    current: SpatialScale,
    requested: Option<SpatialScale>,
}

impl Default for UsfPrimaryInteractionSlice {
    fn default() -> Self {
        Self {
            current: SpatialScale::MAX,
            requested: None,
        }
    }
}

impl UsfPrimaryInteractionSlice {
    pub const fn scale(self) -> SpatialScale {
        self.current
    }

    pub const fn requested_scale(self) -> Option<SpatialScale> {
        self.requested
    }

    pub const fn target_scale(self) -> SpatialScale {
        match self.requested {
            Some(scale) => scale,
            None => self.current,
        }
    }

    pub const fn handoff_pending(self) -> bool {
        self.requested.is_some()
    }

    pub(crate) fn request_handoff(&mut self, scale: SpatialScale) {
        self.requested = (scale != self.current).then_some(scale);
    }

    pub(crate) fn cancel_handoff(&mut self) {
        self.requested = None;
    }

    pub(crate) fn complete_handoff(&mut self, scale: SpatialScale) {
        self.current = scale;
        self.requested = None;
    }
}
