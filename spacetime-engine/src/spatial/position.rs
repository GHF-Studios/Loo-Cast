use std::{
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
};

use bevy::prelude::*;

pub const SPATIAL_SCALE_MAX: i8 = 35;
pub const SPATIAL_SCALE_MIN: i8 = -35;
pub const SPATIAL_SCALE_COUNT: usize =
    (SPATIAL_SCALE_MAX as i16 - SPATIAL_SCALE_MIN as i16 + 1) as usize;

pub const USF_CHUNK_NATIVE_SIZE: f32 = 1000.0;
pub const USF_CHILD_CHUNKS_PER_AXIS: i32 = 10;
pub const USF_BALANCED_DIGIT_MIN: i32 = -5;
pub const USF_BALANCED_DIGIT_MAX_EXCLUSIVE: i32 = 5;
pub const USF_LOCAL_MIN: f32 = -USF_CHUNK_NATIVE_SIZE * 0.5;
pub const USF_LOCAL_MAX_EXCLUSIVE: f32 = USF_CHUNK_NATIVE_SIZE * 0.5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SpatialScale(i8);

impl SpatialScale {
    pub const MIN: Self = Self(SPATIAL_SCALE_MIN);
    pub const MAX: Self = Self(SPATIAL_SCALE_MAX);
    pub const ZERO: Self = Self(0);

    pub const fn new(value: i8) -> Option<Self> {
        if value >= SPATIAL_SCALE_MIN && value <= SPATIAL_SCALE_MAX {
            Some(Self(value))
        } else {
            None
        }
    }

    pub const fn exponent(self) -> i8 {
        self.0
    }

    pub const fn index_from_top(self) -> usize {
        (SPATIAL_SCALE_MAX - self.0) as usize
    }
}

impl Display for SpatialScale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+}", self.0)
    }
}

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
/// M7 fixes `leaf_scale` to scale 0 for runtime movement. The full 71-slot stack
/// is present now so later scale transitions do not need to replace the spatial
/// identity representation.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct UsfPosition {
    digits: [IVec3; SPATIAL_SCALE_COUNT],
    leaf_scale: SpatialScale,
    offset: Vec3,
}

impl Default for UsfPosition {
    fn default() -> Self {
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

    pub fn from_scale0_local(local_meters: Vec3) -> Result<Self, UsfPositionError> {
        Self::zero(SpatialScale::ZERO).translated_native(local_meters)
    }

    pub const fn leaf_scale(&self) -> SpatialScale {
        self.leaf_scale
    }

    pub const fn offset(&self) -> Vec3 {
        self.offset
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
        let mut delta = Vec3::ZERO;
        for axis in 0..3 {
            set_axis_f32(
                &mut delta,
                axis,
                self.relative_axis_at_scale_bounded(origin, scale, axis, max_abs)?,
            );
        }
        Ok(delta)
    }

    /// Axis-local form used by bounded algorithms that intentionally do not
    /// require the other two coordinates to fit the same local chart.
    pub(crate) fn relative_native_axis_bounded(
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

    /// Returns whether the wrapped canonical displacement on one axis is on
    /// the negative side of the origin. This is useful for bounded algorithms
    /// that need an orientation even when the magnitude is intentionally not
    /// projected into `f32`.
    pub(crate) fn relative_native_axis_is_negative(
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
        for axis in 0..3 {
            let offset = axis_f32(self.offset, axis);
            if !offset.is_finite() {
                return Err(UsfPositionError::NonFiniteTranslation);
            }

            let carry_f = ((offset + USF_CHUNK_NATIVE_SIZE * 0.5) / USF_CHUNK_NATIVE_SIZE).floor();
            if carry_f < i64::MIN as f32 || carry_f > i64::MAX as f32 {
                return Err(UsfPositionError::TranslationTooLarge);
            }
            let carry = carry_f as i64;
            set_axis_f32(
                &mut self.offset,
                axis,
                offset - carry as f32 * USF_CHUNK_NATIVE_SIZE,
            );

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

/// Canonical identity of one USF Chunk at one spatial scale.
///
/// An address retains only the balanced-decimal digits that identify the chunk
/// at `scale`; finer digits and the leaf-local offset are intentionally ignored.
/// This makes the same type useful for sparse Phenomenon state, generation
/// caches and later representation attachments without allocating the hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsfChunkAddress {
    scale: SpatialScale,
    digits: [IVec3; SPATIAL_SCALE_COUNT],
}

impl UsfChunkAddress {
    pub fn containing(
        position: UsfPosition,
        scale: SpatialScale,
    ) -> Result<Self, UsfPositionError> {
        if scale < position.leaf_scale {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }

        let mut digits = [IVec3::ZERO; SPATIAL_SCALE_COUNT];
        for raw_scale in scale.exponent()..=SPATIAL_SCALE_MAX {
            let digit_scale = SpatialScale::new(raw_scale).expect("validated spatial scale range");
            digits[digit_scale.index_from_top()] = position.digit(digit_scale);
        }

        Ok(Self { scale, digits })
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub fn digit(self, scale: SpatialScale) -> Option<IVec3> {
        if scale < self.scale {
            None
        } else {
            Some(self.digits[scale.index_from_top()])
        }
    }

    pub fn parent(self) -> Option<Self> {
        if self.scale == SpatialScale::MAX {
            return None;
        }

        let mut parent = self;
        parent.digits[self.scale.index_from_top()] = IVec3::ZERO;
        parent.scale = SpatialScale::new(self.scale.exponent() + 1)
            .expect("non-root scale always has a parent");
        Some(parent)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spatial_scale_stack_has_seventy_one_levels() {
        assert_eq!(SPATIAL_SCALE_COUNT, 71);
        assert_eq!(SpatialScale::MAX.exponent(), 35);
        assert_eq!(SpatialScale::MIN.exponent(), -35);
    }

    #[test]
    fn unit_offset_carries_into_scale_zero_chunk_digit() {
        let position = UsfPosition::from_scale0_local(Vec3::new(600.0, 0.0, 0.0)).unwrap();
        assert_eq!(position.offset().x, -400.0);
        assert_eq!(position.digit(SpatialScale::ZERO).x, 1);
    }

    #[test]
    fn ten_scale_zero_chunks_carry_once_into_scale_one() {
        let position = UsfPosition::from_scale0_local(Vec3::new(10_000.0, 0.0, 0.0)).unwrap();
        assert_eq!(position.offset().x, 0.0);
        assert_eq!(position.digit(SpatialScale::ZERO).x, 0);
        assert_eq!(position.digit(SpatialScale::new(1).unwrap()).x, 1);
    }

    #[test]
    fn whole_native_translation_matches_bounded_float_translation() {
        let exact = UsfPosition::default()
            .translated_whole_native([1_280, -600, 42])
            .unwrap();
        let bounded = UsfPosition::default()
            .translated_native(Vec3::new(1_280.0, -600.0, 42.0))
            .unwrap();

        assert_eq!(exact, bounded);
    }

    #[test]
    fn scale_relative_projection_does_not_require_a_giant_leaf_coordinate() {
        let origin = UsfPosition::default();
        let point = origin
            .translated_whole_native([12_345, -6_780, 50])
            .unwrap();
        let s1 = SpatialScale::new(1).unwrap();

        let delta = point
            .relative_at_scale_bounded(&origin, s1, 2_000.0)
            .unwrap();
        assert!((delta.x - 1_234.5).abs() < 1.0e-4);
        assert!((delta.y + 678.0).abs() < 1.0e-4);
        assert!((delta.z - 5.0).abs() < 1.0e-4);
    }

    #[test]
    fn bounded_relative_position_crosses_balanced_digit_carry_exactly() {
        let origin = UsfPosition::from_scale0_local(Vec3::new(499.0, 0.0, 0.0)).unwrap();
        let point = origin.translated_native(Vec3::new(4.0, -3.0, 2.0)).unwrap();

        assert_eq!(
            point.relative_native_bounded(&origin, 8.0).unwrap(),
            Vec3::new(4.0, -3.0, 2.0)
        );
    }

    #[test]
    fn bounded_relative_position_rejects_far_semantic_points() {
        let origin = UsfPosition::default();
        let far = origin.translated_whole_native([20_000, 0, 0]).unwrap();

        assert_eq!(
            far.relative_native_bounded(&origin, 512.0),
            Err(UsfPositionError::RelativePositionOutsideBound)
        );
    }

    #[test]
    fn bounded_relative_position_survives_a_long_decimal_carry_chain() {
        let mut origin = UsfPosition::default();
        let mut point = UsfPosition::default();

        // 0444...444 + one scale-0 USF chunk = 1(-5)(-5)...(-5)
        // in balanced decimal representation. Difference normalization must
        // happen before bounded float projection; a raw prefix accumulator would
        // grow enormous before the low digit differences cancel back to +1.
        for raw_scale in SpatialScale::ZERO.exponent()..SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw_scale).unwrap();
            origin.digits[scale.index_from_top()] = IVec3::new(4, 0, 0);
            point.digits[scale.index_from_top()] = IVec3::new(-5, 0, 0);
        }
        point.digits[SpatialScale::MAX.index_from_top()] = IVec3::new(1, 0, 0);

        assert_eq!(
            point.relative_native_bounded(&origin, 2_000.0).unwrap(),
            Vec3::new(1_000.0, 0.0, 0.0)
        );
    }

    #[test]
    fn root_carry_wraps_to_the_opposite_world_edge() {
        let mut before = UsfPosition::default();
        before.offset.x = 499.0;
        for raw_scale in SpatialScale::ZERO.exponent()..=SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw_scale).unwrap();
            before.digits[scale.index_from_top()].x = 4;
        }

        let after = before.translated_native(Vec3::new(2.0, 0.0, 0.0)).unwrap();
        assert_eq!(after.offset.x, -499.0);
        for raw_scale in SpatialScale::ZERO.exponent()..=SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw_scale).unwrap();
            assert_eq!(after.digit(scale).x, -5);
        }
        assert_eq!(
            after.relative_native_bounded(&before, 4.0).unwrap(),
            Vec3::new(2.0, 0.0, 0.0)
        );
    }

    #[test]
    fn root_borrow_wraps_to_the_opposite_world_edge() {
        let mut before = UsfPosition::default();
        before.offset.x = -499.0;
        for raw_scale in SpatialScale::ZERO.exponent()..=SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw_scale).unwrap();
            before.digits[scale.index_from_top()].x = -5;
        }

        let after = before.translated_native(Vec3::new(-2.0, 0.0, 0.0)).unwrap();
        assert_eq!(after.offset.x, 499.0);
        for raw_scale in SpatialScale::ZERO.exponent()..=SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw_scale).unwrap();
            assert_eq!(after.digit(scale).x, 4);
        }
        assert_eq!(
            after.relative_native_bounded(&before, 4.0).unwrap(),
            Vec3::new(-2.0, 0.0, 0.0)
        );
    }

    #[test]
    fn negative_translation_uses_balanced_decimal_carry() {
        let position = UsfPosition::from_scale0_local(Vec3::new(-600.0, 0.0, 0.0)).unwrap();
        assert_eq!(position.offset().x, 400.0);
        assert_eq!(position.digit(SpatialScale::ZERO).x, -1);
    }
}
