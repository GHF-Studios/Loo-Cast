//! Bounded relative projections that retain canonical position authority.

use super::*;

impl UsfPosition {
    pub fn relative_native_bounded(
        &self,
        origin: &Self,
        max_abs: f32,
    ) -> Result<Vec3, UsfPositionError> {
        let mut delta = Vec3::ZERO;
        for axis in 0..3 {
            set_axis_f32(
                &mut delta,
                axis,
                self.relative_native_axis_bounded(origin, axis, max_abs)?,
            );
        }
        Ok(delta)
    }

    /// Measures this position from `origin` in units native to `scale`.
    ///
    /// Unlike measuring in leaf units and dividing afterward, this accumulates
    /// the balanced-decimal hierarchy directly at the requested scale. A galaxy-
    /// scale view therefore never constructs a galaxy-sized metre coordinate.
    pub fn relative_at_scale_bounded(
        &self,
        origin: &Self,
        scale: SpatialScale,
        max_abs: f32,
    ) -> Result<Vec3, UsfPositionError> {
        // Canonical points may carry different resolved leaf scales. For an
        // explicitly requested coarser/equal projection scale, refine the
        // coarser operand to the finer leaf first. Refinement is exact in the
        // USF hierarchy; it never discards semantic information.
        let common_leaf = self.leaf_scale.min(origin.leaf_scale);
        if scale < common_leaf {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }

        let lhs = if self.leaf_scale == common_leaf {
            *self
        } else {
            self.reexpressed_at(common_leaf)?
        };
        let rhs = if origin.leaf_scale == common_leaf {
            *origin
        } else {
            origin.reexpressed_at(common_leaf)?
        };

        let mut delta = Vec3::ZERO;
        for axis in 0..3 {
            set_axis_f32(
                &mut delta,
                axis,
                lhs.relative_axis_at_scale_bounded(&rhs, scale, axis, max_abs)?,
            );
        }
        Ok(delta)
    }

    /// Measures this position from `origin` directly into an f64 scale-local
    /// chart without collapsing the canonical Scale Stack into one global float.
    ///
    /// This is a high-dynamic-range adapter for choosing a coarse direction or
    /// anchor. Fine simulation still projects into a bounded local f32 chart.
    pub fn relative_at_scale_bounded_f64(
        &self,
        origin: &Self,
        scale: SpatialScale,
        max_abs: f64,
    ) -> Result<DVec3, UsfPositionError> {
        let common_leaf = self.leaf_scale.min(origin.leaf_scale);
        if scale < common_leaf {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }
        if !max_abs.is_finite() {
            return Err(UsfPositionError::NonFiniteTranslation);
        }

        let lhs = if self.leaf_scale == common_leaf {
            *self
        } else {
            self.reexpressed_at(common_leaf)?
        };
        let rhs = if origin.leaf_scale == common_leaf {
            *origin
        } else {
            origin.reexpressed_at(common_leaf)?
        };

        Ok(DVec3::new(
            lhs.relative_axis_at_scale_bounded_f64(&rhs, scale, 0, max_abs)?,
            lhs.relative_axis_at_scale_bounded_f64(&rhs, scale, 1, max_abs)?,
            lhs.relative_axis_at_scale_bounded_f64(&rhs, scale, 2, max_abs)?,
        ))
    }

    /// Projects a canonical displacement onto an integer lattice local to
    /// `origin` without ever flattening the Scale Stack into a giant float.
    ///
    /// `cell_size_native` must exactly divide one USF chunk (1000 native
    /// units). High-order chunk displacement stays integer; only the bounded
    /// canonical leaf offsets participate in the final Euclidean floor.
    ///
    /// This is a representation adapter, not alternate spatial authority.
    pub(crate) fn relative_native_lattice_cell(
        &self,
        origin: &Self,
        cell_size_native: i64,
    ) -> Result<[i64; 3], UsfPositionError> {
        if self.leaf_scale != origin.leaf_scale {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }
        assert!(
            cell_size_native > 0 && i64::from(USF_CHUNK_NATIVE_SIZE as i32) % cell_size_native == 0,
            "local lattice cell size must exactly divide one USF chunk"
        );

        let cells_per_chunk =
            i128::from(USF_CHUNK_NATIVE_SIZE as i32) / i128::from(cell_size_native);
        let mut result = [0_i64; 3];

        for axis in 0..3 {
            let (normalized, count) = self.normalized_axis_difference(origin, axis)?;
            let mut chunk_delta = 0_i128;
            for &digit in normalized[..count].iter().rev() {
                chunk_delta = chunk_delta
                    .checked_mul(i128::from(USF_CHILD_CHUNKS_PER_AXIS))
                    .and_then(|value| value.checked_add(i128::from(digit)))
                    .ok_or(UsfPositionError::TranslationTooLarge)?;
            }

            let offset_delta =
                f64::from(axis_f32(self.offset, axis)) - f64::from(axis_f32(origin.offset, axis));
            let local_cell = (offset_delta / cell_size_native as f64).floor() as i128;
            let total = chunk_delta
                .checked_mul(cells_per_chunk)
                .and_then(|value| value.checked_add(local_cell))
                .ok_or(UsfPositionError::TranslationTooLarge)?;
            result[axis] =
                i64::try_from(total).map_err(|_| UsfPositionError::TranslationTooLarge)?;
        }

        Ok(result)
    }

    /// Axis-local form used by bounded algorithms that intentionally do not
    /// require the other two coordinates to fit the same local chart.
    /// Measures one scalar canonical-axis displacement in shared-leaf native units.
    pub fn relative_native_axis_bounded(
        &self,
        origin: &Self,
        axis: usize,
        max_abs: f32,
    ) -> Result<f32, UsfPositionError> {
        if self.leaf_scale != origin.leaf_scale {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }
        if !max_abs.is_finite() {
            return Err(UsfPositionError::NonFiniteTranslation);
        }

        // Subtract then re-normalize the balanced decimal stack from low to
        // high before accumulating it. This is important for nearby positions
        // separated by a very long carry chain: their raw high-to-low digit
        // difference can look enormous until the low digits cancel the carry.
        //
        // The carry beyond the root is deliberately discarded. The finite USF
        // stack wraps there, so opposite root edges are adjacent in canonical
        // space rather than separated by a fatal overflow boundary.
        let (normalized, count) = self.normalized_axis_difference(origin, axis)?;

        let mut chunk_delta = 0_i128;
        for &digit in normalized[..count].iter().rev() {
            chunk_delta = chunk_delta
                .checked_mul(i128::from(USF_CHILD_CHUNKS_PER_AXIS))
                .and_then(|value| value.checked_add(i128::from(digit)))
                .ok_or(UsfPositionError::RelativePositionOutsideBound)?;
        }

        let component = chunk_delta as f64 * USF_CHUNK_NATIVE_SIZE as f64
            + f64::from(axis_f32(self.offset, axis) - axis_f32(origin.offset, axis));
        if component.abs() > f64::from(max_abs.max(0.0)) {
            return Err(UsfPositionError::RelativePositionOutsideBound);
        }
        Ok(component as f32)
    }

    fn relative_axis_at_scale_bounded(
        &self,
        origin: &Self,
        scale: SpatialScale,
        axis: usize,
        max_abs: f32,
    ) -> Result<f32, UsfPositionError> {
        if self.leaf_scale != origin.leaf_scale || scale < self.leaf_scale {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }
        if !max_abs.is_finite() {
            return Err(UsfPositionError::NonFiniteTranslation);
        }

        let (normalized, count) = self.normalized_axis_difference(origin, axis)?;
        let requested_index = (scale.exponent() - self.leaf_scale.exponent()) as usize;
        debug_assert!(requested_index < count);

        let mut chunk_delta = 0_i128;
        for &digit in normalized[requested_index..count].iter().rev() {
            chunk_delta = chunk_delta
                .checked_mul(i128::from(USF_CHILD_CHUNKS_PER_AXIS))
                .and_then(|value| value.checked_add(i128::from(digit)))
                .ok_or(UsfPositionError::RelativePositionOutsideBound)?;
        }

        let mut component = chunk_delta as f64 * USF_CHUNK_NATIVE_SIZE as f64;
        let mut weight = USF_CHUNK_NATIVE_SIZE as f64 / f64::from(USF_CHILD_CHUNKS_PER_AXIS);
        for index in (0..requested_index).rev() {
            component += f64::from(normalized[index]) * weight;
            weight *= 0.1;
        }

        let leaf_native_to_requested = self.leaf_scale.native_to_native_factor(scale);
        component += f64::from(axis_f32(self.offset, axis) - axis_f32(origin.offset, axis))
            * leaf_native_to_requested;

        if component.abs() > f64::from(max_abs.max(0.0)) {
            return Err(UsfPositionError::RelativePositionOutsideBound);
        }
        Ok(component as f32)
    }

    fn relative_axis_at_scale_bounded_f64(
        &self,
        origin: &Self,
        scale: SpatialScale,
        axis: usize,
        max_abs: f64,
    ) -> Result<f64, UsfPositionError> {
        if self.leaf_scale != origin.leaf_scale || scale < self.leaf_scale {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }

        let (normalized, count) = self.normalized_axis_difference(origin, axis)?;
        let requested_index = (scale.exponent() - self.leaf_scale.exponent()) as usize;
        debug_assert!(requested_index < count);

        let mut chunk_delta = 0_i128;
        for &digit in normalized[requested_index..count].iter().rev() {
            chunk_delta = chunk_delta
                .checked_mul(i128::from(USF_CHILD_CHUNKS_PER_AXIS))
                .and_then(|value| value.checked_add(i128::from(digit)))
                .ok_or(UsfPositionError::RelativePositionOutsideBound)?;
        }

        let mut component = chunk_delta as f64 * USF_CHUNK_NATIVE_SIZE as f64;
        let mut weight = USF_CHUNK_NATIVE_SIZE as f64 / f64::from(USF_CHILD_CHUNKS_PER_AXIS);
        for index in (0..requested_index).rev() {
            component += f64::from(normalized[index]) * weight;
            weight *= 0.1;
        }

        let leaf_native_to_requested = self.leaf_scale.native_to_native_factor(scale);
        component += f64::from(axis_f32(self.offset, axis) - axis_f32(origin.offset, axis))
            * leaf_native_to_requested;

        if component.abs() > max_abs.max(0.0) {
            return Err(UsfPositionError::RelativePositionOutsideBound);
        }
        Ok(component)
    }

    /// Returns whether the wrapped canonical displacement on one axis is on
    /// the negative side of the origin. This is useful for bounded algorithms
    /// that need an orientation even when the magnitude is intentionally not
    /// projected into `f32`.
    pub fn relative_native_axis_is_negative(
        &self,
        origin: &Self,
        axis: usize,
    ) -> Result<bool, UsfPositionError> {
        let (normalized, count) = self.normalized_axis_difference(origin, axis)?;
        for &digit in normalized[..count].iter().rev() {
            if digit != 0 {
                return Ok(digit < 0);
            }
        }

        Ok(axis_f32(self.offset, axis) < axis_f32(origin.offset, axis))
    }

    fn normalized_axis_difference(
        &self,
        origin: &Self,
        axis: usize,
    ) -> Result<([i8; SPATIAL_SCALE_COUNT], usize), UsfPositionError> {
        if self.leaf_scale != origin.leaf_scale {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }

        let mut normalized = [0_i8; SPATIAL_SCALE_COUNT];
        let mut count = 0_usize;
        let mut carry = 0_i32;
        for raw_scale in self.leaf_scale.exponent()..=SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw_scale).expect("range is validated");
            let total =
                axis_i32(self.digit(scale), axis) - axis_i32(origin.digit(scale), axis) + carry;
            let parent_carry = (total + 5).div_euclid(USF_CHILD_CHUNKS_PER_AXIS);
            let digit = total - parent_carry * USF_CHILD_CHUNKS_PER_AXIS;
            debug_assert!(digit >= USF_BALANCED_DIGIT_MIN);
            debug_assert!(digit < USF_BALANCED_DIGIT_MAX_EXCLUSIVE);
            normalized[count] = digit as i8;
            count += 1;
            carry = parent_carry;
        }

        // `carry` is the winding number across the finite root. Canonical USF
        // position arithmetic intentionally wraps, so it is not another digit.
        let _winding = carry;
        Ok((normalized, count))
    }
}
