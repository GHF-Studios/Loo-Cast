//! Canonical physical motion projected into bounded Scale-Slice runtime charts.
//!
//! Physical velocity is semantic SI state. [`LinearVelocity`] is a disposable
//! chart-local projection used by physics kernels; changing Scale Slice must
//! never redefine the physical velocity by accident.

use avian3d::prelude::LinearVelocity;
use bevy::{math::DVec3, prelude::*};

use super::{SpatialScale, UsfScaleLayer};

#[derive(Component, Debug, Clone, Copy)]
pub struct UsfCanonicalMotion {
    velocity_metres_per_second: DVec3,
    canonical_authority: bool,
}

impl Default for UsfCanonicalMotion {
    fn default() -> Self {
        Self {
            velocity_metres_per_second: DVec3::ZERO,
            canonical_authority: false,
        }
    }
}

impl UsfCanonicalMotion {
    pub const fn velocity_metres_per_second(self) -> DVec3 {
        self.velocity_metres_per_second
    }

    pub fn speed_metres_per_second(self) -> f64 {
        self.velocity_metres_per_second.length()
    }

    pub const fn canonical_authority(self) -> bool {
        self.canonical_authority
    }

    pub fn set_canonical_authority(&mut self, authoritative: bool) {
        self.canonical_authority = authoritative;
    }

    pub fn set_velocity_metres_per_second(&mut self, velocity: DVec3) {
        assert!(velocity.is_finite(), "canonical velocity must be finite");
        self.velocity_metres_per_second = velocity;
    }

    pub fn stop(&mut self) {
        self.velocity_metres_per_second = DVec3::ZERO;
    }

    pub fn set_from_native_velocity(&mut self, scale: SpatialScale, velocity: Vec3) {
        let metres_per_native = scale.scale0_units_per_native();
        self.set_velocity_metres_per_second(
            DVec3::new(
                f64::from(velocity.x),
                f64::from(velocity.y),
                f64::from(velocity.z),
            ) * metres_per_native,
        );
    }

    pub fn native_velocity(self, scale: SpatialScale) -> Vec3 {
        let factor = scale.scale0_to_native_f64(1.0);
        let native = self.velocity_metres_per_second * factor;
        Vec3::new(
            saturating_f32(native.x),
            saturating_f32(native.y),
            saturating_f32(native.z),
        )
    }
}

fn saturating_f32(value: f64) -> f32 {
    if value.is_nan() {
        0.0
    } else {
        value.clamp(-f64::from(f32::MAX), f64::from(f32::MAX)) as f32
    }
}

/// Keeps canonical velocity synchronized while runtime physics is authoritative.
///
/// Canonical-first flight kernels set `canonical_authority`; detailed character
/// and collision solvers leave it false and publish their chart-local velocity
/// back through this adapter.
pub(in crate::spatial) fn sync_canonical_motion_from_runtime(
    mut motions: Query<(&UsfScaleLayer, &LinearVelocity, &mut UsfCanonicalMotion)>,
) {
    for (layer, velocity, mut motion) in &mut motions {
        if motion.canonical_authority() {
            continue;
        }
        motion.set_from_native_velocity(layer.scale(), velocity.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_velocity_survives_root_scale_projection() {
        let root = SpatialScale::MAX;
        let mut motion = UsfCanonicalMotion::default();
        motion.set_velocity_metres_per_second(DVec3::new(120.0, -3.5, 8.0));

        let native = motion.native_velocity(root);
        assert!(native.is_finite());
        assert_ne!(native, Vec3::ZERO);

        let mut reconstructed = UsfCanonicalMotion::default();
        reconstructed.set_from_native_velocity(root, native);
        let error =
            (reconstructed.velocity_metres_per_second() - motion.velocity_metres_per_second())
                .length();
        assert!(error < 1.0e-4);
    }
}
