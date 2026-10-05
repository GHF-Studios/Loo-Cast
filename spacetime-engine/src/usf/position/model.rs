//! Canonical position value, balanced digit arithmetic, and bounded projection.

use super::decimal::{format_decimal_terms, push_float_decimal_terms};
use crate::usf::{
    SPATIAL_SCALE_COUNT, SPATIAL_SCALE_MAX, SpatialScale, USF_BALANCED_DIGIT_MAX_EXCLUSIVE,
    USF_BALANCED_DIGIT_MIN, USF_CHILD_CHUNKS_PER_AXIS, USF_CHUNK_NATIVE_SIZE,
    USF_LOCAL_MAX_EXCLUSIVE, USF_LOCAL_MIN,
};
use glam::{DVec3, IVec3, Vec3};
use std::{
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsfPositionError {
    NonFiniteTranslation,
    TranslationTooLarge,
    IncompatibleLeafScale,
    RelativePositionOutsideBound,
}

/// Canonical decimal USF spatial position.
///
/// Each digit selects one of `10 x 10 x 10` child chunks using balanced digits
/// `[-5, 5)`. Every chunk spans `1000^3` units native to its own scale. The
/// final `offset` is measured in units native to `leaf_scale` and is normalized
/// into `[-500, 500)` on every axis.
///
/// `leaf_scale` is the finest currently resolved digit of this particular
/// canonical position. It may be anywhere in the full S-35..S+35 range; S0 has
/// no special positional-authority meaning.
#[derive(bevy::prelude::Component, Clone, Copy, PartialEq)]
pub struct UsfPosition {
    digits: [IVec3; SPATIAL_SCALE_COUNT],
    leaf_scale: SpatialScale,
    offset: Vec3,
}

/// One scalar axis of a canonical USF position.
///
/// Its ordinary decimal representation is expressed in S0 units (SI metres).
/// Scale Stack digits therefore become ordinary decimal digits before/after the
/// decimal point without collapsing the canonical value through a machine float.
#[derive(Clone, Copy, PartialEq)]
pub struct UsfCoordinate {
    digits: [i8; SPATIAL_SCALE_COUNT],
    leaf_scale: SpatialScale,
    offset: f32,
}

impl UsfCoordinate {
    pub const fn leaf_scale(self) -> SpatialScale {
        self.leaf_scale
    }

    pub const fn offset(self) -> f32 {
        self.offset
    }

    pub fn digit(self, scale: SpatialScale) -> i8 {
        self.digits[scale.index_from_top()]
    }

    fn plain_decimal(self) -> String {
        let mut terms = Vec::<(i32, i32)>::with_capacity(SPATIAL_SCALE_COUNT + 12);

        for raw_scale in self.leaf_scale.exponent()..=SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw_scale).expect("validated spatial scale");
            let digit = i32::from(self.digit(scale));
            if digit != 0 {
                // One chunk digit at Sx is 1000 * 10^x S0 units.
                terms.push((i32::from(raw_scale) + 3, digit));
            }
        }

        push_float_decimal_terms(
            &mut terms,
            self.offset,
            i32::from(self.leaf_scale.exponent()),
        );

        format_decimal_terms(&terms)
    }
}

impl Display for UsfCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.plain_decimal())
    }
}

impl std::fmt::Debug for UsfCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UsfCoordinate({self})")
    }
}

impl Display for UsfPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl std::fmt::Debug for UsfPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UsfPosition({},{},{})", self.x(), self.y(), self.z())
    }
}

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

    /// Legacy metre-adapter spelling retained for existing callers.
    pub fn from_scale0_local(local_metres: Vec3) -> Result<Self, UsfPositionError> {
        Self::from_metres_local(local_metres)
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
            weight /= f64::from(USF_CHILD_CHUNKS_PER_AXIS);
        }

        let leaf_units_per_requested =
            f64::from(USF_CHILD_CHUNKS_PER_AXIS).powi(requested_index as i32);
        component += f64::from(axis_f32(self.offset, axis) - axis_f32(origin.offset, axis))
            / leaf_units_per_requested;

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
            weight /= f64::from(USF_CHILD_CHUNKS_PER_AXIS);
        }

        let leaf_units_per_requested =
            f64::from(USF_CHILD_CHUNKS_PER_AXIS).powi(requested_index as i32);
        component += f64::from(axis_f32(self.offset, axis) - axis_f32(origin.offset, axis))
            / leaf_units_per_requested;

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

    pub fn nonzero_digits(&self) -> impl Iterator<Item = (SpatialScale, IVec3)> + '_ {
        (self.leaf_scale.exponent()..=SPATIAL_SCALE_MAX)
            .rev()
            .filter_map(|raw| {
                let scale = SpatialScale::new(raw).expect("range is validated");
                let digit = self.digit(scale);
                (digit != IVec3::ZERO).then_some((scale, digit))
            })
    }

    pub fn format_stack(&self) -> String {
        let mut parts = self
            .nonzero_digits()
            .map(|(scale, digit)| format!("S{}=({}, {}, {})", scale, digit.x, digit.y, digit.z))
            .collect::<Vec<_>>();

        if parts.is_empty() {
            parts.push("digits=0".to_string());
        }

        format!(
            "{} | S{} local=({:.3}, {:.3}, {:.3})",
            parts.join(" "),
            self.leaf_scale,
            self.offset.x,
            self.offset.y,
            self.offset.z,
        )
    }

    fn normalize(&mut self) -> Result<(), UsfPositionError> {
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

    fn add_chunk_carry(&mut self, axis: usize, mut carry: i64) -> Result<(), UsfPositionError> {
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

// `UsfPosition` constructors and translation APIs reject non-finite offsets and
// normalize them into one canonical range, so semantic positions have a valid
// equivalence relation even though their bounded leaf offset is stored as f32.
impl Eq for UsfPosition {}

impl Hash for UsfPosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for digit in &self.digits {
            digit.x.hash(state);
            digit.y.hash(state);
            digit.z.hash(state);
        }
        self.leaf_scale.exponent().hash(state);
        canonical_f32_bits(self.offset.x).hash(state);
        canonical_f32_bits(self.offset.y).hash(state);
        canonical_f32_bits(self.offset.z).hash(state);
    }
}

fn canonical_f32_bits(value: f32) -> u32 {
    if value == 0.0 { 0 } else { value.to_bits() }
}

fn axis_f32(value: Vec3, axis: usize) -> f32 {
    match axis {
        0 => value.x,
        1 => value.y,
        2 => value.z,
        _ => unreachable!(),
    }
}

fn set_axis_f32(value: &mut Vec3, axis: usize, component: f32) {
    match axis {
        0 => value.x = component,
        1 => value.y = component,
        2 => value.z = component,
        _ => unreachable!(),
    }
}

fn axis_i32(value: IVec3, axis: usize) -> i32 {
    match axis {
        0 => value.x,
        1 => value.y,
        2 => value.z,
        _ => unreachable!(),
    }
}

fn set_axis_i32(value: &mut IVec3, axis: usize, component: i32) {
    match axis {
        0 => value.x = component,
        1 => value.y = component,
        2 => value.z = component,
        _ => unreachable!(),
    }
}
