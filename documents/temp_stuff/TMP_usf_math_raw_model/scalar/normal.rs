#![allow(dead_code)]

use super::super::field::Field;

#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct NormalScalar(pub(super) Field<NormalScalarRepr>);

#[derive(Clone, Debug, PartialEq)]
pub(super) enum NormalScalarRepr {
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

#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct NormalFractionalScalar(pub(super) Field<NormalFractionalScalarRepr>);

#[derive(Clone, Debug, PartialEq)]
pub(super) enum NormalFractionalScalarRepr {
    F32(f32),
    F64(f64),
}

impl super::shared::ScalarCoreOps for NormalScalar {}
impl super::shared::ScalarFieldOps for NormalScalar {}
impl super::shared::ScalarBridgeOps for NormalScalar {}

impl super::shared::ScalarCoreOps for NormalFractionalScalar {}
impl super::shared::ScalarFieldOps for NormalFractionalScalar {}
impl super::shared::ScalarBridgeOps for NormalFractionalScalar {}
impl super::shared::FractionalScalarContract for NormalFractionalScalar {}
