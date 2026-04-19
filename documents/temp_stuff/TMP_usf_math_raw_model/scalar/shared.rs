#![allow(dead_code)]

use crate::utils::one_of::OneOf2;

pub trait ScalarType: Clone + 'static {}
pub trait IntegerType: ScalarType {}
pub trait SignedIntegerType: IntegerType {}
pub trait UnsignedIntegerType: IntegerType {}
pub trait FloatType: ScalarType {}

impl ScalarType for i8 {}
impl IntegerType for i8 {}
impl SignedIntegerType for i8 {}

impl ScalarType for i16 {}
impl IntegerType for i16 {}
impl SignedIntegerType for i16 {}

impl ScalarType for i32 {}
impl IntegerType for i32 {}
impl SignedIntegerType for i32 {}

impl ScalarType for i64 {}
impl IntegerType for i64 {}
impl SignedIntegerType for i64 {}

impl ScalarType for i128 {}
impl IntegerType for i128 {}
impl SignedIntegerType for i128 {}

impl ScalarType for isize {}
impl IntegerType for isize {}
impl SignedIntegerType for isize {}

impl ScalarType for u8 {}
impl IntegerType for u8 {}
impl UnsignedIntegerType for u8 {}

impl ScalarType for u16 {}
impl IntegerType for u16 {}
impl UnsignedIntegerType for u16 {}

impl ScalarType for u32 {}
impl IntegerType for u32 {}
impl UnsignedIntegerType for u32 {}

impl ScalarType for u64 {}
impl IntegerType for u64 {}
impl UnsignedIntegerType for u64 {}

impl ScalarType for u128 {}
impl IntegerType for u128 {}
impl UnsignedIntegerType for u128 {}

impl ScalarType for usize {}
impl IntegerType for usize {}
impl UnsignedIntegerType for usize {}

impl ScalarType for f32 {}
impl FloatType for f32 {}

impl ScalarType for f64 {}
impl FloatType for f64 {}

pub trait ScalarCoreOps: Clone + Sized {
    fn zero() -> Self {
        todo!()
    }
    fn one() -> Self {
        todo!()
    }
    fn two() -> Self {
        todo!()
    }
    fn ten() -> Self {
        todo!()
    }
    fn neg_one() -> Self {
        todo!()
    }
    fn pi() -> Self {
        todo!()
    }
    fn tau() -> Self {
        todo!()
    }
    fn e() -> Self {
        todo!()
    }
    fn nan() -> Self {
        todo!()
    }
    fn infinity() -> Self {
        todo!()
    }
    fn neg_infinity() -> Self {
        todo!()
    }
    fn parse_decimal(_text: &str) -> Self {
        todo!()
    }
    fn to_decimal_string(&self) -> String {
        todo!()
    }
    fn to_scientific_string(&self) -> String {
        todo!()
    }
    fn normalize(&self) -> Self {
        todo!()
    }
    fn is_zero(&self) -> bool {
        todo!()
    }
    fn is_one(&self) -> bool {
        todo!()
    }
    fn is_nan(&self) -> bool {
        todo!()
    }
    fn is_infinite(&self) -> bool {
        todo!()
    }
    fn is_finite(&self) -> bool {
        todo!()
    }
    fn is_positive(&self) -> bool {
        todo!()
    }
    fn is_negative(&self) -> bool {
        todo!()
    }
    fn signum(&self) -> Self {
        todo!()
    }
    fn recip(&self) -> Self {
        todo!()
    }
    fn square(&self) -> Self {
        todo!()
    }
    fn cube(&self) -> Self {
        todo!()
    }
    fn sqrt(&self) -> Self {
        todo!()
    }
    fn cbrt(&self) -> Self {
        todo!()
    }
    fn exp(&self) -> Self {
        todo!()
    }
    fn exp2(&self) -> Self {
        todo!()
    }
    fn exp10(&self) -> Self {
        todo!()
    }
    fn ln(&self) -> Self {
        todo!()
    }
    fn log2(&self) -> Self {
        todo!()
    }
    fn log10(&self) -> Self {
        todo!()
    }
    fn sin(&self) -> Self {
        todo!()
    }
    fn cos(&self) -> Self {
        todo!()
    }
    fn tan(&self) -> Self {
        todo!()
    }
    fn asin(&self) -> Self {
        todo!()
    }
    fn acos(&self) -> Self {
        todo!()
    }
    fn atan(&self) -> Self {
        todo!()
    }
    fn sinh(&self) -> Self {
        todo!()
    }
    fn cosh(&self) -> Self {
        todo!()
    }
    fn tanh(&self) -> Self {
        todo!()
    }
    fn floor(&self) -> Self {
        todo!()
    }
    fn ceil(&self) -> Self {
        todo!()
    }
    fn round(&self) -> Self {
        todo!()
    }
    fn trunc(&self) -> Self {
        todo!()
    }
    fn fract(&self) -> Self {
        todo!()
    }
    fn neg(&self) -> Self {
        todo!()
    }
    fn abs(&self) -> Self {
        todo!()
    }
    fn add(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn sub(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn mul(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn div(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn rem(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn min(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn max(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn clamp(&self, _lo: Self, _hi: Self) -> Self {
        todo!()
    }
    fn pow(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn atan2(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn hypot(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn mod_euclid(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn fma(&self, _b: Self, _c: Self) -> Self {
        todo!()
    }
    fn lerp<ScalarB: ScalarContract>(&self, _rhs: Self, _t: OneOf2<Self, ScalarB>) -> Self {
        todo!()
    }
    fn smoothstep<ScalarB: ScalarContract>(&self, _edge0: Self, _edge1: Self, _t: OneOf2<Self, ScalarB>) -> Self {
        todo!()
    }
    fn cmp_eq(&self, _rhs: Self) -> bool {
        todo!()
    }
    fn cmp_ne(&self, _rhs: Self) -> bool {
        todo!()
    }
    fn cmp_lt(&self, _rhs: Self) -> bool {
        todo!()
    }
    fn cmp_le(&self, _rhs: Self) -> bool {
        todo!()
    }
    fn cmp_gt(&self, _rhs: Self) -> bool {
        todo!()
    }
    fn cmp_ge(&self, _rhs: Self) -> bool {
        todo!()
    }
}

pub trait ScalarFieldOps: ScalarCoreOps {
    fn get_value(&self) -> Self {
        todo!()
    }
    fn set_value(&mut self, _value: Self) {
        todo!()
    }
}

pub trait ScalarBridgeOps: ScalarCoreOps {
    fn from_scalar<ScalarB: ScalarContract>(_value: OneOf2<Self, ScalarB>) -> Self {
        todo!()
    }
    fn to_scalar<ScalarB: ScalarContract>(&self) -> OneOf2<Self, ScalarB> {
        todo!()
    }
}

pub trait ScalarContract: ScalarCoreOps + ScalarFieldOps + ScalarBridgeOps {}

impl<T> ScalarContract for T where T: ScalarCoreOps + ScalarFieldOps + ScalarBridgeOps {}
