#![allow(dead_code)]

use super::super::aliases::OutputMode;
use super::super::scalar::shared::ScalarContract;
use crate::utils::one_of::OneOf2;

pub trait Tensor4CoreOps<Scalar: ScalarContract, TensorBcd, MatrixCd, VectorD, const A: usize, const B: usize, const C: usize, const D: usize>:
    Clone + Sized
{
    /// Returns additive identity tensor.
    fn zero() -> Self {
        todo!()
    }
    /// Builds tensor from axis-A chunks.
    fn from_chunks(_chunks: [TensorBcd; A]) -> Self {
        todo!()
    }
    /// Returns axis-A chunks in requested output mode.
    fn to_chunks(&self, _output_mode: OutputMode) -> [TensorBcd; A] {
        todo!()
    }
    /// Adds tensor or scalar operand.
    fn add(&self, _rhs: OneOf2<Self, Scalar>) -> Self {
        todo!()
    }
    /// Subtracts tensor or scalar operand.
    fn sub(&self, _rhs: OneOf2<Self, Scalar>) -> Self {
        todo!()
    }
    /// Multiplies tensor or scalar operand component-wise.
    fn component_mul(&self, _rhs: OneOf2<Self, Scalar>) -> Self {
        todo!()
    }
    /// Divides tensor or scalar operand component-wise.
    fn component_div(&self, _rhs: OneOf2<Self, Scalar>) -> Self {
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
    /// Clamps element-wise to `[lo, hi]`.
    fn clamp(&self, _lo: Self, _hi: Self) -> Self {
        todo!()
    }
    /// Returns `(A, B, C, D)` dimensions.
    fn get_dimensions(&self) -> (usize, usize, usize, usize) {
        todo!()
    }
    /// Returns total element count.
    fn get_element_count(&self) -> usize {
        todo!()
    }
}

pub trait Tensor4FieldOps<Scalar: ScalarContract, TensorBcd, MatrixCd, VectorD, const A: usize, const B: usize, const C: usize, const D: usize>:
    Tensor4CoreOps<Scalar, TensorBcd, MatrixCd, VectorD, A, B, C, D>
{
    /// Hyperplane orthogonal to axis A, shape `(B, C, D)`.
    fn get_chunk(&self, _index: usize, _output_mode: OutputMode) -> TensorBcd {
        todo!()
    }
    /// Hyperplane orthogonal to axis A, shape `(B, C, D)`.
    fn set_chunk(&mut self, _index: usize, _value: TensorBcd) {
        todo!()
    }
    /// Matrix orthogonal to axes A and B, shape `(C, D)`.
    fn get_matrix(&self, _i: usize, _j: usize, _output_mode: OutputMode) -> MatrixCd {
        todo!()
    }
    /// Matrix orthogonal to axes A and B, shape `(C, D)`.
    fn set_matrix(&mut self, _i: usize, _j: usize, _value: MatrixCd) {
        todo!()
    }
    /// Vector along axis D at fixed `(A=i, B=j, C=k)`.
    fn get_vector(&self, _i: usize, _j: usize, _k: usize, _output_mode: OutputMode) -> VectorD {
        todo!()
    }
    /// Vector along axis D at fixed `(A=i, B=j, C=k)`.
    fn set_vector(&mut self, _i: usize, _j: usize, _k: usize, _value: VectorD) {
        todo!()
    }
    /// Returns scalar tensor component `(i, j, k, l)`.
    fn get_component(&self, _i: usize, _j: usize, _k: usize, _l: usize, _output_mode: OutputMode) -> Scalar {
        todo!()
    }
    /// Sets scalar tensor component `(i, j, k, l)`.
    fn set_component(&mut self, _i: usize, _j: usize, _k: usize, _l: usize, _value: Scalar) {
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
    /// Returns tensor view orthogonal to axis A.
    fn get_tensor_bcd(&self, _a: usize, _output_mode: OutputMode) -> TensorBcd {
        todo!()
    }
    /// Sets tensor view orthogonal to axis A.
    fn set_tensor_bcd(&mut self, _a: usize, _value: TensorBcd) {
        todo!()
    }

    /// Returns tensor view orthogonal to axis B.
    fn get_tensor_acd(&self, _b: usize, _output_mode: OutputMode) -> TensorAcd {
        todo!()
    }
    /// Sets tensor view orthogonal to axis B.
    fn set_tensor_acd(&mut self, _b: usize, _value: TensorAcd) {
        todo!()
    }

    /// Returns tensor view orthogonal to axis C.
    fn get_tensor_abd(&self, _c: usize, _output_mode: OutputMode) -> TensorAbd {
        todo!()
    }
    /// Sets tensor view orthogonal to axis C.
    fn set_tensor_abd(&mut self, _c: usize, _value: TensorAbd) {
        todo!()
    }

    /// Returns tensor view orthogonal to axis D.
    fn get_tensor_abc(&self, _d: usize, _output_mode: OutputMode) -> TensorAbc {
        todo!()
    }
    /// Sets tensor view orthogonal to axis D.
    fn set_tensor_abc(&mut self, _d: usize, _value: TensorAbc) {
        todo!()
    }

    /// Returns `(C, D)` matrix at fixed `(A=a, B=b)`.
    fn get_matrix_cd(&self, _a: usize, _b: usize, _output_mode: OutputMode) -> MatrixCd {
        todo!()
    }
    /// Sets `(C, D)` matrix at fixed `(A=a, B=b)`.
    fn set_matrix_cd(&mut self, _a: usize, _b: usize, _value: MatrixCd) {
        todo!()
    }

    /// Returns `(B, D)` matrix at fixed `(A=a, C=c)`.
    fn get_matrix_bd(&self, _a: usize, _c: usize, _output_mode: OutputMode) -> MatrixBd {
        todo!()
    }
    /// Sets `(B, D)` matrix at fixed `(A=a, C=c)`.
    fn set_matrix_bd(&mut self, _a: usize, _c: usize, _value: MatrixBd) {
        todo!()
    }

    /// Returns `(B, C)` matrix at fixed `(A=a, D=d)`.
    fn get_matrix_bc(&self, _a: usize, _d: usize, _output_mode: OutputMode) -> MatrixBc {
        todo!()
    }
    /// Sets `(B, C)` matrix at fixed `(A=a, D=d)`.
    fn set_matrix_bc(&mut self, _a: usize, _d: usize, _value: MatrixBc) {
        todo!()
    }

    /// Returns `(A, D)` matrix at fixed `(B=b, C=c)`.
    fn get_matrix_ad(&self, _b: usize, _c: usize, _output_mode: OutputMode) -> MatrixAd {
        todo!()
    }
    /// Sets `(A, D)` matrix at fixed `(B=b, C=c)`.
    fn set_matrix_ad(&mut self, _b: usize, _c: usize, _value: MatrixAd) {
        todo!()
    }

    /// Returns `(A, C)` matrix at fixed `(B=b, D=d)`.
    fn get_matrix_ac(&self, _b: usize, _d: usize, _output_mode: OutputMode) -> MatrixAc {
        todo!()
    }
    /// Sets `(A, C)` matrix at fixed `(B=b, D=d)`.
    fn set_matrix_ac(&mut self, _b: usize, _d: usize, _value: MatrixAc) {
        todo!()
    }

    /// Returns `(A, B)` matrix at fixed `(C=c, D=d)`.
    fn get_matrix_ab(&self, _c: usize, _d: usize, _output_mode: OutputMode) -> MatrixAb {
        todo!()
    }
    /// Sets `(A, B)` matrix at fixed `(C=c, D=d)`.
    fn set_matrix_ab(&mut self, _c: usize, _d: usize, _value: MatrixAb) {
        todo!()
    }

    /// Returns vector along axis D.
    fn get_vector_d(&self, _a: usize, _b: usize, _c: usize, _output_mode: OutputMode) -> VectorD {
        todo!()
    }
    /// Sets vector along axis D.
    fn set_vector_d(&mut self, _a: usize, _b: usize, _c: usize, _value: VectorD) {
        todo!()
    }

    /// Returns vector along axis C.
    fn get_vector_c(&self, _a: usize, _b: usize, _d: usize, _output_mode: OutputMode) -> VectorC {
        todo!()
    }
    /// Sets vector along axis C.
    fn set_vector_c(&mut self, _a: usize, _b: usize, _d: usize, _value: VectorC) {
        todo!()
    }

    /// Returns vector along axis B.
    fn get_vector_b(&self, _a: usize, _c: usize, _d: usize, _output_mode: OutputMode) -> VectorB {
        todo!()
    }
    /// Sets vector along axis B.
    fn set_vector_b(&mut self, _a: usize, _c: usize, _d: usize, _value: VectorB) {
        todo!()
    }

    /// Returns vector along axis A.
    fn get_vector_a(&self, _b: usize, _c: usize, _d: usize, _output_mode: OutputMode) -> VectorA {
        todo!()
    }
    /// Sets vector along axis A.
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
