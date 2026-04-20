#![allow(dead_code)]

use super::super::aliases::OutputMode;
use super::super::scalar::aliases::{UsfOrNormalDecimalScalar, UsfOrNormalScalar};
use super::super::scalar::shared::SignedIntegerType;
use crate::utils::one_of::OneOf2;

pub trait MatrixCoreOps<RowVector, ColVector, TransposedMatrix, const R: usize, const C: usize>: Clone + Sized {
    /// Returns additive identity matrix.
    fn zero() -> Self {
        todo!()
    }
    /// Builds matrix from rows.
    fn from_rows(_rows: [RowVector; R]) -> Self {
        todo!()
    }
    /// Returns row array.
    fn to_rows(&self) -> [RowVector; R] {
        todo!()
    }
    /// Returns transposed matrix.
    fn transpose(&self) -> TransposedMatrix {
        todo!()
    }
    /// Element-wise multiplication.
    fn mul_elem(&self, _rhs: Self) -> Self {
        todo!()
    }
    /// Element-wise division.
    fn div_elem(&self, _rhs: Self) -> Self {
        todo!()
    }
    /// Adds matrix or scalar operand.
    fn add(&self, _rhs: OneOf2<Self, UsfOrNormalScalar>) -> Self {
        todo!()
    }
    /// Subtracts matrix or scalar operand.
    fn sub(&self, _rhs: OneOf2<Self, UsfOrNormalScalar>) -> Self {
        todo!()
    }
    /// Multiplies matrix or scalar operand.
    fn mul(&self, _rhs: OneOf2<Self, UsfOrNormalScalar>) -> Self {
        todo!()
    }
    /// Divides matrix or scalar operand.
    fn div(&self, _rhs: OneOf2<Self, UsfOrNormalScalar>) -> Self {
        todo!()
    }
    /// Computes remainder with matrix or scalar operand.
    fn rem(&self, _rhs: OneOf2<Self, UsfOrNormalScalar>) -> Self {
        todo!()
    }
    /// Returns element-wise minimum.
    fn min(&self, _rhs: Self) -> Self {
        todo!()
    }
    /// Returns element-wise maximum.
    fn max(&self, _rhs: Self) -> Self {
        todo!()
    }
    /// Clamps each matrix component to `[lo, hi]`.
    fn clamp(&self, _lo: Self, _hi: Self) -> Self {
        todo!()
    }
    /// Matrix-vector product.
    fn mul_vec(&self, _rhs: RowVector) -> ColVector {
        todo!()
    }
    /// Returns row count.
    fn get_row_count(&self) -> usize {
        todo!()
    }
    /// Returns column count.
    fn get_col_count(&self) -> usize {
        todo!()
    }
    /// Returns `(rows, cols)`.
    fn get_shape(&self) -> (usize, usize) {
        todo!()
    }
    /// Returns element count.
    fn get_element_count(&self) -> usize {
        todo!()
    }
}

pub trait MatrixFieldOps<RowVector, ColVector, TransposedMatrix, const R: usize, const C: usize>:
    MatrixCoreOps<RowVector, ColVector, TransposedMatrix, R, C>
{
    /// Returns row by index.
    fn get_row(&self, _index: usize) -> RowVector {
        todo!()
    }
    /// Returns column by index.
    fn get_col(&self, _index: usize) -> ColVector {
        todo!()
    }
    /// Returns matrix component.
    fn get_component(&self, _row: usize, _col: usize, _output_mode: OutputMode) -> UsfOrNormalScalar {
        todo!()
    }
    /// Sets matrix component.
    fn set_component(&mut self, _row: usize, _col: usize, _value: UsfOrNormalScalar) {
        todo!()
    }
}

pub trait MatrixBridgeOps<RowVector, ColVector, TransposedMatrix, const R: usize, const C: usize>:
    MatrixCoreOps<RowVector, ColVector, TransposedMatrix, R, C>
{
}

pub trait SquareMatrixCoreOps<Vector, const D: usize>: MatrixCoreOps<Vector, Vector, Self, D, D> {
    /// Returns identity matrix.
    fn identity() -> Self {
        todo!()
    }
    /// Returns determinant in requested output mode.
    fn determinant(&self, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }
    /// Returns inverse matrix.
    fn inverse(&self) -> Self {
        todo!()
    }
    /// Returns trace in requested output mode.
    fn trace(&self, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }
    /// Raises matrix to signed integer power.
    fn powi<T: SignedIntegerType>(&self, _exp: T) -> Self {
        todo!()
    }
    /// Performs square matrix product.
    fn mul_mat(&self, _rhs: Self) -> Self {
        todo!()
    }
}

pub trait SquareMatrixBridgeOps<Vector, const D: usize>: SquareMatrixCoreOps<Vector, D> {}

pub trait MatrixContract<RowVector, ColVector, TransposedMatrix, const R: usize, const C: usize>:
    MatrixCoreOps<RowVector, ColVector, TransposedMatrix, R, C>
    + MatrixFieldOps<RowVector, ColVector, TransposedMatrix, R, C>
    + MatrixBridgeOps<RowVector, ColVector, TransposedMatrix, R, C>
{
}
impl<T, RowVector, ColVector, TransposedMatrix, const R: usize, const C: usize> MatrixContract<RowVector, ColVector, TransposedMatrix, R, C> for T where
    T: MatrixCoreOps<RowVector, ColVector, TransposedMatrix, R, C>
        + MatrixFieldOps<RowVector, ColVector, TransposedMatrix, R, C>
        + MatrixBridgeOps<RowVector, ColVector, TransposedMatrix, R, C>
{
}

pub trait SquareMatrixContract<Vector, const D: usize>:
    SquareMatrixCoreOps<Vector, D> + MatrixFieldOps<Vector, Vector, Self, D, D> + SquareMatrixBridgeOps<Vector, D>
{
}
impl<T, Vector, const D: usize> SquareMatrixContract<Vector, D> for T where
    T: SquareMatrixCoreOps<Vector, D> + MatrixFieldOps<Vector, Vector, Self, D, D> + SquareMatrixBridgeOps<Vector, D>
{
}
