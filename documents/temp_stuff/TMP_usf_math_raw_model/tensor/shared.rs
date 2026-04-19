#![allow(dead_code)]

use super::super::scalar::shared::ScalarContract;
use crate::utils::one_of::OneOf2;

pub trait TensorCoreOps<Scalar: ScalarContract, MatrixBc, VectorC, const A: usize, const B: usize, const C: usize>: Clone + Sized {
    fn zero() -> Self {
        todo!()
    }
    fn from_slices(_slices: [MatrixBc; A]) -> Self {
        todo!()
    }
    fn to_slices(&self) -> [MatrixBc; A] {
        todo!()
    }
    fn add(&self, _rhs: OneOf2<Self, Scalar>) -> Self {
        todo!()
    }
    fn sub(&self, _rhs: OneOf2<Self, Scalar>) -> Self {
        todo!()
    }
    fn mul(&self, _rhs: OneOf2<Self, Scalar>) -> Self {
        todo!()
    }
    fn div(&self, _rhs: OneOf2<Self, Scalar>) -> Self {
        todo!()
    }
    fn rem(&self, _rhs: OneOf2<Self, Scalar>) -> Self {
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
    fn get_dimensions(&self) -> (usize, usize, usize) {
        todo!()
    }
    fn get_element_count(&self) -> usize {
        todo!()
    }
}

pub trait TensorFieldOps<Scalar: ScalarContract, MatrixBc, VectorC, const A: usize, const B: usize, const C: usize>:
    TensorCoreOps<Scalar, MatrixBc, VectorC, A, B, C>
{
    /// Slice orthogonal to axis A, shape `(B, C)`.
    fn get_slice(&self, _index: usize) -> MatrixBc {
        todo!()
    }
    /// Slice orthogonal to axis A, shape `(B, C)`.
    fn set_slice(&mut self, _index: usize, _value: MatrixBc) {
        todo!()
    }
    /// Vector along axis C at fixed `(A=i, B=j)`.
    fn get_vector(&self, _i: usize, _j: usize) -> VectorC {
        todo!()
    }
    /// Vector along axis C at fixed `(A=i, B=j)`.
    fn set_vector(&mut self, _i: usize, _j: usize, _value: VectorC) {
        todo!()
    }
    fn get_lane(&self, _i: usize, _j: usize, _k: usize) -> Scalar {
        todo!()
    }
    fn set_lane(&mut self, _i: usize, _j: usize, _k: usize, _value: Scalar) {
        todo!()
    }
}

pub trait TensorProjectionCoreOps<
    Scalar: ScalarContract,
    MatrixAb,
    MatrixAc,
    MatrixBc,
    VectorA,
    VectorB,
    VectorC,
    const A: usize,
    const B: usize,
    const C: usize,
>: TensorFieldOps<Scalar, MatrixBc, VectorC, A, B, C>
{
    /// Slice orthogonal to axis C, shape `(A, B)`.
    fn get_matrix_ab(&self, _k: usize) -> MatrixAb {
        todo!()
    }
    fn set_matrix_ab(&mut self, _k: usize, _value: MatrixAb) {
        todo!()
    }

    /// Slice orthogonal to axis B, shape `(A, C)`.
    fn get_matrix_ac(&self, _j: usize) -> MatrixAc {
        todo!()
    }
    fn set_matrix_ac(&mut self, _j: usize, _value: MatrixAc) {
        todo!()
    }

    /// Slice orthogonal to axis A, shape `(B, C)`.
    fn get_matrix_bc(&self, _i: usize) -> MatrixBc {
        todo!()
    }
    fn set_matrix_bc(&mut self, _i: usize, _value: MatrixBc) {
        todo!()
    }

    /// Vector along axis A at fixed `(B=b, C=c)`.
    fn get_vector_a(&self, _b: usize, _c: usize) -> VectorA {
        todo!()
    }
    fn set_vector_a(&mut self, _b: usize, _c: usize, _value: VectorA) {
        todo!()
    }

    /// Vector along axis B at fixed `(A=a, C=c)`.
    fn get_vector_b(&self, _a: usize, _c: usize) -> VectorB {
        todo!()
    }
    fn set_vector_b(&mut self, _a: usize, _c: usize, _value: VectorB) {
        todo!()
    }

    /// Vector along axis C at fixed `(A=a, B=b)`.
    fn get_vector_c(&self, _a: usize, _b: usize) -> VectorC {
        todo!()
    }
    fn set_vector_c(&mut self, _a: usize, _b: usize, _value: VectorC) {
        todo!()
    }
}

pub trait TensorBridgeOps<Scalar: ScalarContract, MatrixBc, VectorC, const A: usize, const B: usize, const C: usize>:
    TensorCoreOps<Scalar, MatrixBc, VectorC, A, B, C>
{
}

pub trait TensorContract<Scalar: ScalarContract, MatrixBc, VectorC, const A: usize, const B: usize, const C: usize>:
    TensorCoreOps<Scalar, MatrixBc, VectorC, A, B, C> + TensorFieldOps<Scalar, MatrixBc, VectorC, A, B, C> + TensorBridgeOps<Scalar, MatrixBc, VectorC, A, B, C>
{
}
impl<T, Scalar: ScalarContract, MatrixBc, VectorC, const A: usize, const B: usize, const C: usize> TensorContract<Scalar, MatrixBc, VectorC, A, B, C> for T where
    T: TensorCoreOps<Scalar, MatrixBc, VectorC, A, B, C>
        + TensorFieldOps<Scalar, MatrixBc, VectorC, A, B, C>
        + TensorBridgeOps<Scalar, MatrixBc, VectorC, A, B, C>
{
}
