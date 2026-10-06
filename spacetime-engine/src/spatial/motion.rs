//! Canonical physical motion projected into bounded Scale-Slice runtime charts.
//!
//! Physical velocity is semantic SI state. [`LinearVelocity`] is a disposable
//! chart-local projection used by physics kernels; changing Scale Slice must
//! never redefine the physical velocity by accident.

use avian3d::prelude::LinearVelocity;
use bevy::{math::DVec3, prelude::*, time::Virtual};

use super::{SpatialScale, UsfScaleLayer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsfMotionAuthority {
    RuntimePhysics,
    CanonicalKinematics,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct UsfCanonicalMotion {
    velocity_metres_per_second: DVec3,
    angular_velocity_radians_per_second: DVec3,
    epoch_seconds: f64,
    authority: UsfMotionAuthority,
}

impl Default for UsfCanonicalMotion {
    fn default() -> Self {
        Self {
            velocity_metres_per_second: DVec3::ZERO,
            angular_velocity_radians_per_second: DVec3::ZERO,
            epoch_seconds: 0.0,
            authority: UsfMotionAuthority::RuntimePhysics,
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

    pub const fn angular_velocity_radians_per_second(self) -> DVec3 {
        self.angular_velocity_radians_per_second
    }
    pub const fn epoch_seconds(self) -> f64 {
        self.epoch_seconds
    }
    pub const fn authority(self) -> UsfMotionAuthority {
        self.authority
    }
    pub fn canonical_kinematic_at_rest() -> Self {
        Self {
            authority: UsfMotionAuthority::CanonicalKinematics,
            ..Self::default()
        }
    }

    pub const fn is_canonical_kinematic(self) -> bool {
        matches!(self.authority, UsfMotionAuthority::CanonicalKinematics)
    }

    pub fn set_authority(&mut self, authority: UsfMotionAuthority) {
        self.authority = authority;
    }

    pub fn set_velocity_metres_per_second(&mut self, velocity: DVec3) {
        assert!(velocity.is_finite(), "canonical velocity must be finite");
        self.velocity_metres_per_second = velocity;
    }

    pub fn set_angular_velocity_radians_per_second(&mut self, value: DVec3) {
        assert!(
            value.is_finite(),
            "canonical angular velocity must be finite"
        );
        self.angular_velocity_radians_per_second = value;
    }
    pub fn set_epoch_seconds(&mut self, value: f64) {
        assert!(value.is_finite(), "canonical motion epoch must be finite");
        self.epoch_seconds = value;
    }

    pub fn stop(&mut self) {
        self.velocity_metres_per_second = DVec3::ZERO;
    }

    pub fn set_from_native_velocity(&mut self, scale: SpatialScale, velocity: Vec3) {
        let metres_per_native = scale.metres_per_native();
        self.set_velocity_metres_per_second(
            DVec3::new(
                f64::from(velocity.x),
                f64::from(velocity.y),
                f64::from(velocity.z),
            ) * metres_per_native,
        );
    }

    pub fn native_velocity(self, scale: SpatialScale) -> Vec3 {
        let factor = scale.metres_to_native_f64(1.0);
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
    time: Res<Time<Virtual>>,
    mut motions: Query<(&UsfScaleLayer, &LinearVelocity, &mut UsfCanonicalMotion)>,
) {
    for (layer, velocity, mut motion) in &mut motions {
        if motion.is_canonical_kinematic() {
            continue;
        }
        motion.set_from_native_velocity(layer.scale(), velocity.0);
        motion.set_epoch_seconds(time.elapsed_secs_f64());
    }
}
