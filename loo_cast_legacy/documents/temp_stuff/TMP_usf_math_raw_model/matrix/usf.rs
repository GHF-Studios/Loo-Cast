#![allow(dead_code)]

use super::super::field::Field;
use super::super::vector::usf::UsfVector;
pub use super::aliases::{MatrixOrScalar, MatrixOrVector, UsfOrNormalMatrix};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfMatrix<const R: usize, const C: usize> {
    // CONTRACT: R >= 2 and C >= 2. 1xN / Nx1 are vector-equivalent and forbidden.
    pub(super) rows: Field<[UsfVector<C>; R]>,
}

pub type UsfMatrix2x2 = UsfMatrix<2, 2>;
pub type UsfMatrix2x3 = UsfMatrix<2, 3>;
pub type UsfMatrix2x4 = UsfMatrix<2, 4>;
pub type UsfMatrix3x2 = UsfMatrix<3, 2>;
pub type UsfMatrix3x3 = UsfMatrix<3, 3>;
pub type UsfMatrix3x4 = UsfMatrix<3, 4>;
pub type UsfMatrix4x2 = UsfMatrix<4, 2>;
pub type UsfMatrix4x3 = UsfMatrix<4, 3>;
pub type UsfMatrix4x4 = UsfMatrix<4, 4>;
pub type UsfMatrix5x5 = UsfMatrix<5, 5>;
pub type UsfMatrix6x6 = UsfMatrix<6, 6>;
pub type UsfMatrix7x7 = UsfMatrix<7, 7>;
pub type UsfMatrix8x8 = UsfMatrix<8, 8>;

impl<const R: usize, const C: usize> super::shared::MatrixCoreOps<R, C> for UsfMatrix<R, C> {}

impl<const R: usize, const C: usize> super::shared::MatrixFieldOps<R, C> for UsfMatrix<R, C> {}

impl<const R: usize, const C: usize> super::shared::MatrixBridgeOps<R, C> for UsfMatrix<R, C> {}

impl<const D: usize> super::shared::SquareMatrixCoreOps<D> for UsfMatrix<D, D> {}

impl<const D: usize> super::shared::SquareMatrixBridgeOps<D> for UsfMatrix<D, D> {}
