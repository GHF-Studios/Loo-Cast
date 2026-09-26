//! Pure canonical Universal Simulation Framework spatial-number algebra.
//!
//! This crate owns the hierarchical Scale Stack number itself: spatial scales,
//! balanced decimal digits, normalization/carry, canonical displacement,
//! re-expression, and unit conversion.
//!
//! The default feature set deliberately has no Bevy/ECS dependency. The
//! `bevy-integration` feature exists only so the current host engine can attach
//! transitional Bevy traits directly to these canonical value types while the
//! runtime adapter layers are migrated independently.

use std::{
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
};

use glam::{DVec3, IVec3, Vec3};

mod chunk_address;
pub use chunk_address::UsfChunkAddress;

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

#[cfg_attr(feature = "bevy-integration", derive(bevy_reflect::Reflect))]
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

    /// SI metres represented by one native unit in this Scale Slice.
    ///
    /// The fact that S0's native spatial unit equals one metre is a unit-system
    /// convention only. S0 is not an architectural origin, center, minimum, or
    /// otherwise privileged Scale Slice.
    pub fn metres_per_native(self) -> f64 {
        10.0_f64.powi(self.exponent() as i32)
    }

    /// Projects an SI-metre distance/speed/acceleration into this slice's native units.
    pub fn metres_to_native_f64(self, value: f64) -> f64 {
        value / self.metres_per_native()
    }

    /// f32 adapter for bounded chart-local runtime APIs.
    pub fn metres_to_native_f32(self, value: f32) -> f32 {
        self.metres_to_native_f64(f64::from(value))
            .clamp(-(f32::MAX as f64), f32::MAX as f64) as f32
    }

    /// Converts one chart-native runtime value back to SI metres.
    pub fn native_to_metres_f32(self, value: f32) -> f32 {
        (f64::from(value) * self.metres_per_native())
            .clamp(-(f32::MAX as f64), f32::MAX as f64) as f32
    }

    // Compatibility aliases while older code migrates. They describe the same
    // metre-unit convention; they do not establish S0 as USF's origin.
    pub fn scale0_units_per_native(self) -> f64 {
        self.metres_per_native()
    }

    pub fn scale0_to_native_f64(self, value: f64) -> f64 {
        self.metres_to_native_f64(value)
    }

    pub fn scale0_to_native_f32(self, value: f32) -> f32 {
        self.metres_to_native_f32(value)
    }

    pub fn native_to_scale0_f32(self, value: f32) -> f32 {
        self.native_to_metres_f32(value)
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
/// `leaf_scale` is the finest currently resolved digit of this particular
/// canonical position. It may be anywhere in the full S-35..S+35 range; S0 has
/// no special positional-authority meaning.
#[cfg_attr(feature = "bevy-integration", derive(bevy_ecs::prelude::Component))]
#[derive(Clone, Copy, PartialEq)]
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
            let parent = SpatialScale::new(child.exponent() + 1)
                .expect("target scale bounds coarsening");
            let digit = self.digit(child).as_vec3();
            self.digits[child.index_from_top()] = IVec3::ZERO;
            self.offset = digit
                * (USF_CHUNK_NATIVE_SIZE / USF_CHILD_CHUNKS_PER_AXIS as f32)
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
    pub fn coordinate_at_scale_f64(
        &self,
        scale: SpatialScale,
    ) -> Result<DVec3, UsfPositionError> {
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

        let encoded_delta = UsfPosition::from_scale_native_f64(
            delta,
            scale,
            self.leaf_scale,
        )?;
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
            let offset_sum = f64::from(axis_f32(self.offset, axis))
                + f64::from(axis_f32(delta.offset, axis));

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
                let digit =
                    total - parent_carry * i64::from(USF_CHILD_CHUNKS_PER_AXIS);

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
        let mut weight =
            USF_CHUNK_NATIVE_SIZE as f64 / f64::from(USF_CHILD_CHUNKS_PER_AXIS);
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


fn push_float_decimal_terms(terms: &mut Vec<(i32, i32)>, value: f32, scale_shift: i32) {
    if value == 0.0 {
        return;
    }

    let negative = value.is_sign_negative();
    let rendered = value.abs().to_string();
    let (mantissa, exponent) = match rendered.find(|c| c == 'e' || c == 'E') {
        Some(index) => (
            &rendered[..index],
            rendered[index + 1..]
                .parse::<i32>()
                .expect("f32 exponent formatting is decimal"),
        ),
        None => (rendered.as_str(), 0),
    };

    let fractional_digits = mantissa
        .split_once('.')
        .map_or(0_i32, |(_, fractional)| fractional.len() as i32);
    let digits = mantissa.chars().filter(|c| *c != '.').collect::<Vec<_>>();
    let least_exponent = scale_shift + exponent - fractional_digits;

    for (index, ch) in digits.iter().rev().enumerate() {
        let digit = ch
            .to_digit(10)
            .expect("f32 display contains decimal digits") as i32;
        if digit == 0 {
            continue;
        }
        terms.push((
            least_exponent + index as i32,
            if negative { -digit } else { digit },
        ));
    }
}

fn normalize_unsigned_decimal(digits: &mut Vec<i32>) {
    let mut index = 0;
    while index < digits.len() {
        if digits[index] >= 10 {
            let carry = digits[index] / 10;
            digits[index] %= 10;
            if index + 1 == digits.len() {
                digits.push(carry);
            } else {
                digits[index + 1] += carry;
            }
        }
        index += 1;
    }

    while digits.len() > 1 && digits.last() == Some(&0) {
        digits.pop();
    }
}

fn compare_unsigned_decimal(lhs: &[i32], rhs: &[i32]) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    let lhs_top = lhs.iter().rposition(|digit| *digit != 0);
    let rhs_top = rhs.iter().rposition(|digit| *digit != 0);

    match (lhs_top, rhs_top) {
        (None, None) => Ordering::Equal,
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (Some(lhs_top), Some(rhs_top)) => match lhs_top.cmp(&rhs_top) {
            Ordering::Equal => {
                for index in (0..=lhs_top).rev() {
                    match lhs[index].cmp(&rhs[index]) {
                        Ordering::Equal => {}
                        other => return other,
                    }
                }
                Ordering::Equal
            }
            other => other,
        },
    }
}

fn subtract_unsigned_decimal(lhs: &[i32], rhs: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(lhs.len());
    let mut borrow = 0;

    for index in 0..lhs.len() {
        let mut value = lhs[index] - rhs.get(index).copied().unwrap_or(0) - borrow;
        if value < 0 {
            value += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }
        result.push(value);
    }

    debug_assert_eq!(borrow, 0);
    while result.len() > 1 && result.last() == Some(&0) {
        result.pop();
    }
    result
}

fn format_decimal_terms(terms: &[(i32, i32)]) -> String {
    use std::cmp::Ordering;

    if terms.is_empty() {
        return "0".to_string();
    }

    let min_exponent = terms.iter().map(|(exponent, _)| *exponent).min().unwrap();
    let max_exponent = terms.iter().map(|(exponent, _)| *exponent).max().unwrap();
    let width = (max_exponent - min_exponent + 2) as usize;

    let mut positive = vec![0_i32; width];
    let mut negative = vec![0_i32; width];

    for &(exponent, coefficient) in terms {
        let index = (exponent - min_exponent) as usize;
        if coefficient >= 0 {
            positive[index] += coefficient;
        } else {
            negative[index] += -coefficient;
        }
    }

    normalize_unsigned_decimal(&mut positive);
    normalize_unsigned_decimal(&mut negative);

    let ordering = compare_unsigned_decimal(&positive, &negative);
    if ordering == Ordering::Equal {
        return "0".to_string();
    }

    let (negative_sign, magnitude) = match ordering {
        Ordering::Greater => (false, subtract_unsigned_decimal(&positive, &negative)),
        Ordering::Less => (true, subtract_unsigned_decimal(&negative, &positive)),
        Ordering::Equal => unreachable!(),
    };

    let highest_index = magnitude
        .iter()
        .rposition(|digit| *digit != 0)
        .expect("nonzero magnitude has a nonzero digit");
    let highest_exponent = min_exponent + highest_index as i32;

    let digit_at = |exponent: i32| -> i32 {
        let index = exponent - min_exponent;
        if index < 0 {
            0
        } else {
            magnitude.get(index as usize).copied().unwrap_or(0)
        }
    };

    let mut rendered = String::new();
    if negative_sign {
        rendered.push('-');
    }

    if highest_exponent >= 0 {
        for exponent in (0..=highest_exponent).rev() {
            rendered.push(char::from_digit(digit_at(exponent) as u32, 10).unwrap());
        }
    } else {
        rendered.push('0');
    }

    if min_exponent < 0 {
        let mut fractional = String::new();
        for exponent in (min_exponent..=-1).rev() {
            fractional.push(char::from_digit(digit_at(exponent) as u32, 10).unwrap());
        }

        while fractional.ends_with('0') {
            fractional.pop();
        }

        if !fractional.is_empty() {
            rendered.push('.');
            rendered.push_str(&fractional);
        }
    }

    rendered
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
mod tests;
