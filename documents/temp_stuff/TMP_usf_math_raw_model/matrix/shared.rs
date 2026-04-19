#![allow(dead_code)]

use super::super::scalar::shared::SignedIntegerType;
use crate::utils::one_of::OneOf2;

pub trait MatrixCoreOps<Scalar, RowVector, ColVector, TransposedMatrix, const R: usize, const C: usize>: Clone + Sized {
    fn zero() -> Self {
        todo!()
    }
    fn from_rows(_rows: [RowVector; R]) -> Self {
        todo!()
    }
    fn to_rows(&self) -> [RowVector; R] {
        todo!()
    }
    fn transpose(&self) -> TransposedMatrix {
        todo!()
    }
    fn mul_elem(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn div_elem(&self, _rhs: Self) -> Self {
        todo!()
    }
    fn add<ScalarB>(&self, _rhs: OneOf2<Self, OneOf2<Scalar, ScalarB>>) -> Self {
        todo!()
    }
    fn sub<ScalarB>(&self, _rhs: OneOf2<Self, OneOf2<Scalar, ScalarB>>) -> Self {
        todo!()
    }
    fn mul<ScalarB>(&self, _rhs: OneOf2<Self, OneOf2<Scalar, ScalarB>>) -> Self {
        todo!()
    }
    fn div<ScalarB>(&self, _rhs: OneOf2<Self, OneOf2<Scalar, ScalarB>>) -> Self {
        todo!()
    }
    fn rem<ScalarB>(&self, _rhs: OneOf2<Self, OneOf2<Scalar, ScalarB>>) -> Self {
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
    fn mul_vec(&self, _rhs: RowVector) -> ColVector {
        todo!()
    }
    fn get_row_count(&self) -> usize {
        todo!()
    }
    fn get_col_count(&self) -> usize {
        todo!()
    }
    fn get_shape(&self) -> (usize, usize) {
        todo!()
    }
    fn get_element_count(&self) -> usize {
        todo!()
    }
}

pub trait MatrixFieldOps<Scalar, RowVector, ColVector, TransposedMatrix, const R: usize, const C: usize>:
    MatrixCoreOps<Scalar, RowVector, ColVector, TransposedMatrix, R, C>
{
    fn get_row(&self, _index: usize) -> RowVector {
        todo!()
    }
    fn get_col(&self, _index: usize) -> ColVector {
        todo!()
    }
    fn get_lane(&self, _row: usize, _col: usize) -> Scalar {
        todo!()
    }
    fn set_lane(&mut self, _row: usize, _col: usize, _value: Scalar) {
        todo!()
    }
}

pub trait SquareMatrixCoreOps<Scalar, Vector, const D: usize>: MatrixCoreOps<Scalar, Vector, Vector, Self, D, D> {
    fn identity() -> Self {
        todo!()
    }
    fn determinant<ScalarB>(&self) -> OneOf2<Scalar, ScalarB> {
        todo!()
    }
    fn inverse(&self) -> Self {
        todo!()
    }
    fn trace<ScalarB>(&self) -> OneOf2<Scalar, ScalarB> {
        todo!()
    }
    fn powi<T: SignedIntegerType>(&self, _exp: T) -> Self {
        todo!()
    }
    fn mul_mat(&self, _rhs: Self) -> Self {
        todo!()
    }
}

pub trait SquareMatrixBridgeOps<Scalar, Vector, const D: usize>: SquareMatrixCoreOps<Scalar, Vector, D> {}

pub trait MatrixOps<Scalar, RowVector, ColVector, TransposedMatrix, const R: usize, const C: usize>:
    MatrixCoreOps<Scalar, RowVector, ColVector, TransposedMatrix, R, C> + MatrixFieldOps<Scalar, RowVector, ColVector, TransposedMatrix, R, C>
{
}
impl<T, Scalar, RowVector, ColVector, TransposedMatrix, const R: usize, const C: usize> MatrixOps<Scalar, RowVector, ColVector, TransposedMatrix, R, C> for T where
    T: MatrixCoreOps<Scalar, RowVector, ColVector, TransposedMatrix, R, C> + MatrixFieldOps<Scalar, RowVector, ColVector, TransposedMatrix, R, C>
{
}

pub trait SquareMatrixOps<Scalar, Vector, const D: usize>: SquareMatrixCoreOps<Scalar, Vector, D> + SquareMatrixBridgeOps<Scalar, Vector, D> {}
impl<T, Scalar, Vector, const D: usize> SquareMatrixOps<Scalar, Vector, D> for T where
    T: SquareMatrixCoreOps<Scalar, Vector, D> + SquareMatrixBridgeOps<Scalar, Vector, D>
{
}
