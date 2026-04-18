//! TEMP SKETCH ONLY - RAW MODEL SURFACE
//! - structs/enums/traits/type aliases only
//! - no impl blocks
//! - trait methods are declarations only
//! - no free functions

#![allow(dead_code)]

// ---------------------------------------------------------------------------
// Supported "normal" scalar numeric types (canonical list via aliases)
// ---------------------------------------------------------------------------

pub type NormalI8 = i8;
pub type NormalI16 = i16;
pub type NormalI32 = i32;
pub type NormalI64 = i64;
pub type NormalI128 = i128;
pub type NormalIsize = isize;

pub type NormalU8 = u8;
pub type NormalU16 = u16;
pub type NormalU32 = u32;
pub type NormalU64 = u64;
pub type NormalU128 = u128;
pub type NormalUsize = usize;

pub type NormalF32 = f32;
pub type NormalF64 = f64;

pub type SupportedNormalScalars = (
    NormalI8,
    NormalI16,
    NormalI32,
    NormalI64,
    NormalI128,
    NormalIsize,
    NormalU8,
    NormalU16,
    NormalU32,
    NormalU64,
    NormalU128,
    NormalUsize,
    NormalF32,
    NormalF64,
);

// Normal canonical math aliases (Bevy/glam-backed).
pub type NormalVec2f32 = bevy::math::Vec2;
pub type NormalVec3f32 = bevy::math::Vec3;
pub type NormalVec4f32 = bevy::math::Vec4;
pub type NormalVec2f64 = bevy::math::DVec2;
pub type NormalVec3f64 = bevy::math::DVec3;
pub type NormalVec4f64 = bevy::math::DVec4;

pub type NormalVec2i32 = bevy::math::IVec2;
pub type NormalVec3i32 = bevy::math::IVec3;
pub type NormalVec4i32 = bevy::math::IVec4;
pub type NormalVec2u32 = bevy::math::UVec2;
pub type NormalVec3u32 = bevy::math::UVec3;
pub type NormalVec4u32 = bevy::math::UVec4;

pub type NormalMat2f32 = bevy::math::Mat2;
pub type NormalMat3f32 = bevy::math::Mat3;
pub type NormalMat4f32 = bevy::math::Mat4;
pub type NormalMat2f64 = bevy::math::DMat2;
pub type NormalMat3f64 = bevy::math::DMat3;
pub type NormalMat4f64 = bevy::math::DMat4;

pub type NormalQuatf32 = bevy::math::Quat;
pub type NormalQuatf64 = bevy::math::DQuat;

pub type NormalTranslation3f32 = bevy::math::Vec3;
pub type NormalRotationf32 = bevy::math::Quat;
pub type NormalScalef32 = bevy::math::Vec3;
pub type NormalTransformf32 = bevy::prelude::Transform;

// ---------------------------------------------------------------------------
// USF core numeric/tensor data model
// ---------------------------------------------------------------------------

pub type UsfDigit = i8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfScalar {
    pub digits: Vec<UsfDigit>,
    pub radix_position: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfVec<const N: usize> {
    pub lanes: [UsfScalar; N],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfMat<const R: usize, const C: usize> {
    pub rows: [UsfVec<C>; R],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfQuat {
    pub x: UsfScalar,
    pub y: UsfScalar,
    pub z: UsfScalar,
    pub w: UsfScalar,
}

// ---------------------------------------------------------------------------
// USF wrappers/newtypes (translation, rotation, scale, transform)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfTranslation<const N: usize>(pub UsfVec<N>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfRotation(pub UsfQuat);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfScale {
    pub log_base: UsfScalar,
    pub scale_index: i16,
    pub fractional_log_offset: UsfScalar,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfTransform {
    pub translation: UsfTranslation<3>,
    pub rotation: UsfRotation,
    pub scale: UsfScale,
}

// ---------------------------------------------------------------------------
// Static-core trait contracts (operation-heavy declarations only)
// ---------------------------------------------------------------------------

pub trait UsfScalarSpec {
    // constructors / constants
    fn zero() -> Self;
    fn one() -> Self;
    fn two() -> Self;
    fn ten() -> Self;
    fn neg_one() -> Self;
    fn pi() -> Self;
    fn tau() -> Self;
    fn e() -> Self;
    fn nan() -> Self;
    fn infinity() -> Self;
    fn neg_infinity() -> Self;

    // parsing / formatting
    fn parse_decimal(text: &str) -> Self;
    fn to_decimal_string(&self) -> String;
    fn to_scientific_string(&self) -> String;
    fn normalize(&self) -> Self;

    // classification
    fn is_zero(&self) -> bool;
    fn is_one(&self) -> bool;
    fn is_nan(&self) -> bool;
    fn is_infinite(&self) -> bool;
    fn is_finite(&self) -> bool;
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
    fn signum(&self) -> Self;

    // comparison / bounds
    fn min(&self, rhs: &Self) -> Self;
    fn max(&self, rhs: &Self) -> Self;
    fn clamp(&self, lo: &Self, hi: &Self) -> Self;
    fn cmp_eq(&self, rhs: &Self) -> bool;
    fn cmp_ne(&self, rhs: &Self) -> bool;
    fn cmp_lt(&self, rhs: &Self) -> bool;
    fn cmp_le(&self, rhs: &Self) -> bool;
    fn cmp_gt(&self, rhs: &Self) -> bool;
    fn cmp_ge(&self, rhs: &Self) -> bool;

    // unary
    fn neg(&self) -> Self;
    fn abs(&self) -> Self;
    fn recip(&self) -> Self;
    fn square(&self) -> Self;
    fn cube(&self) -> Self;
    fn sqrt(&self) -> Self;
    fn cbrt(&self) -> Self;
    fn exp(&self) -> Self;
    fn exp2(&self) -> Self;
    fn exp10(&self) -> Self;
    fn ln(&self) -> Self;
    fn log2(&self) -> Self;
    fn log10(&self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn tan(&self) -> Self;
    fn asin(&self) -> Self;
    fn acos(&self) -> Self;
    fn atan(&self) -> Self;
    fn sinh(&self) -> Self;
    fn cosh(&self) -> Self;
    fn tanh(&self) -> Self;
    fn floor(&self) -> Self;
    fn ceil(&self) -> Self;
    fn round(&self) -> Self;
    fn trunc(&self) -> Self;
    fn fract(&self) -> Self;

    // binary
    fn add(&self, rhs: &Self) -> Self;
    fn sub(&self, rhs: &Self) -> Self;
    fn mul(&self, rhs: &Self) -> Self;
    fn div(&self, rhs: &Self) -> Self;
    fn rem(&self, rhs: &Self) -> Self;
    fn pow(&self, rhs: &Self) -> Self;
    fn atan2(&self, rhs: &Self) -> Self;
    fn hypot(&self, rhs: &Self) -> Self;
    fn mod_euclid(&self, rhs: &Self) -> Self;

    // ternary
    fn fma(&self, b: &Self, c: &Self) -> Self;
    fn lerp_f32(&self, rhs: &Self, t: NormalF32) -> Self;
    fn lerp_f64(&self, rhs: &Self, t: NormalF64) -> Self;
    fn smoothstep_f32(&self, edge0: &Self, edge1: &Self) -> Self;
    fn smoothstep_f64(&self, edge0: &Self, edge1: &Self) -> Self;

    // mixed interop (USF + normal; no pure-normal ops)
    fn from_i8(value: NormalI8) -> Self;
    fn from_i16(value: NormalI16) -> Self;
    fn from_i32(value: NormalI32) -> Self;
    fn from_i64(value: NormalI64) -> Self;
    fn from_i128(value: NormalI128) -> Self;
    fn from_isize(value: NormalIsize) -> Self;
    fn from_u8(value: NormalU8) -> Self;
    fn from_u16(value: NormalU16) -> Self;
    fn from_u32(value: NormalU32) -> Self;
    fn from_u64(value: NormalU64) -> Self;
    fn from_u128(value: NormalU128) -> Self;
    fn from_usize(value: NormalUsize) -> Self;
    fn from_f32(value: NormalF32) -> Self;
    fn from_f64(value: NormalF64) -> Self;

    fn to_i8(&self) -> NormalI8;
    fn to_i16(&self) -> NormalI16;
    fn to_i32(&self) -> NormalI32;
    fn to_i64(&self) -> NormalI64;
    fn to_i128(&self) -> NormalI128;
    fn to_isize(&self) -> NormalIsize;
    fn to_u8(&self) -> NormalU8;
    fn to_u16(&self) -> NormalU16;
    fn to_u32(&self) -> NormalU32;
    fn to_u64(&self) -> NormalU64;
    fn to_u128(&self) -> NormalU128;
    fn to_usize(&self) -> NormalUsize;
    fn to_f32(&self) -> NormalF32;
    fn to_f64(&self) -> NormalF64;

    // signed int mixed ops
    fn add_i8(&self, rhs: NormalI8) -> Self;
    fn sub_i8(&self, rhs: NormalI8) -> Self;
    fn mul_i8(&self, rhs: NormalI8) -> Self;
    fn div_i8(&self, rhs: NormalI8) -> Self;
    fn rem_i8(&self, rhs: NormalI8) -> Self;
    fn min_i8(&self, rhs: NormalI8) -> Self;
    fn max_i8(&self, rhs: NormalI8) -> Self;
    fn clamp_i8(&self, lo: NormalI8, hi: NormalI8) -> Self;

    fn add_i16(&self, rhs: NormalI16) -> Self;
    fn sub_i16(&self, rhs: NormalI16) -> Self;
    fn mul_i16(&self, rhs: NormalI16) -> Self;
    fn div_i16(&self, rhs: NormalI16) -> Self;
    fn rem_i16(&self, rhs: NormalI16) -> Self;
    fn min_i16(&self, rhs: NormalI16) -> Self;
    fn max_i16(&self, rhs: NormalI16) -> Self;
    fn clamp_i16(&self, lo: NormalI16, hi: NormalI16) -> Self;

    fn add_i32(&self, rhs: NormalI32) -> Self;
    fn sub_i32(&self, rhs: NormalI32) -> Self;
    fn mul_i32(&self, rhs: NormalI32) -> Self;
    fn div_i32(&self, rhs: NormalI32) -> Self;
    fn rem_i32(&self, rhs: NormalI32) -> Self;
    fn min_i32(&self, rhs: NormalI32) -> Self;
    fn max_i32(&self, rhs: NormalI32) -> Self;
    fn clamp_i32(&self, lo: NormalI32, hi: NormalI32) -> Self;
    fn pow_i32(&self, rhs: NormalI32) -> Self;

    fn add_i64(&self, rhs: NormalI64) -> Self;
    fn sub_i64(&self, rhs: NormalI64) -> Self;
    fn mul_i64(&self, rhs: NormalI64) -> Self;
    fn div_i64(&self, rhs: NormalI64) -> Self;
    fn rem_i64(&self, rhs: NormalI64) -> Self;
    fn min_i64(&self, rhs: NormalI64) -> Self;
    fn max_i64(&self, rhs: NormalI64) -> Self;
    fn clamp_i64(&self, lo: NormalI64, hi: NormalI64) -> Self;

    fn add_i128(&self, rhs: NormalI128) -> Self;
    fn sub_i128(&self, rhs: NormalI128) -> Self;
    fn mul_i128(&self, rhs: NormalI128) -> Self;
    fn div_i128(&self, rhs: NormalI128) -> Self;
    fn rem_i128(&self, rhs: NormalI128) -> Self;
    fn min_i128(&self, rhs: NormalI128) -> Self;
    fn max_i128(&self, rhs: NormalI128) -> Self;
    fn clamp_i128(&self, lo: NormalI128, hi: NormalI128) -> Self;

    fn add_isize(&self, rhs: NormalIsize) -> Self;
    fn sub_isize(&self, rhs: NormalIsize) -> Self;
    fn mul_isize(&self, rhs: NormalIsize) -> Self;
    fn div_isize(&self, rhs: NormalIsize) -> Self;
    fn rem_isize(&self, rhs: NormalIsize) -> Self;
    fn min_isize(&self, rhs: NormalIsize) -> Self;
    fn max_isize(&self, rhs: NormalIsize) -> Self;
    fn clamp_isize(&self, lo: NormalIsize, hi: NormalIsize) -> Self;

    // unsigned int mixed ops
    fn add_u8(&self, rhs: NormalU8) -> Self;
    fn sub_u8(&self, rhs: NormalU8) -> Self;
    fn mul_u8(&self, rhs: NormalU8) -> Self;
    fn div_u8(&self, rhs: NormalU8) -> Self;
    fn rem_u8(&self, rhs: NormalU8) -> Self;
    fn min_u8(&self, rhs: NormalU8) -> Self;
    fn max_u8(&self, rhs: NormalU8) -> Self;
    fn clamp_u8(&self, lo: NormalU8, hi: NormalU8) -> Self;

    fn add_u16(&self, rhs: NormalU16) -> Self;
    fn sub_u16(&self, rhs: NormalU16) -> Self;
    fn mul_u16(&self, rhs: NormalU16) -> Self;
    fn div_u16(&self, rhs: NormalU16) -> Self;
    fn rem_u16(&self, rhs: NormalU16) -> Self;
    fn min_u16(&self, rhs: NormalU16) -> Self;
    fn max_u16(&self, rhs: NormalU16) -> Self;
    fn clamp_u16(&self, lo: NormalU16, hi: NormalU16) -> Self;

    fn add_u32(&self, rhs: NormalU32) -> Self;
    fn sub_u32(&self, rhs: NormalU32) -> Self;
    fn mul_u32(&self, rhs: NormalU32) -> Self;
    fn div_u32(&self, rhs: NormalU32) -> Self;
    fn rem_u32(&self, rhs: NormalU32) -> Self;
    fn min_u32(&self, rhs: NormalU32) -> Self;
    fn max_u32(&self, rhs: NormalU32) -> Self;
    fn clamp_u32(&self, lo: NormalU32, hi: NormalU32) -> Self;
    fn pow_u32(&self, rhs: NormalU32) -> Self;

    fn add_u64(&self, rhs: NormalU64) -> Self;
    fn sub_u64(&self, rhs: NormalU64) -> Self;
    fn mul_u64(&self, rhs: NormalU64) -> Self;
    fn div_u64(&self, rhs: NormalU64) -> Self;
    fn rem_u64(&self, rhs: NormalU64) -> Self;
    fn min_u64(&self, rhs: NormalU64) -> Self;
    fn max_u64(&self, rhs: NormalU64) -> Self;
    fn clamp_u64(&self, lo: NormalU64, hi: NormalU64) -> Self;

    fn add_u128(&self, rhs: NormalU128) -> Self;
    fn sub_u128(&self, rhs: NormalU128) -> Self;
    fn mul_u128(&self, rhs: NormalU128) -> Self;
    fn div_u128(&self, rhs: NormalU128) -> Self;
    fn rem_u128(&self, rhs: NormalU128) -> Self;
    fn min_u128(&self, rhs: NormalU128) -> Self;
    fn max_u128(&self, rhs: NormalU128) -> Self;
    fn clamp_u128(&self, lo: NormalU128, hi: NormalU128) -> Self;

    fn add_usize(&self, rhs: NormalUsize) -> Self;
    fn sub_usize(&self, rhs: NormalUsize) -> Self;
    fn mul_usize(&self, rhs: NormalUsize) -> Self;
    fn div_usize(&self, rhs: NormalUsize) -> Self;
    fn rem_usize(&self, rhs: NormalUsize) -> Self;
    fn min_usize(&self, rhs: NormalUsize) -> Self;
    fn max_usize(&self, rhs: NormalUsize) -> Self;
    fn clamp_usize(&self, lo: NormalUsize, hi: NormalUsize) -> Self;

    // float mixed ops
    fn add_f32(&self, rhs: NormalF32) -> Self;
    fn sub_f32(&self, rhs: NormalF32) -> Self;
    fn mul_f32(&self, rhs: NormalF32) -> Self;
    fn div_f32(&self, rhs: NormalF32) -> Self;
    fn rem_f32(&self, rhs: NormalF32) -> Self;
    fn min_f32(&self, rhs: NormalF32) -> Self;
    fn max_f32(&self, rhs: NormalF32) -> Self;
    fn clamp_f32(&self, lo: NormalF32, hi: NormalF32) -> Self;
    fn pow_f32(&self, rhs: NormalF32) -> Self;
    fn atan2_f32(&self, rhs: NormalF32) -> Self;
    fn hypot_f32(&self, rhs: NormalF32) -> Self;
    fn lerp_to_f32(&self, rhs: NormalF32, t: NormalF32) -> Self;
    fn smoothstep_to_f32(&self, edge0: NormalF32, edge1: NormalF32) -> Self;

    fn add_f64(&self, rhs: NormalF64) -> Self;
    fn sub_f64(&self, rhs: NormalF64) -> Self;
    fn mul_f64(&self, rhs: NormalF64) -> Self;
    fn div_f64(&self, rhs: NormalF64) -> Self;
    fn rem_f64(&self, rhs: NormalF64) -> Self;
    fn min_f64(&self, rhs: NormalF64) -> Self;
    fn max_f64(&self, rhs: NormalF64) -> Self;
    fn clamp_f64(&self, lo: NormalF64, hi: NormalF64) -> Self;
    fn pow_f64(&self, rhs: NormalF64) -> Self;
    fn atan2_f64(&self, rhs: NormalF64) -> Self;
    fn hypot_f64(&self, rhs: NormalF64) -> Self;
    fn lerp_to_f64(&self, rhs: NormalF64, t: NormalF64) -> Self;
    fn smoothstep_to_f64(&self, edge0: NormalF64, edge1: NormalF64) -> Self;
}

pub trait UsfVectorSpec<const N: usize>: Sized {
    // construction / indexing
    fn zero() -> Self;
    fn one() -> Self;
    fn splat(value: &UsfScalar) -> Self;
    fn from_lanes(lanes: [UsfScalar; N]) -> Self;
    fn to_lanes(&self) -> [UsfScalar; N];
    fn lane(&self, index: usize) -> UsfScalar;
    fn set_lane(&self, index: usize, lane: &UsfScalar) -> Self;

    // unary
    fn neg(&self) -> Self;
    fn abs(&self) -> Self;
    fn normalize(&self) -> Self;
    fn floor(&self) -> Self;
    fn ceil(&self) -> Self;
    fn round(&self) -> Self;
    fn fract(&self) -> Self;

    // binary
    fn add(&self, rhs: &Self) -> Self;
    fn sub(&self, rhs: &Self) -> Self;
    fn mul_elem(&self, rhs: &Self) -> Self;
    fn div_elem(&self, rhs: &Self) -> Self;
    fn min(&self, rhs: &Self) -> Self;
    fn max(&self, rhs: &Self) -> Self;
    fn dot(&self, rhs: &Self) -> UsfScalar;
    fn distance(&self, rhs: &Self) -> UsfScalar;
    fn angle_between(&self, rhs: &Self) -> UsfScalar;
    fn project(&self, onto: &Self) -> Self;
    fn reject(&self, onto: &Self) -> Self;
    fn reflect(&self, normal: &Self) -> Self;

    // ternary
    fn fma(&self, b: &Self, c: &Self) -> Self;
    fn lerp_f32(&self, rhs: &Self, t: NormalF32) -> Self;
    fn lerp_f64(&self, rhs: &Self, t: NormalF64) -> Self;
    fn smoothstep_f32(&self, rhs: &Self, t: NormalF32) -> Self;
    fn smoothstep_f64(&self, rhs: &Self, t: NormalF64) -> Self;
    fn clamp(&self, lo: &Self, hi: &Self) -> Self;

    // scalar-vector interop
    fn add_scalar(&self, rhs: &UsfScalar) -> Self;
    fn sub_scalar(&self, rhs: &UsfScalar) -> Self;
    fn mul_scalar(&self, rhs: &UsfScalar) -> Self;
    fn div_scalar(&self, rhs: &UsfScalar) -> Self;
    fn scale(&self, rhs: &UsfScalar) -> Self;

    // mixed with normal scalars
    fn add_f32(&self, rhs: NormalF32) -> Self;
    fn sub_f32(&self, rhs: NormalF32) -> Self;
    fn mul_f32(&self, rhs: NormalF32) -> Self;
    fn div_f32(&self, rhs: NormalF32) -> Self;
    fn rem_f32(&self, rhs: NormalF32) -> Self;
    fn min_f32(&self, rhs: NormalF32) -> Self;
    fn max_f32(&self, rhs: NormalF32) -> Self;
    fn clamp_f32(&self, lo: NormalF32, hi: NormalF32) -> Self;
    fn add_f64(&self, rhs: NormalF64) -> Self;
    fn sub_f64(&self, rhs: NormalF64) -> Self;
    fn mul_f64(&self, rhs: NormalF64) -> Self;
    fn div_f64(&self, rhs: NormalF64) -> Self;
    fn rem_f64(&self, rhs: NormalF64) -> Self;
    fn min_f64(&self, rhs: NormalF64) -> Self;
    fn max_f64(&self, rhs: NormalF64) -> Self;
    fn clamp_f64(&self, lo: NormalF64, hi: NormalF64) -> Self;
}

pub trait UsfVector3Extras: UsfVectorSpec<3> {
    fn cross(&self, rhs: &Self) -> Self;
}

pub trait UsfMatrixSpec<const R: usize, const C: usize>: Sized {
    fn zero() -> Self;
    fn from_rows(rows: [UsfVec<C>; R]) -> Self;
    fn to_rows(&self) -> [UsfVec<C>; R];
    fn row(&self, index: usize) -> UsfVec<C>;
    fn col(&self, index: usize) -> UsfVec<R>;
    fn transpose(&self) -> UsfMat<C, R>;

    fn add(&self, rhs: &Self) -> Self;
    fn sub(&self, rhs: &Self) -> Self;
    fn mul_elem(&self, rhs: &Self) -> Self;
    fn div_elem(&self, rhs: &Self) -> Self;
    fn mul_scalar(&self, rhs: &UsfScalar) -> Self;
    fn div_scalar(&self, rhs: &UsfScalar) -> Self;
    fn mul_vec(&self, rhs: &UsfVec<C>) -> UsfVec<R>;

    // mixed with normal scalars
    fn add_f32(&self, rhs: NormalF32) -> Self;
    fn sub_f32(&self, rhs: NormalF32) -> Self;
    fn mul_f32(&self, rhs: NormalF32) -> Self;
    fn div_f32(&self, rhs: NormalF32) -> Self;
    fn add_f64(&self, rhs: NormalF64) -> Self;
    fn sub_f64(&self, rhs: NormalF64) -> Self;
    fn mul_f64(&self, rhs: NormalF64) -> Self;
    fn div_f64(&self, rhs: NormalF64) -> Self;
}

pub trait UsfSquareMatrixSpec<const N: usize>: UsfMatrixSpec<N, N> {
    fn identity() -> Self;
    fn determinant(&self) -> UsfScalar;
    fn inverse(&self) -> Self;
    fn trace(&self) -> UsfScalar;
    fn powi(&self, exp: i32) -> Self;
}

pub trait UsfMatrixMulSpec<const R: usize, const K: usize, const C: usize> {
    fn mul(lhs: &UsfMat<R, K>, rhs: &UsfMat<K, C>) -> UsfMat<R, C>;
}

pub trait UsfQuatSpec: Sized {
    fn identity() -> Self;
    fn from_xyzw(x: UsfScalar, y: UsfScalar, z: UsfScalar, w: UsfScalar) -> Self;
    fn to_xyzw(&self) -> [UsfScalar; 4];
    fn normalize(&self) -> Self;
    fn conjugate(&self) -> Self;
    fn inverse(&self) -> Self;
    fn dot(&self, rhs: &Self) -> UsfScalar;
    fn add(&self, rhs: &Self) -> Self;
    fn sub(&self, rhs: &Self) -> Self;
    fn mul(&self, rhs: &Self) -> Self;
    fn mul_scalar(&self, rhs: &UsfScalar) -> Self;
    fn div_scalar(&self, rhs: &UsfScalar) -> Self;
    fn rotate_vec3(&self, rhs: &UsfVec<3>) -> UsfVec<3>;
    fn from_axis_angle(axis: &UsfVec<3>, angle_rad: &UsfScalar) -> Self;
    fn to_axis_angle(&self) -> (UsfVec<3>, UsfScalar);
    fn from_euler_xyz(x_rad: &UsfScalar, y_rad: &UsfScalar, z_rad: &UsfScalar) -> Self;
    fn to_euler_xyz(&self) -> [UsfScalar; 3];
    fn slerp_f32(&self, rhs: &Self, t: NormalF32) -> Self;
    fn slerp_f64(&self, rhs: &Self, t: NormalF64) -> Self;
    fn nlerp_f32(&self, rhs: &Self, t: NormalF32) -> Self;
    fn nlerp_f64(&self, rhs: &Self, t: NormalF64) -> Self;
    fn to_mat3(&self) -> UsfMat<3, 3>;
    fn from_mat3(value: &UsfMat<3, 3>) -> Self;
}

pub trait UsfTranslationSpec<const N: usize>: Sized {
    fn from_vec(value: UsfVec<N>) -> Self;
    fn to_vec(&self) -> UsfVec<N>;
    fn add(&self, rhs: &Self) -> Self;
    fn sub(&self, rhs: &Self) -> Self;
    fn scale(&self, rhs: &UsfScalar) -> Self;
}

pub trait UsfRotationSpec: Sized {
    fn from_quat(value: UsfQuat) -> Self;
    fn to_quat(&self) -> UsfQuat;
    fn compose(&self, rhs: &Self) -> Self;
}

pub trait UsfScaleSpec: Sized {
    fn make(log_base: UsfScalar, scale_index: i16, fractional_log_offset: UsfScalar) -> Self;
    fn log_base(&self) -> UsfScalar;
    fn scale_index(&self) -> i16;
    fn fractional_log_offset(&self) -> UsfScalar;
}

pub trait UsfTransformSpec: Sized {
    fn make(translation: UsfTranslation<3>, rotation: UsfRotation, scale: UsfScale) -> Self;
    fn translation(&self) -> UsfTranslation<3>;
    fn rotation(&self) -> UsfRotation;
    fn scale(&self) -> UsfScale;
    fn with_translation(&self, translation: UsfTranslation<3>) -> Self;
    fn with_rotation(&self, rotation: UsfRotation) -> Self;
    fn with_scale(&self, scale: UsfScale) -> Self;
}

// ---------------------------------------------------------------------------
// Dynamic-only model + dynamic-specific trait contracts
// ---------------------------------------------------------------------------

pub mod dynamic {
    use super::{
        NormalF32, NormalF64, UsfMat, UsfRotation, UsfScalar, UsfScale, UsfTransform,
        UsfTranslation, UsfVec,
    };

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfDVec {
        pub lanes: Vec<UsfScalar>,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfDMat {
        pub row_count: usize,
        pub col_count: usize,
        pub lanes_row_major: Vec<UsfScalar>,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfDTranslation(pub UsfDVec);

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfDTransform {
        pub translation: UsfDTranslation,
        pub rotation: UsfRotation,
        pub scale: UsfScale,
    }

    pub trait UsfDVectorSpec: Sized {
        fn zero(dim: usize) -> Self;
        fn one(dim: usize) -> Self;
        fn splat(dim: usize, value: &UsfScalar) -> Self;
        fn from_lanes(lanes: Vec<UsfScalar>) -> Self;
        fn to_lanes(&self) -> Vec<UsfScalar>;
        fn len(&self) -> usize;
        fn lane(&self, index: usize) -> UsfScalar;
        fn set_lane(&self, index: usize, lane: &UsfScalar) -> Self;

        // unary
        fn neg(&self) -> Self;
        fn abs(&self) -> Self;
        fn normalize(&self) -> Self;
        fn floor(&self) -> Self;
        fn ceil(&self) -> Self;
        fn round(&self) -> Self;
        fn fract(&self) -> Self;
        fn norm_l2(&self) -> UsfScalar;

        // binary
        fn add(&self, rhs: &Self) -> Self;
        fn sub(&self, rhs: &Self) -> Self;
        fn mul_elem(&self, rhs: &Self) -> Self;
        fn div_elem(&self, rhs: &Self) -> Self;
        fn min(&self, rhs: &Self) -> Self;
        fn max(&self, rhs: &Self) -> Self;
        fn dot(&self, rhs: &Self) -> UsfScalar;
        fn distance(&self, rhs: &Self) -> UsfScalar;
        fn angle_between(&self, rhs: &Self) -> UsfScalar;
        fn project(&self, onto: &Self) -> Self;
        fn reject(&self, onto: &Self) -> Self;
        fn reflect(&self, normal: &Self) -> Self;

        // ternary
        fn fma(&self, b: &Self, c: &Self) -> Self;
        fn lerp_f32(&self, rhs: &Self, t: NormalF32) -> Self;
        fn lerp_f64(&self, rhs: &Self, t: NormalF64) -> Self;
        fn smoothstep_f32(&self, rhs: &Self, t: NormalF32) -> Self;
        fn smoothstep_f64(&self, rhs: &Self, t: NormalF64) -> Self;
        fn clamp(&self, lo: &Self, hi: &Self) -> Self;

        // scalar-vector interop
        fn add_scalar(&self, rhs: &UsfScalar) -> Self;
        fn sub_scalar(&self, rhs: &UsfScalar) -> Self;
        fn mul_scalar(&self, rhs: &UsfScalar) -> Self;
        fn div_scalar(&self, rhs: &UsfScalar) -> Self;
        fn scale(&self, rhs: &UsfScalar) -> Self;

        // mixed with normal scalars
        fn add_f32(&self, rhs: NormalF32) -> Self;
        fn sub_f32(&self, rhs: NormalF32) -> Self;
        fn mul_f32(&self, rhs: NormalF32) -> Self;
        fn div_f32(&self, rhs: NormalF32) -> Self;
        fn rem_f32(&self, rhs: NormalF32) -> Self;
        fn min_f32(&self, rhs: NormalF32) -> Self;
        fn max_f32(&self, rhs: NormalF32) -> Self;
        fn clamp_f32(&self, lo: NormalF32, hi: NormalF32) -> Self;
        fn add_f64(&self, rhs: NormalF64) -> Self;
        fn sub_f64(&self, rhs: NormalF64) -> Self;
        fn mul_f64(&self, rhs: NormalF64) -> Self;
        fn div_f64(&self, rhs: NormalF64) -> Self;
        fn rem_f64(&self, rhs: NormalF64) -> Self;
        fn min_f64(&self, rhs: NormalF64) -> Self;
        fn max_f64(&self, rhs: NormalF64) -> Self;
        fn clamp_f64(&self, lo: NormalF64, hi: NormalF64) -> Self;

        fn into_static<const N: usize>(&self) -> UsfVec<N>;
        fn from_static<const N: usize>(value: &UsfVec<N>) -> Self;
    }

    pub trait UsfDMatrixSpec: Sized {
        fn zero(rows: usize, cols: usize) -> Self;
        fn from_rows(rows: Vec<UsfDVec>) -> Self;
        fn from_row_major(rows: usize, cols: usize, lanes: Vec<UsfScalar>) -> Self;
        fn to_rows(&self) -> Vec<UsfDVec>;
        fn to_row_major(&self) -> Vec<UsfScalar>;
        fn row_count(&self) -> usize;
        fn col_count(&self) -> usize;
        fn row(&self, index: usize) -> UsfDVec;
        fn col(&self, index: usize) -> UsfDVec;
        fn lane(&self, row: usize, col: usize) -> UsfScalar;
        fn set_lane(&self, row: usize, col: usize, lane: &UsfScalar) -> Self;

        fn add(&self, rhs: &Self) -> Self;
        fn sub(&self, rhs: &Self) -> Self;
        fn mul_elem(&self, rhs: &Self) -> Self;
        fn div_elem(&self, rhs: &Self) -> Self;
        fn mul_scalar(&self, rhs: &UsfScalar) -> Self;
        fn div_scalar(&self, rhs: &UsfScalar) -> Self;
        fn transpose(&self) -> Self;
        fn mul_vec(&self, rhs: &UsfDVec) -> UsfDVec;
        fn mul_mat(&self, rhs: &Self) -> Self;

        fn add_f32(&self, rhs: NormalF32) -> Self;
        fn sub_f32(&self, rhs: NormalF32) -> Self;
        fn mul_f32(&self, rhs: NormalF32) -> Self;
        fn div_f32(&self, rhs: NormalF32) -> Self;
        fn add_f64(&self, rhs: NormalF64) -> Self;
        fn sub_f64(&self, rhs: NormalF64) -> Self;
        fn mul_f64(&self, rhs: NormalF64) -> Self;
        fn div_f64(&self, rhs: NormalF64) -> Self;

        fn into_static<const R: usize, const C: usize>(&self) -> UsfMat<R, C>;
        fn from_static<const R: usize, const C: usize>(value: &UsfMat<R, C>) -> Self;
    }

    pub trait UsfDTranslationSpec: Sized {
        fn from_vec(value: UsfDVec) -> Self;
        fn to_vec(&self) -> UsfDVec;
        fn add(&self, rhs: &Self) -> Self;
        fn sub(&self, rhs: &Self) -> Self;
        fn scale(&self, rhs: &UsfScalar) -> Self;
        fn into_static<const N: usize>(&self) -> UsfTranslation<N>;
        fn from_static<const N: usize>(value: &UsfTranslation<N>) -> Self;
    }

    pub trait UsfDTransformSpec: Sized {
        fn make(translation: UsfDTranslation, rotation: UsfRotation, scale: UsfScale) -> Self;
        fn translation(&self) -> UsfDTranslation;
        fn rotation(&self) -> UsfRotation;
        fn scale(&self) -> UsfScale;
        fn with_translation(&self, translation: UsfDTranslation) -> Self;
        fn with_rotation(&self, rotation: UsfRotation) -> Self;
        fn with_scale(&self, scale: UsfScale) -> Self;
        fn into_static(&self) -> UsfTransform;
        fn from_static(value: &UsfTransform) -> Self;
    }
}
