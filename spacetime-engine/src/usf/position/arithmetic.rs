//! Precision-preserving scale conversion, translation and balanced carries.

use super::*;

impl UsfPosition {
    /// Re-expresses the same canonical location with a different finest
    /// resolved scale.
    ///
    /// This changes representation precision, not semantic position. Refining
    /// distributes the old leaf offset into newly-visible decimal child digits;
    /// coarsening folds child digits back into the parent-scale offset.
    pub fn reexpressed_at(mut self, target: SpatialScale) -> Result<Self, UsfPositionError> {
        while self.leaf_scale > target {
            let next = SpatialScale::new(self.leaf_scale.exponent() - 1)
                .expect("target scale bounds refinement");
            self.leaf_scale = next;
            self.digits[next.index_from_top()] = IVec3::ZERO;
            self.offset *= USF_CHILD_CHUNKS_PER_AXIS as f32;
            self.normalize()?;
        }

        while self.leaf_scale < target {
            let child = self.leaf_scale;
            let parent =
                SpatialScale::new(child.exponent() + 1).expect("target scale bounds coarsening");
            let digit = self.digit(child).as_vec3();
            self.digits[child.index_from_top()] = IVec3::ZERO;
            self.offset = digit * (USF_CHUNK_NATIVE_SIZE / USF_CHILD_CHUNKS_PER_AXIS as f32)
                + self.offset / USF_CHILD_CHUNKS_PER_AXIS as f32;
            self.leaf_scale = parent;
            self.normalize()?;
        }

        Ok(self)
    }

    /// Approximate absolute coordinates in units native to `scale`.
    ///
    /// Canonical identity remains the balanced hierarchical digit stack. This
    /// projection exists only for bounded runtime-chart metadata such as one
    /// scale layer's floating origin; it is never semantic authority.
    pub fn coordinate_at_scale_f64(&self, scale: SpatialScale) -> Result<DVec3, UsfPositionError> {
        let expressed = self.reexpressed_at(scale)?;
        let mut result = DVec3::ZERO;

        for axis in 0..3 {
            let mut chunk_coordinate = 0.0_f64;
            for raw_scale in (scale.exponent()..=SPATIAL_SCALE_MAX).rev() {
                let digit_scale =
                    SpatialScale::new(raw_scale).expect("validated spatial scale range");
                chunk_coordinate = chunk_coordinate * USF_CHILD_CHUNKS_PER_AXIS as f64
                    + axis_i32(expressed.digit(digit_scale), axis) as f64;
            }

            let component = chunk_coordinate * USF_CHUNK_NATIVE_SIZE as f64
                + axis_f32(expressed.offset, axis) as f64;
            match axis {
                0 => result.x = component,
                1 => result.y = component,
                2 => result.z = component,
                _ => unreachable!(),
            }
        }

        Ok(result)
    }

    pub fn digit(&self, scale: SpatialScale) -> IVec3 {
        self.digits[scale.index_from_top()]
    }

    pub fn translated_native(mut self, delta: Vec3) -> Result<Self, UsfPositionError> {
        if !delta.is_finite() {
            return Err(UsfPositionError::NonFiniteTranslation);
        }

        self.offset += delta;
        self.normalize()?;
        Ok(self)
    }

    /// Translates this canonical position by a bounded displacement expressed
    /// in units native to `scale` WITHOUT coarsening the position itself.
    ///
    /// Runtime physics/render coordinates are allowed to be floats inside one
    /// bounded scale-local chart. What is forbidden is routing the existing
    /// semantic identity through that coarser float representation.
    ///
    /// Only the displacement is expanded down to this position's leaf scale;
    /// all previously resolved finer digits remain intact.
    pub fn translated_at_scale(
        mut self,
        scale: SpatialScale,
        delta: Vec3,
    ) -> Result<Self, UsfPositionError> {
        if !delta.is_finite() {
            return Err(UsfPositionError::NonFiniteTranslation);
        }
        // Refining semantic precision is exact: it introduces finer decimal
        // slots without discarding any existing information. Coarsening the
        // semantic position is the operation we must never do implicitly.
        if scale < self.leaf_scale {
            self = self.reexpressed_at(scale)?;
        }
        if delta == Vec3::ZERO {
            return Ok(self);
        }

        let encoded_delta = UsfPosition::zero(scale)
            .translated_native(delta)?
            .reexpressed_at(self.leaf_scale)?;

        self.add_same_leaf_delta(encoded_delta)?;
        Ok(self)
    }

    /// Translates by an f64 displacement authored in one Scale Slice without
    /// routing physical motion through a chart-local f32 Transform.
    pub fn translated_at_scale_f64(
        mut self,
        scale: SpatialScale,
        delta: DVec3,
    ) -> Result<Self, UsfPositionError> {
        if !delta.is_finite() {
            return Err(UsfPositionError::NonFiniteTranslation);
        }
        if scale < self.leaf_scale {
            self = self.reexpressed_at(scale)?;
        }
        if delta == DVec3::ZERO {
            return Ok(self);
        }

        let encoded_delta = UsfPosition::from_scale_native_f64(delta, scale, self.leaf_scale)?;
        self.add_same_leaf_delta(encoded_delta)?;
        Ok(self)
    }

    /// Canonical SI-motion convenience: metres are S0 units by convention.
    /// S0 is a unit adapter here, not an architectural center or floor.
    pub fn translated_metres_f64(self, delta_metres: DVec3) -> Result<Self, UsfPositionError> {
        self.translated_at_scale_f64(SpatialScale::ZERO, delta_metres)
    }

    /// Adds a canonical displacement with the same leaf scale directly over
    /// the balanced-decimal hierarchy.
    fn add_same_leaf_delta(&mut self, delta: Self) -> Result<(), UsfPositionError> {
        if self.leaf_scale != delta.leaf_scale {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }

        let chunk_size = f64::from(USF_CHUNK_NATIVE_SIZE);
        let half_chunk = chunk_size * 0.5;

        for axis in 0..3 {
            let offset_sum =
                f64::from(axis_f32(self.offset, axis)) + f64::from(axis_f32(delta.offset, axis));

            let carry_f = ((offset_sum + half_chunk) / chunk_size).floor();
            if carry_f < i64::MIN as f64 || carry_f > i64::MAX as f64 {
                return Err(UsfPositionError::TranslationTooLarge);
            }

            let mut carry = carry_f as i64;
            let mut local = (offset_sum - carry as f64 * chunk_size) as f32;

            if local >= USF_LOCAL_MAX_EXCLUSIVE {
                local -= USF_CHUNK_NATIVE_SIZE;
                carry = carry
                    .checked_add(1)
                    .ok_or(UsfPositionError::TranslationTooLarge)?;
            } else if local < USF_LOCAL_MIN {
                local += USF_CHUNK_NATIVE_SIZE;
                carry = carry
                    .checked_sub(1)
                    .ok_or(UsfPositionError::TranslationTooLarge)?;
            }

            set_axis_f32(&mut self.offset, axis, local);

            for raw_scale in self.leaf_scale.exponent()..=SPATIAL_SCALE_MAX {
                let scale = SpatialScale::new(raw_scale).expect("validated spatial scale");
                let index = scale.index_from_top();

                let total = i64::from(axis_i32(self.digits[index], axis))
                    .checked_add(i64::from(axis_i32(delta.digits[index], axis)))
                    .and_then(|value| value.checked_add(carry))
                    .ok_or(UsfPositionError::TranslationTooLarge)?;

                let parent_carry = total
                    .checked_add(5)
                    .ok_or(UsfPositionError::TranslationTooLarge)?
                    .div_euclid(i64::from(USF_CHILD_CHUNKS_PER_AXIS));
                let digit = total - parent_carry * i64::from(USF_CHILD_CHUNKS_PER_AXIS);

                debug_assert!(digit >= i64::from(USF_BALANCED_DIGIT_MIN));
                debug_assert!(digit < i64::from(USF_BALANCED_DIGIT_MAX_EXCLUSIVE));
                set_axis_i32(&mut self.digits[index], axis, digit as i32);

                carry = parent_carry;
            }

            let _winding = carry;
        }

        Ok(())
    }

    /// Translates by exact whole units native to the current leaf scale.
    ///
    /// Unlike [`Self::translated_native`], this path never first collapses a
    /// potentially huge displacement into one floating-point vector. It is used
    /// when canonical identities are derived from integer-aligned representation
    /// addresses such as decimal voxel materialization chunks.
    pub fn translated_whole_native(mut self, delta: [i64; 3]) -> Result<Self, UsfPositionError> {
        let chunk_size = USF_CHUNK_NATIVE_SIZE as i64;

        for (axis, delta) in delta.into_iter().enumerate() {
            let chunk_carry = delta.div_euclid(chunk_size);
            let local_delta = delta.rem_euclid(chunk_size) as f32;

            self.add_chunk_carry(axis, chunk_carry)?;
            let offset = axis_f32(self.offset, axis) + local_delta;
            set_axis_f32(&mut self.offset, axis, offset);
        }

        self.normalize()?;
        Ok(self)
    }

    /// Measures this position from `origin` in units native to the shared leaf
    /// scale, but only when every resulting axis lies inside `max_abs`.
    ///
    /// This is the deliberate inverse of projecting canonical space into a
    /// bounded local chart. It never constructs one universe-wide float
    /// coordinate: decimal digits are subtracted and normalized exactly before
    /// the result is projected into the requested bounded float chart.
    pub(super) fn normalize(&mut self) -> Result<(), UsfPositionError> {
        // Do quotient/remainder arithmetic in f64 even though the stored local
        // offset is f32. A large chunk carry cannot in general be represented
        // exactly as f32; converting that integer carry back to f32 before
        // subtraction can leave a bogus remainder outside [-500, 500).
        let chunk_size = f64::from(USF_CHUNK_NATIVE_SIZE);
        let half_chunk = chunk_size * 0.5;

        for axis in 0..3 {
            let offset = axis_f32(self.offset, axis);
            if !offset.is_finite() {
                return Err(UsfPositionError::NonFiniteTranslation);
            }

            let offset_f64 = f64::from(offset);
            let carry_f = ((offset_f64 + half_chunk) / chunk_size).floor();
            if carry_f < i64::MIN as f64 || carry_f > i64::MAX as f64 {
                return Err(UsfPositionError::TranslationTooLarge);
            }

            let mut carry = carry_f as i64;
            let mut local = (offset_f64 - carry as f64 * chunk_size) as f32;

            // The exact f64 remainder is in [-500, 500), but the final f32 cast
            // may round a value immediately below +500 to exactly +500. Repair
            // the half-open canonical interval and keep the digit carry in sync.
            if local >= USF_LOCAL_MAX_EXCLUSIVE {
                local -= USF_CHUNK_NATIVE_SIZE;
                carry = carry
                    .checked_add(1)
                    .ok_or(UsfPositionError::TranslationTooLarge)?;
            } else if local < USF_LOCAL_MIN {
                local += USF_CHUNK_NATIVE_SIZE;
                carry = carry
                    .checked_sub(1)
                    .ok_or(UsfPositionError::TranslationTooLarge)?;
            }

            set_axis_f32(&mut self.offset, axis, local);
            self.add_chunk_carry(axis, carry)?;
        }

        debug_assert!(self.offset.cmpge(Vec3::splat(USF_LOCAL_MIN)).all());
        debug_assert!(
            self.offset
                .cmplt(Vec3::splat(USF_LOCAL_MAX_EXCLUSIVE))
                .all()
        );
        Ok(())
    }

    pub(super) fn add_chunk_carry(
        &mut self,
        axis: usize,
        mut carry: i64,
    ) -> Result<(), UsfPositionError> {
        if carry == 0 {
            return Ok(());
        }

        let mut raw_scale = self.leaf_scale.exponent();
        loop {
            let scale = SpatialScale::new(raw_scale).expect("leaf scale is valid");
            let index = scale.index_from_top();
            let current = axis_i32(self.digits[index], axis) as i64;
            let total = current
                .checked_add(carry)
                .ok_or(UsfPositionError::TranslationTooLarge)?;
            let parent_carry = total
                .checked_add(5)
                .ok_or(UsfPositionError::TranslationTooLarge)?
                .div_euclid(USF_CHILD_CHUNKS_PER_AXIS as i64);
            let digit = total - parent_carry * USF_CHILD_CHUNKS_PER_AXIS as i64;

            debug_assert!(digit >= USF_BALANCED_DIGIT_MIN as i64);
            debug_assert!(digit < USF_BALANCED_DIGIT_MAX_EXCLUSIVE as i64);
            set_axis_i32(&mut self.digits[index], axis, digit as i32);

            if raw_scale == SPATIAL_SCALE_MAX {
                // The root is the finite-universe wrap seam. Carry/borrow beyond
                // it is intentionally discarded, producing canonical world wrap
                // instead of a terminal arithmetic error.
                return Ok(());
            }

            carry = parent_carry;
            if carry == 0 {
                return Ok(());
            }
            raw_scale += 1;
        }
    }
}
