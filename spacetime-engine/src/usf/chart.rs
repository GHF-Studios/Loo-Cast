//! Pure bounded chart algebra over canonical USF space.

use glam::{DVec3, Vec3};

use super::{SpatialScale, UsfPosition, UsfPositionError};

/// One bounded numeric chart over canonical USF space.
///
/// A chart is only an origin plus the Scale Slice whose native units are used
/// for local coordinates. It has no ECS/backend/runtime ownership.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfChart {
    origin: UsfPosition,
    scale: SpatialScale,
}

impl UsfChart {
    pub const fn new(origin: UsfPosition, scale: SpatialScale) -> Self {
        Self { origin, scale }
    }

    pub const fn origin(self) -> UsfPosition {
        self.origin
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub fn project(
        self,
        position: &UsfPosition,
        max_abs: f32,
    ) -> Result<Vec3, UsfPositionError> {
        position.relative_at_scale_bounded(&self.origin, self.scale, max_abs)
    }

    pub fn project_f64(
        self,
        position: &UsfPosition,
        max_abs: f64,
    ) -> Result<DVec3, UsfPositionError> {
        position.relative_at_scale_bounded_f64(&self.origin, self.scale, max_abs)
    }

    pub fn unproject(self, local: Vec3) -> Result<UsfPosition, UsfPositionError> {
        self.origin.translated_at_scale(self.scale, local)
    }

    pub fn unproject_f64(self, local: DVec3) -> Result<UsfPosition, UsfPositionError> {
        self.origin.translated_at_scale_f64(self.scale, local)
    }
}

/// One canonical chart-origin displacement authored in a Scale Slice's native units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfChartDelta {
    source_scale: SpatialScale,
    local_shift: Vec3,
}

impl UsfChartDelta {
    pub const fn new(source_scale: SpatialScale, local_shift: Vec3) -> Self {
        Self {
            source_scale,
            local_shift,
        }
    }

    pub const fn source_scale(self) -> SpatialScale {
        self.source_scale
    }

    pub const fn local_shift(self) -> Vec3 {
        self.local_shift
    }

    pub fn at_scale(self, target_scale: SpatialScale) -> Result<Vec3, UsfPositionError> {
        if !self.local_shift.is_finite() {
            return Err(UsfPositionError::NonFiniteTranslation);
        }

        let exponent_delta =
            i32::from(self.source_scale.exponent()) - i32::from(target_scale.exponent());
        let factor = 10.0_f64.powi(exponent_delta);
        let converted = [
            f64::from(self.local_shift.x) * factor,
            f64::from(self.local_shift.y) * factor,
            f64::from(self.local_shift.z) * factor,
        ];

        if converted
            .iter()
            .any(|value| !value.is_finite() || value.abs() > f64::from(f32::MAX))
        {
            return Err(UsfPositionError::TranslationTooLarge);
        }

        Ok(Vec3::new(
            converted[0] as f32,
            converted[1] as f32,
            converted[2] as f32,
        ))
    }

    pub fn apply_to(self, origin: UsfPosition) -> Result<UsfPosition, UsfPositionError> {
        origin.translated_at_scale(self.source_scale, self.local_shift)
    }
}
