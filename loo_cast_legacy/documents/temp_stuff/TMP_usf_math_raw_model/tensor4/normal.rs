#![allow(dead_code)]

use super::super::field::Field;
use super::super::matrix::normal::NormalMatrix;
use super::super::scalar::normal::NormalScalar;
use super::super::tensor::normal::NormalTensor;
use super::super::vector::normal::NormalVector;

#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct NormalTensor4<const A: usize, const B: usize, const C: usize, const D: usize>(pub(super) Field<NormalTensor4Repr<A, B, C, D>>);

#[derive(Clone, Debug, PartialEq)]
pub(super) enum NormalTensor4Repr<const A: usize, const B: usize, const C: usize, const D: usize> {
    // CONTRACT: A,B,C,D >= 2. Any axis == 1 is reducible and forbidden.
    I8([[[[i8; D]; C]; B]; A]),
    I16([[[[i16; D]; C]; B]; A]),
    I32([[[[i32; D]; C]; B]; A]),
    I64([[[[i64; D]; C]; B]; A]),
    I128([[[[i128; D]; C]; B]; A]),
    Isize([[[[isize; D]; C]; B]; A]),
    U8([[[[u8; D]; C]; B]; A]),
    U16([[[[u16; D]; C]; B]; A]),
    U32([[[[u32; D]; C]; B]; A]),
    U64([[[[u64; D]; C]; B]; A]),
    U128([[[[u128; D]; C]; B]; A]),
    Usize([[[[usize; D]; C]; B]; A]),
    F32([[[[f32; D]; C]; B]; A]),
    F64([[[[f64; D]; C]; B]; A]),
}

pub type NormalTensor2x2x2x2 = NormalTensor4<2, 2, 2, 2>;
pub type NormalTensor2x2x3x4 = NormalTensor4<2, 2, 3, 4>;
pub type NormalTensor2x3x3x4 = NormalTensor4<2, 3, 3, 4>;
pub type NormalTensor3x3x3x3 = NormalTensor4<3, 3, 3, 3>;
pub type NormalTensor4x4x4x4 = NormalTensor4<4, 4, 4, 4>;
pub type NormalTensor2x4x4x8 = NormalTensor4<2, 4, 4, 8>;
pub type NormalTensor8x4x4x2 = NormalTensor4<8, 4, 4, 2>;

impl<const A: usize, const B: usize, const C: usize, const D: usize> super::shared::Tensor4CoreOps<A, B, C, D> for NormalTensor4<A, B, C, D> {}

impl<const A: usize, const B: usize, const C: usize, const D: usize> super::shared::Tensor4FieldOps<A, B, C, D> for NormalTensor4<A, B, C, D> {}

impl<const A: usize, const B: usize, const C: usize, const D: usize> super::shared::Tensor4BridgeOps<A, B, C, D> for NormalTensor4<A, B, C, D> {}

impl<const A: usize, const B: usize, const C: usize, const D: usize> super::shared::Tensor4ProjectionCoreOps<A, B, C, D> for NormalTensor4<A, B, C, D> {}
