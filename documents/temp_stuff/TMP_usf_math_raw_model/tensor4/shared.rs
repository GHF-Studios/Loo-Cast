#![allow(dead_code)]

use super::super::scalar::shared::ScalarContract;
use crate::utils::one_of::OneOf2;

pub trait Tensor4CoreOps<Scalar: ScalarContract, TensorBcd, MatrixCd, VectorD, const A: usize, const B: usize, const C: usize, const D: usize>:
    Clone + Sized
{
    fn zero() -> Self {
        todo!()
    }
    fn from_chunks(_chunks: [TensorBcd; A]) -> Self {
        todo!()
    }
    fn to_chunks(&self) -> [TensorBcd; A] {
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
    fn get_dimensions(&self) -> (usize, usize, usize, usize) {
        todo!()
    }
    fn get_element_count(&self) -> usize {
        todo!()
    }
}

pub trait Tensor4FieldOps<Scalar: ScalarContract, TensorBcd, MatrixCd, VectorD, const A: usize, const B: usize, const C: usize, const D: usize>:
    Tensor4CoreOps<Scalar, TensorBcd, MatrixCd, VectorD, A, B, C, D>
{
    /// Hyperplane orthogonal to axis A, shape `(B, C, D)`.
    fn get_chunk(&self, _index: usize) -> TensorBcd {
        todo!()
    }
    /// Hyperplane orthogonal to axis A, shape `(B, C, D)`.
    fn set_chunk(&mut self, _index: usize, _value: TensorBcd) {
        todo!()
    }
    /// Matrix orthogonal to axes A and B, shape `(C, D)`.
    fn get_matrix(&self, _i: usize, _j: usize) -> MatrixCd {
        todo!()
    }
    /// Matrix orthogonal to axes A and B, shape `(C, D)`.
    fn set_matrix(&mut self, _i: usize, _j: usize, _value: MatrixCd) {
        todo!()
    }
    /// Vector along axis D at fixed `(A=i, B=j, C=k)`.
    fn get_vector(&self, _i: usize, _j: usize, _k: usize) -> VectorD {
        todo!()
    }
    /// Vector along axis D at fixed `(A=i, B=j, C=k)`.
    fn set_vector(&mut self, _i: usize, _j: usize, _k: usize, _value: VectorD) {
        todo!()
    }
    fn get_lane(&self, _i: usize, _j: usize, _k: usize, _l: usize) -> Scalar {
        todo!()
    }
    fn set_lane(&mut self, _i: usize, _j: usize, _k: usize, _l: usize, _value: Scalar) {
        todo!()
    }
}

pub trait Tensor4ProjectionCoreOps<
    Scalar: ScalarContract,
    TensorAbc,
    TensorAbd,
    TensorAcd,
    TensorBcd,
    MatrixAb,
    MatrixAc,
    MatrixAd,
    MatrixBc,
    MatrixBd,
    MatrixCd,
    VectorA,
    VectorB,
    VectorC,
    VectorD,
    const A: usize,
    const B: usize,
    const C: usize,
    const D: usize,
>: Tensor4FieldOps<Scalar, TensorBcd, MatrixCd, VectorD, A, B, C, D>
{
    fn get_tensor_bcd(&self, _a: usize) -> TensorBcd {
        todo!()
    }
    fn set_tensor_bcd(&mut self, _a: usize, _value: TensorBcd) {
        todo!()
    }

    fn get_tensor_acd(&self, _b: usize) -> TensorAcd {
        todo!()
    }
    fn set_tensor_acd(&mut self, _b: usize, _value: TensorAcd) {
        todo!()
    }

    fn get_tensor_abd(&self, _c: usize) -> TensorAbd {
        todo!()
    }
    fn set_tensor_abd(&mut self, _c: usize, _value: TensorAbd) {
        todo!()
    }

    fn get_tensor_abc(&self, _d: usize) -> TensorAbc {
        todo!()
    }
    fn set_tensor_abc(&mut self, _d: usize, _value: TensorAbc) {
        todo!()
    }

    fn get_matrix_cd(&self, _a: usize, _b: usize) -> MatrixCd {
        todo!()
    }
    fn set_matrix_cd(&mut self, _a: usize, _b: usize, _value: MatrixCd) {
        todo!()
    }

    fn get_matrix_bd(&self, _a: usize, _c: usize) -> MatrixBd {
        todo!()
    }
    fn set_matrix_bd(&mut self, _a: usize, _c: usize, _value: MatrixBd) {
        todo!()
    }

    fn get_matrix_bc(&self, _a: usize, _d: usize) -> MatrixBc {
        todo!()
    }
    fn set_matrix_bc(&mut self, _a: usize, _d: usize, _value: MatrixBc) {
        todo!()
    }

    fn get_matrix_ad(&self, _b: usize, _c: usize) -> MatrixAd {
        todo!()
    }
    fn set_matrix_ad(&mut self, _b: usize, _c: usize, _value: MatrixAd) {
        todo!()
    }

    fn get_matrix_ac(&self, _b: usize, _d: usize) -> MatrixAc {
        todo!()
    }
    fn set_matrix_ac(&mut self, _b: usize, _d: usize, _value: MatrixAc) {
        todo!()
    }

    fn get_matrix_ab(&self, _c: usize, _d: usize) -> MatrixAb {
        todo!()
    }
    fn set_matrix_ab(&mut self, _c: usize, _d: usize, _value: MatrixAb) {
        todo!()
    }

    fn get_vector_d(&self, _a: usize, _b: usize, _c: usize) -> VectorD {
        todo!()
    }
    fn set_vector_d(&mut self, _a: usize, _b: usize, _c: usize, _value: VectorD) {
        todo!()
    }

    fn get_vector_c(&self, _a: usize, _b: usize, _d: usize) -> VectorC {
        todo!()
    }
    fn set_vector_c(&mut self, _a: usize, _b: usize, _d: usize, _value: VectorC) {
        todo!()
    }

    fn get_vector_b(&self, _a: usize, _c: usize, _d: usize) -> VectorB {
        todo!()
    }
    fn set_vector_b(&mut self, _a: usize, _c: usize, _d: usize, _value: VectorB) {
        todo!()
    }

    fn get_vector_a(&self, _b: usize, _c: usize, _d: usize) -> VectorA {
        todo!()
    }
    fn set_vector_a(&mut self, _b: usize, _c: usize, _d: usize, _value: VectorA) {
        todo!()
    }
}

pub trait Tensor4BridgeOps<Scalar: ScalarContract, TensorBcd, MatrixCd, VectorD, const A: usize, const B: usize, const C: usize, const D: usize>:
    Tensor4CoreOps<Scalar, TensorBcd, MatrixCd, VectorD, A, B, C, D>
{
}

pub trait Tensor4Contract<Scalar: ScalarContract, TensorBcd, MatrixCd, VectorD, const A: usize, const B: usize, const C: usize, const D: usize>:
    Tensor4CoreOps<Scalar, TensorBcd, MatrixCd, VectorD, A, B, C, D>
    + Tensor4FieldOps<Scalar, TensorBcd, MatrixCd, VectorD, A, B, C, D>
    + Tensor4BridgeOps<Scalar, TensorBcd, MatrixCd, VectorD, A, B, C, D>
{
}
impl<T, Scalar: ScalarContract, TensorBcd, MatrixCd, VectorD, const A: usize, const B: usize, const C: usize, const D: usize>
    Tensor4Contract<Scalar, TensorBcd, MatrixCd, VectorD, A, B, C, D> for T
where
    T: Tensor4CoreOps<Scalar, TensorBcd, MatrixCd, VectorD, A, B, C, D>
        + Tensor4FieldOps<Scalar, TensorBcd, MatrixCd, VectorD, A, B, C, D>
        + Tensor4BridgeOps<Scalar, TensorBcd, MatrixCd, VectorD, A, B, C, D>,
{
}
