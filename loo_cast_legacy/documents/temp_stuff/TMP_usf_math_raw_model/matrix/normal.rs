#![allow(dead_code)]

use super::super::field::Field;
use super::super::scalar::normal::NormalScalar;
use super::super::vector::normal::NormalVector;

#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct NormalMatrix<const R: usize, const C: usize>(pub(super) Field<NormalMatrixRepr<R, C>>);

#[derive(Clone, Debug, PartialEq)]
pub(super) enum NormalMatrixRepr<const R: usize, const C: usize> {
    Generic(NormalMatrixGenericRepr<R, C>),
    Concrete(NormalMatrixConcreteRepr),
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum NormalMatrixGenericRepr<const R: usize, const C: usize> {
    // CONTRACT: R >= 2 and C >= 2. 1xN / Nx1 are vector-equivalent and forbidden.
    I8([[i8; C]; R]),
    I16([[i16; C]; R]),
    I32([[i32; C]; R]),
    I64([[i64; C]; R]),
    I128([[i128; C]; R]),
    Isize([[isize; C]; R]),
    U8([[u8; C]; R]),
    U16([[u16; C]; R]),
    U32([[u32; C]; R]),
    U64([[u64; C]; R]),
    U128([[u128; C]; R]),
    Usize([[usize; C]; R]),
    F32([[f32; C]; R]),
    F64([[f64; C]; R]),
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum NormalMatrixConcreteRepr {
    // CONTRACT: Concrete variant shape must match the enclosing `NormalMatrix<R, C>`.
    // - Mat2/DMat2 => (R, C) = (2, 2)
    // - Mat3/Mat3A/DMat3 => (R, C) = (3, 3)
    // - Mat4/DMat4 => (R, C) = (4, 4)
    // - Affine2/Affine3A/DAffine2/DAffine3: shape mapping is conversion-contract-defined.
    Mat2(bevy::math::Mat2),
    Mat3(bevy::math::Mat3),
    Mat3A(bevy::math::Mat3A),
    Mat4(bevy::math::Mat4),
    DMat2(bevy::math::DMat2),
    DMat3(bevy::math::DMat3),
    DMat4(bevy::math::DMat4),
    Affine2(bevy::math::Affine2),
    Affine3A(bevy::math::Affine3A),
    DAffine2(bevy::math::DAffine2),
    DAffine3(bevy::math::DAffine3),
}

pub type NormalMatrix2x2 = NormalMatrix<2, 2>;
pub type NormalMatrix2x3 = NormalMatrix<2, 3>;
pub type NormalMatrix2x4 = NormalMatrix<2, 4>;
pub type NormalMatrix3x2 = NormalMatrix<3, 2>;
pub type NormalMatrix3x3 = NormalMatrix<3, 3>;
pub type NormalMatrix3x4 = NormalMatrix<3, 4>;
pub type NormalMatrix4x2 = NormalMatrix<4, 2>;
pub type NormalMatrix4x3 = NormalMatrix<4, 3>;
pub type NormalMatrix4x4 = NormalMatrix<4, 4>;
pub type NormalMatrix5x5 = NormalMatrix<5, 5>;
pub type NormalMatrix6x6 = NormalMatrix<6, 6>;
pub type NormalMatrix7x7 = NormalMatrix<7, 7>;
pub type NormalMatrix8x8 = NormalMatrix<8, 8>;

impl<const R: usize, const C: usize> super::shared::MatrixCoreOps<R, C> for NormalMatrix<R, C> {}

impl<const R: usize, const C: usize> super::shared::MatrixFieldOps<R, C> for NormalMatrix<R, C> {}

impl<const R: usize, const C: usize> super::shared::MatrixBridgeOps<R, C> for NormalMatrix<R, C> {}

impl<const D: usize> super::shared::SquareMatrixCoreOps<D> for NormalMatrix<D, D> {}

impl<const D: usize> super::shared::SquareMatrixBridgeOps<D> for NormalMatrix<D, D> {}
