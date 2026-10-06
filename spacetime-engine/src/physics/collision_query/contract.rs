//! Canonical swept-collision contracts and one-frame query journal.
//!
//! Conservative candidates are observations. No provider owns a physical
//! impulse merely because it produced a candidate.

use crate::spatial::{SpatialScale, UsfPosition, UsfPositionError};
use bevy::{math::DVec3, prelude::*};

/// Policy demand for conservative swept-collision query support.
///
/// This component does not itself select voxel chunks or a Scale Slice. Query
/// providers consume it together with canonical position/motion and decide what
/// sparse hierarchy/caches are required to meet the requested error bound.
///
/// `lookahead_seconds` may exceed one fixed tick so residency can prepare ahead
/// of motion. The actual collision transaction still resolves one concrete
/// [`UsfCanonicalSweep`] at a time.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct UsfCollisionQueryDemand {
    lookahead_seconds: f64,
    bounding_radius_metres: f64,
    target_error_metres: f64,
}

impl UsfCollisionQueryDemand {
    pub fn new(
        lookahead_seconds: f64,
        bounding_radius_metres: f64,
        target_error_metres: f64,
    ) -> Self {
        assert!(
            lookahead_seconds.is_finite() && lookahead_seconds > 0.0,
            "collision-query lookahead must be finite and positive"
        );
        assert!(
            bounding_radius_metres.is_finite() && bounding_radius_metres >= 0.0,
            "collision-query bounding radius must be finite and non-negative"
        );
        assert!(
            target_error_metres.is_finite() && target_error_metres > 0.0,
            "collision-query target error must be finite and positive"
        );

        Self {
            lookahead_seconds,
            bounding_radius_metres,
            target_error_metres,
        }
    }

    pub const fn lookahead_seconds(self) -> f64 {
        self.lookahead_seconds
    }

    pub const fn bounding_radius_metres(self) -> f64 {
        self.bounding_radius_metres
    }

    pub const fn target_error_metres(self) -> f64 {
        self.target_error_metres
    }
}

/// One authoritative physical-motion segment to test for collision.
///
/// `displacement_metres` is semantic SI displacement over `duration_seconds`.
/// The sweep is independent from whichever bounded Scale Slice/backend happens
/// to answer it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfCanonicalSweep {
    subject: Entity,
    start: UsfPosition,
    displacement_metres: DVec3,
    duration_seconds: f64,
    bounding_radius_metres: f64,
}

impl UsfCanonicalSweep {
    pub fn new(
        subject: Entity,
        start: UsfPosition,
        displacement_metres: DVec3,
        duration_seconds: f64,
        bounding_radius_metres: f64,
    ) -> Self {
        assert!(
            displacement_metres.is_finite(),
            "canonical collision sweep displacement must be finite"
        );
        assert!(
            duration_seconds.is_finite() && duration_seconds > 0.0,
            "canonical collision sweep duration must be finite and positive"
        );
        assert!(
            bounding_radius_metres.is_finite() && bounding_radius_metres >= 0.0,
            "canonical collision sweep bounding radius must be finite and non-negative"
        );

        Self {
            subject,
            start,
            displacement_metres,
            duration_seconds,
            bounding_radius_metres,
        }
    }

    pub const fn subject(self) -> Entity {
        self.subject
    }

    pub const fn start(self) -> UsfPosition {
        self.start
    }

    pub const fn displacement_metres(self) -> DVec3 {
        self.displacement_metres
    }

    pub const fn duration_seconds(self) -> f64 {
        self.duration_seconds
    }

    pub const fn bounding_radius_metres(self) -> f64 {
        self.bounding_radius_metres
    }

    pub fn velocity_metres_per_second(self) -> DVec3 {
        self.displacement_metres / self.duration_seconds
    }

    pub fn end(self) -> Result<UsfPosition, UsfPositionError> {
        self.start.translated_metres_f64(self.displacement_metres)
    }

    pub fn position_at(self, fraction: f64) -> Result<UsfPosition, UsfPositionError> {
        let fraction = fraction.clamp(0.0, 1.0);
        self.start
            .translated_metres_f64(self.displacement_metres * fraction)
    }
}

/// Closed normalized interval over one [`UsfCanonicalSweep`].
///
/// `0` is the sweep start and `1` is the requested end. Coarse providers should
/// conservatively *contain* the true impact fraction; refinement narrows this
/// interval rather than applying a response.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfSweepInterval {
    minimum: f64,
    maximum: f64,
}

impl UsfSweepInterval {
    pub const WHOLE: Self = Self {
        minimum: 0.0,
        maximum: 1.0,
    };

    pub fn new(minimum: f64, maximum: f64) -> Option<Self> {
        if !minimum.is_finite()
            || !maximum.is_finite()
            || minimum < 0.0
            || maximum > 1.0
            || minimum > maximum
        {
            return None;
        }
        Some(Self { minimum, maximum })
    }

    pub const fn minimum(self) -> f64 {
        self.minimum
    }

    pub const fn maximum(self) -> f64 {
        self.maximum
    }

    pub fn width(self) -> f64 {
        self.maximum - self.minimum
    }
}

/// Conservative collision possibility emitted by one query representation.
///
/// This is *evidence*, never response authority. Many candidates at different
/// scales may describe the same physical encounter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfCollisionCandidate {
    authority: Entity,
    scale: SpatialScale,
    interval: UsfSweepInterval,
    error_bound_metres: f64,
}

impl UsfCollisionCandidate {
    pub fn new(
        authority: Entity,
        scale: SpatialScale,
        interval: UsfSweepInterval,
        error_bound_metres: f64,
    ) -> Self {
        assert!(
            error_bound_metres.is_finite() && error_bound_metres >= 0.0,
            "collision candidate error bound must be finite and non-negative"
        );
        Self {
            authority,
            scale,
            interval,
            error_bound_metres,
        }
    }

    pub const fn authority(self) -> Entity {
        self.authority
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub const fn interval(self) -> UsfSweepInterval {
        self.interval
    }

    pub const fn error_bound_metres(self) -> f64 {
        self.error_bound_metres
    }
}

/// Refined geometric result for one canonical sweep.
///
/// A resolution is still only a query result. A higher-level collision episode
/// transaction decides whether to accept it and is the *only* layer allowed to
/// mutate canonical motion.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfCollisionResolution {
    authority: Entity,
    scale: SpatialScale,
    toi_fraction: f64,
    contact_position: UsfPosition,
    outward_normal: DVec3,
    error_bound_metres: f64,
}

impl UsfCollisionResolution {
    pub fn new(
        authority: Entity,
        scale: SpatialScale,
        toi_fraction: f64,
        contact_position: UsfPosition,
        outward_normal: DVec3,
        error_bound_metres: f64,
    ) -> Self {
        assert!(
            toi_fraction.is_finite() && (0.0..=1.0).contains(&toi_fraction),
            "collision resolution TOI must be a finite sweep fraction"
        );
        assert!(
            outward_normal.is_finite() && outward_normal.length_squared() > 0.0,
            "collision resolution normal must be finite and non-zero"
        );
        assert!(
            error_bound_metres.is_finite() && error_bound_metres >= 0.0,
            "collision resolution error bound must be finite and non-negative"
        );

        Self {
            authority,
            scale,
            toi_fraction,
            contact_position,
            outward_normal: outward_normal.normalize(),
            error_bound_metres,
        }
    }

    pub const fn authority(self) -> Entity {
        self.authority
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub const fn toi_fraction(self) -> f64 {
        self.toi_fraction
    }

    pub const fn contact_position(self) -> UsfPosition {
        self.contact_position
    }

    pub const fn outward_normal(self) -> DVec3 {
        self.outward_normal
    }

    pub const fn error_bound_metres(self) -> f64 {
        self.error_bound_metres
    }
}
