//! Canonical construction and axis access.

use super::*;

impl Default for UsfPosition {
    fn default() -> Self {
        // Compatibility convenience only. Semantic systems should choose their
        // required leaf scale explicitly rather than treating this as a world origin.
        Self::zero(SpatialScale::ZERO)
    }
}

impl UsfPosition {
    pub const fn zero(leaf_scale: SpatialScale) -> Self {
        Self {
            digits: [IVec3::ZERO; SPATIAL_SCALE_COUNT],
            leaf_scale,
            offset: Vec3::ZERO,
        }
    }

    /// Convenience constructor for metre-authored local content.
    /// This is a unit adapter, not the canonical USF origin.
    pub fn from_metres_local(local_metres: Vec3) -> Result<Self, UsfPositionError> {
        Self::zero(SpatialScale::ZERO).translated_native(local_metres)
    }

    /// Builds one canonical hierarchical position from a bounded coordinate
    /// authored in `source_scale` native units, retaining f64 precision while
    /// refining down to `leaf_scale`.
    pub fn from_scale_native_f64(
        source_native: DVec3,
        source_scale: SpatialScale,
        leaf_scale: SpatialScale,
    ) -> Result<Self, UsfPositionError> {
        if !source_native.is_finite() {
            return Err(UsfPositionError::NonFiniteTranslation);
        }
        if leaf_scale > source_scale {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }

        let mut position = Self::zero(source_scale);
        let mut local = source_native;
        let chunk_size = f64::from(USF_CHUNK_NATIVE_SIZE);
        let half_chunk = chunk_size * 0.5;

        fn normalize_local_f64(
            position: &mut UsfPosition,
            local: &mut DVec3,
            chunk_size: f64,
            half_chunk: f64,
        ) -> Result<(), UsfPositionError> {
            for axis in 0..3 {
                let value = match axis {
                    0 => local.x,
                    1 => local.y,
                    2 => local.z,
                    _ => unreachable!(),
                };

                let carry_f = ((value + half_chunk) / chunk_size).floor();
                if carry_f < i64::MIN as f64 || carry_f > i64::MAX as f64 {
                    return Err(UsfPositionError::TranslationTooLarge);
                }

                let carry = carry_f as i64;
                let remainder = value - carry as f64 * chunk_size;
                match axis {
                    0 => local.x = remainder,
                    1 => local.y = remainder,
                    2 => local.z = remainder,
                    _ => unreachable!(),
                }
                position.add_chunk_carry(axis, carry)?;
            }
            Ok(())
        }

        normalize_local_f64(&mut position, &mut local, chunk_size, half_chunk)?;

        while position.leaf_scale > leaf_scale {
            let next = SpatialScale::new(position.leaf_scale.exponent() - 1)
                .expect("leaf target bounds authored-position refinement");
            position.leaf_scale = next;
            position.digits[next.index_from_top()] = IVec3::ZERO;
            local *= f64::from(USF_CHILD_CHUNKS_PER_AXIS);
            normalize_local_f64(&mut position, &mut local, chunk_size, half_chunk)?;
        }

        position.offset = Vec3::new(local.x as f32, local.y as f32, local.z as f32);
        position.normalize()?;
        Ok(position)
    }

    fn coordinate(&self, axis: usize) -> UsfCoordinate {
        let mut digits = [0_i8; SPATIAL_SCALE_COUNT];
        for raw_scale in self.leaf_scale.exponent()..=SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw_scale).expect("validated spatial scale");
            digits[scale.index_from_top()] = axis_i32(self.digit(scale), axis) as i8;
        }

        UsfCoordinate {
            digits,
            leaf_scale: self.leaf_scale,
            offset: axis_f32(self.offset, axis),
        }
    }

    pub fn x(&self) -> UsfCoordinate {
        self.coordinate(0)
    }

    pub fn y(&self) -> UsfCoordinate {
        self.coordinate(1)
    }

    pub fn z(&self) -> UsfCoordinate {
        self.coordinate(2)
    }

    pub const fn leaf_scale(&self) -> SpatialScale {
        self.leaf_scale
    }

    pub const fn offset(&self) -> Vec3 {
        self.offset
    }
}
