use super::super::field::Field;

/// General-purpose scalar wrapper around heterogeneous primitive numeric reprs.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NormalScalar(pub Field<NormalScalarRepr>);

/// Primitive representation variants for [`NormalScalar`].
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum NormalScalarRepr {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    F32(f32),
    F64(f64),
}

/// Fractional-only scalar wrapper around float reprs.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NormalFractionalScalar(pub Field<NormalFractionalScalarRepr>);

/// Primitive representation variants for [`NormalFractionalScalar`].
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum NormalFractionalScalarRepr {
    F32(f32),
    F64(f64),
}

impl super::shared::ScalarConstOps for NormalScalar {
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

    fn max() -> Self {
        todo!()
    }

    fn min() -> Self {
        todo!()
    }

    fn neg_one() -> Self {
        todo!()
    }

    fn epsilon() -> Self {
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
}

impl super::shared::ScalarCoreOps for NormalScalar {
    fn from_decimal_str(_s: &str) -> Self {
        todo!()
    }

    fn to_decimal_str(&self) -> String {
        todo!()
    }

    fn from_scientific_str(_s: &str) -> Self {
        todo!()
    }

    fn to_scientific_str(&self) -> String {
        todo!()
    }

    fn from_digits(
        _negative: bool,
        _int_digits: super::shared::ScalarIntDigitBuffer,
        _frac_digits: super::shared::ScalarFracDigitBuffer,
        _radix_index: i8,
    ) -> Self {
        todo!()
    }

    fn to_digits(&self) -> (bool, super::shared::ScalarIntDigitBuffer, super::shared::ScalarFracDigitBuffer, i8) {
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

    fn log(&self, _base: super::aliases::UsfOrNormalScalar) -> Self {
        todo!()
    }

    fn add(&self, _rhs: super::aliases::UsfOrNormalScalar) -> Self {
        todo!()
    }

    fn sub(&self, _rhs: super::aliases::UsfOrNormalScalar) -> Self {
        todo!()
    }

    fn mul(&self, _rhs: super::aliases::UsfOrNormalScalar) -> Self {
        todo!()
    }

    fn div(&self, _rhs: super::aliases::UsfOrNormalScalar) -> Self {
        todo!()
    }

    fn rem(&self, _rhs: super::aliases::UsfOrNormalScalar) -> Self {
        todo!()
    }

    fn pow(&self, _rhs: super::aliases::UsfOrNormalScalar) -> Self {
        todo!()
    }
}
impl super::shared::ScalarFieldOps for NormalScalar {}
impl super::shared::ScalarBridgeOps for NormalScalar {}

impl super::shared::ScalarConstOps for NormalFractionalScalar {
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

    fn max() -> Self {
        todo!()
    }

    fn min() -> Self {
        todo!()
    }

    fn neg_one() -> Self {
        todo!()
    }

    fn epsilon() -> Self {
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
}

impl super::shared::ScalarCoreOps for NormalFractionalScalar {
    fn from_decimal_str(_s: &str) -> Self {
        todo!()
    }

    fn to_decimal_str(&self) -> String {
        todo!()
    }

    fn from_scientific_str(_s: &str) -> Self {
        todo!()
    }

    fn to_scientific_str(&self) -> String {
        todo!()
    }

    fn from_digits(
        _negative: bool,
        _int_digits: super::shared::ScalarIntDigitBuffer,
        _frac_digits: super::shared::ScalarFracDigitBuffer,
        _radix_index: i8,
    ) -> Self {
        todo!()
    }

    fn to_digits(&self) -> (bool, super::shared::ScalarIntDigitBuffer, super::shared::ScalarFracDigitBuffer, i8) {
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

    fn log(&self, _base: super::aliases::UsfOrNormalScalar) -> Self {
        todo!()
    }

    fn add(&self, _rhs: super::aliases::UsfOrNormalScalar) -> Self {
        todo!()
    }

    fn sub(&self, _rhs: super::aliases::UsfOrNormalScalar) -> Self {
        todo!()
    }

    fn mul(&self, _rhs: super::aliases::UsfOrNormalScalar) -> Self {
        todo!()
    }

    fn div(&self, _rhs: super::aliases::UsfOrNormalScalar) -> Self {
        todo!()
    }

    fn rem(&self, _rhs: super::aliases::UsfOrNormalScalar) -> Self {
        todo!()
    }

    fn pow(&self, _rhs: super::aliases::UsfOrNormalScalar) -> Self {
        todo!()
    }
}
impl super::shared::ScalarFieldOps for NormalFractionalScalar {}
impl super::shared::ScalarBridgeOps for NormalFractionalScalar {}
impl super::shared::FractionalScalarContract for NormalFractionalScalar {}
