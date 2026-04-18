//! TEMP SKETCH ONLY - RAW MODEL SURFACE
//! - structs/enums/traits/type aliases only
//! - no impl blocks
//! - trait methods are declarations only
//! - no free functions

#![allow(dead_code)]

// ---------------------------------------------------------------------------
// Supported "normal" scalar numeric types (canonical list via aliases)
// ---------------------------------------------------------------------------

pub type NormalI8 = i8;
pub type NormalI16 = i16;
pub type NormalI32 = i32;
pub type NormalI64 = i64;
pub type NormalI128 = i128;
pub type NormalIsize = isize;

pub type NormalU8 = u8;
pub type NormalU16 = u16;
pub type NormalU32 = u32;
pub type NormalU64 = u64;
pub type NormalU128 = u128;
pub type NormalUsize = usize;

pub type NormalF32 = f32;
pub type NormalF64 = f64;

pub type SupportedNormalScalars = (
    NormalI8,
    NormalI16,
    NormalI32,
    NormalI64,
    NormalI128,
    NormalIsize,
    NormalU8,
    NormalU16,
    NormalU32,
    NormalU64,
    NormalU128,
    NormalUsize,
    NormalF32,
    NormalF64,
);

// Normal canonical math aliases (Bevy/glam-backed).
pub type NormalVec2f32 = bevy::math::Vec2;
pub type NormalVec3f32 = bevy::math::Vec3;
pub type NormalVec4f32 = bevy::math::Vec4;
pub type NormalVec2f64 = bevy::math::DVec2;
pub type NormalVec3f64 = bevy::math::DVec3;
pub type NormalVec4f64 = bevy::math::DVec4;

pub type NormalVec2i32 = bevy::math::IVec2;
pub type NormalVec3i32 = bevy::math::IVec3;
pub type NormalVec4i32 = bevy::math::IVec4;
pub type NormalVec2u32 = bevy::math::UVec2;
pub type NormalVec3u32 = bevy::math::UVec3;
pub type NormalVec4u32 = bevy::math::UVec4;

pub type NormalMat2f32 = bevy::math::Mat2;
pub type NormalMat3f32 = bevy::math::Mat3;
pub type NormalMat4f32 = bevy::math::Mat4;
pub type NormalMat2f64 = bevy::math::DMat2;
pub type NormalMat3f64 = bevy::math::DMat3;
pub type NormalMat4f64 = bevy::math::DMat4;

pub type NormalQuatf32 = bevy::math::Quat;
pub type NormalQuatf64 = bevy::math::DQuat;

pub type NormalTranslation3f32 = bevy::math::Vec3;
pub type NormalRotationf32 = bevy::math::Quat;
pub type NormalScalef32 = bevy::math::Vec3;
pub type NormalTransformf32 = bevy::prelude::Transform;

// ---------------------------------------------------------------------------
// Normal numeric marker contracts (no impls in this sketch)
// ---------------------------------------------------------------------------

pub trait NormalScalarType: Copy + 'static {}
pub trait NormalIntegerType: NormalScalarType {}
pub trait NormalSignedIntegerType: NormalIntegerType {}
pub trait NormalUnsignedIntegerType: NormalIntegerType {}
pub trait NormalFloatType: NormalScalarType {}

// ---------------------------------------------------------------------------
// Kind-specific normal value carriers (full all-width coverage)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct NormalScalar(Field<NormalScalarRepr, FieldReadWrite>);
#[derive(Clone, Debug, PartialEq)]
enum NormalScalarRepr {
    I8(NormalI8),
    I16(NormalI16),
    I32(NormalI32),
    I64(NormalI64),
    I128(NormalI128),
    Isize(NormalIsize),
    U8(NormalU8),
    U16(NormalU16),
    U32(NormalU32),
    U64(NormalU64),
    U128(NormalU128),
    Usize(NormalUsize),
    F32(NormalF32),
    F64(NormalF64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NormalVector<const D: usize>(Field<NormalVectorRepr<D>, FieldReadWrite>);
#[derive(Clone, Debug, PartialEq)]
enum NormalVectorRepr<const D: usize> {
    // CONTRACT: D >= 2. D == 1 is scalar-equivalent and forbidden by model contract.
    I8([NormalI8; D]),
    I16([NormalI16; D]),
    I32([NormalI32; D]),
    I64([NormalI64; D]),
    I128([NormalI128; D]),
    Isize([NormalIsize; D]),
    U8([NormalU8; D]),
    U16([NormalU16; D]),
    U32([NormalU32; D]),
    U64([NormalU64; D]),
    U128([NormalU128; D]),
    Usize([NormalUsize; D]),
    F32([NormalF32; D]),
    F64([NormalF64; D]),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NormalMatrix<const R: usize, const C: usize>(Field<NormalMatrixRepr<R, C>, FieldReadWrite>);
#[derive(Clone, Debug, PartialEq)]
enum NormalMatrixRepr<const R: usize, const C: usize> {
    // CONTRACT: R >= 2 and C >= 2. 1xN / Nx1 are vector-equivalent and forbidden.
    I8([[NormalI8; C]; R]),
    I16([[NormalI16; C]; R]),
    I32([[NormalI32; C]; R]),
    I64([[NormalI64; C]; R]),
    I128([[NormalI128; C]; R]),
    Isize([[NormalIsize; C]; R]),
    U8([[NormalU8; C]; R]),
    U16([[NormalU16; C]; R]),
    U32([[NormalU32; C]; R]),
    U64([[NormalU64; C]; R]),
    U128([[NormalU128; C]; R]),
    Usize([[NormalUsize; C]; R]),
    F32([[NormalF32; C]; R]),
    F64([[NormalF64; C]; R]),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NormalTensor<const A: usize, const B: usize, const C: usize>(
    Field<NormalTensorRepr<A, B, C>, FieldReadWrite>,
);
#[derive(Clone, Debug, PartialEq)]
enum NormalTensorRepr<const A: usize, const B: usize, const C: usize> {
    // CONTRACT: A,B,C >= 2. Any axis == 1 is reducible and forbidden.
    I8([[[NormalI8; C]; B]; A]),
    I16([[[NormalI16; C]; B]; A]),
    I32([[[NormalI32; C]; B]; A]),
    I64([[[NormalI64; C]; B]; A]),
    I128([[[NormalI128; C]; B]; A]),
    Isize([[[NormalIsize; C]; B]; A]),
    U8([[[NormalU8; C]; B]; A]),
    U16([[[NormalU16; C]; B]; A]),
    U32([[[NormalU32; C]; B]; A]),
    U64([[[NormalU64; C]; B]; A]),
    U128([[[NormalU128; C]; B]; A]),
    Usize([[[NormalUsize; C]; B]; A]),
    F32([[[NormalF32; C]; B]; A]),
    F64([[[NormalF64; C]; B]; A]),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NormalTensor4<const A: usize, const B: usize, const C: usize, const D: usize>(
    Field<NormalTensor4Repr<A, B, C, D>, FieldReadWrite>,
);
#[derive(Clone, Debug, PartialEq)]
enum NormalTensor4Repr<const A: usize, const B: usize, const C: usize, const D: usize> {
    // CONTRACT: A,B,C,D >= 2. Any axis == 1 is reducible and forbidden.
    I8([[[[NormalI8; D]; C]; B]; A]),
    I16([[[[NormalI16; D]; C]; B]; A]),
    I32([[[[NormalI32; D]; C]; B]; A]),
    I64([[[[NormalI64; D]; C]; B]; A]),
    I128([[[[NormalI128; D]; C]; B]; A]),
    Isize([[[[NormalIsize; D]; C]; B]; A]),
    U8([[[[NormalU8; D]; C]; B]; A]),
    U16([[[[NormalU16; D]; C]; B]; A]),
    U32([[[[NormalU32; D]; C]; B]; A]),
    U64([[[[NormalU64; D]; C]; B]; A]),
    U128([[[[NormalU128; D]; C]; B]; A]),
    Usize([[[[NormalUsize; D]; C]; B]; A]),
    F32([[[[NormalF32; D]; C]; B]; A]),
    F64([[[[NormalF64; D]; C]; B]; A]),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NormalQuaternion(Field<NormalQuaternionRepr, FieldReadWrite>);
#[derive(Clone, Debug, PartialEq)]
enum NormalQuaternionRepr {
    // Rotation-quaternion contract:
    // - unit quaternion for valid rotation state (norm == 1)
    // - components are typically in [-1, 1], not [0, 1]
    I8([NormalI8; 4]),
    I16([NormalI16; 4]),
    I32([NormalI32; 4]),
    I64([NormalI64; 4]),
    I128([NormalI128; 4]),
    Isize([NormalIsize; 4]),
    U8([NormalU8; 4]),
    U16([NormalU16; 4]),
    U32([NormalU32; 4]),
    U64([NormalU64; 4]),
    U128([NormalU128; 4]),
    Usize([NormalUsize; 4]),
    F32([NormalF32; 4]),
    F64([NormalF64; 4]),
}

// ---------------------------------------------------------------------------
// USF core numeric/tensor data model
// ---------------------------------------------------------------------------

pub type UsfDigit = i8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfScalar {
    digits: Field<Vec<UsfDigit>, FieldReadWrite>,
    radix_position: Field<i64, FieldReadWrite>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfVector<const D: usize> {
    // CONTRACT: D >= 2. D == 1 is scalar-equivalent and forbidden by model contract.
    lanes: Field<[UsfScalar; D], FieldReadWrite>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfMatrix<const R: usize, const C: usize> {
    // CONTRACT: R >= 2 and C >= 2. 1xN / Nx1 are vector-equivalent and forbidden.
    rows: Field<[UsfVector<C>; R], FieldReadWrite>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfTensor<const A: usize, const B: usize, const C: usize> {
    // CONTRACT: A,B,C >= 2. Any axis == 1 is reducible and forbidden.
    slices: Field<[UsfMatrix<B, C>; A], FieldReadWrite>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfTensor4<const A: usize, const B: usize, const C: usize, const D: usize> {
    // CONTRACT: A,B,C,D >= 2. Any axis == 1 is reducible and forbidden.
    chunks: Field<[UsfTensor<B, C, D>; A], FieldReadWrite>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfQuaternion {
    // High-precision quaternion representation for cross-scale/ultra-precision workflows.
    // Rotation usage still expects unit normalization semantics.
    x: Field<UsfScalar, FieldReadWrite>,
    y: Field<UsfScalar, FieldReadWrite>,
    z: Field<UsfScalar, FieldReadWrite>,
    w: Field<UsfScalar, FieldReadWrite>,
}

// ---------------------------------------------------------------------------
// Kind unification containers (Normal or Usf for each kind)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Scalar(Field<ScalarRepr, FieldReadWrite>);
#[derive(Clone, Debug, PartialEq)]
enum ScalarRepr {
    Normal(NormalScalar),
    Usf(UsfScalar),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vector<const D: usize>(Field<VectorRepr<D>, FieldReadWrite>);
#[derive(Clone, Debug, PartialEq)]
enum VectorRepr<const D: usize> {
    Normal(NormalVector<D>),
    Usf(UsfVector<D>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<const R: usize, const C: usize>(Field<MatrixRepr<R, C>, FieldReadWrite>);
#[derive(Clone, Debug, PartialEq)]
enum MatrixRepr<const R: usize, const C: usize> {
    Normal(NormalMatrix<R, C>),
    Usf(UsfMatrix<R, C>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tensor<const A: usize, const B: usize, const C: usize>(
    Field<TensorRepr<A, B, C>, FieldReadWrite>,
);
#[derive(Clone, Debug, PartialEq)]
enum TensorRepr<const A: usize, const B: usize, const C: usize> {
    Normal(NormalTensor<A, B, C>),
    Usf(UsfTensor<A, B, C>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tensor4<const A: usize, const B: usize, const C: usize, const D: usize>(
    Field<Tensor4Repr<A, B, C, D>, FieldReadWrite>,
);
#[derive(Clone, Debug, PartialEq)]
enum Tensor4Repr<const A: usize, const B: usize, const C: usize, const D: usize> {
    Normal(NormalTensor4<A, B, C, D>),
    Usf(UsfTensor4<A, B, C, D>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Quaternion(Field<QuaternionRepr, FieldReadWrite>);
#[derive(Clone, Debug, PartialEq)]
enum QuaternionRepr {
    Normal(NormalQuaternion),
    Usf(UsfQuaternion),
}

// ---------------------------------------------------------------------------
// Curated alias set for monomorphized Rhai/Rust ergonomics
// - no zero-sized axes
// - aliases below are bootstrap defaults, NOT global hard limits
// - vector dimensions: open-ended in core model; aliases below cover common dims
// - matrix/tensor dimensions: open-ended in core model; aliases below are curated
// - tensors: curated practical subset, excluding scalar-equivalent forms
// - this is only a bootstrap set; extend aliases as needed by actual usage
// ---------------------------------------------------------------------------

// Vectors (2d..4d)
pub type Vector2d = Vector<2>;
pub type Vector3d = Vector<3>;
pub type Vector4d = Vector<4>;
pub type NormalVector2d = NormalVector<2>;
pub type NormalVector3d = NormalVector<3>;
pub type NormalVector4d = NormalVector<4>;
pub type UsfVector2d = UsfVector<2>;
pub type UsfVector3d = UsfVector<3>;
pub type UsfVector4d = UsfVector<4>;

// Matrices (curated common + larger square; non-reducible only)
pub type Matrix2x2 = Matrix<2, 2>;
pub type Matrix2x3 = Matrix<2, 3>;
pub type Matrix2x4 = Matrix<2, 4>;
pub type Matrix3x2 = Matrix<3, 2>;
pub type Matrix3x3 = Matrix<3, 3>;
pub type Matrix3x4 = Matrix<3, 4>;
pub type Matrix4x2 = Matrix<4, 2>;
pub type Matrix4x3 = Matrix<4, 3>;
pub type Matrix4x4 = Matrix<4, 4>;
pub type Matrix5x5 = Matrix<5, 5>;
pub type Matrix6x6 = Matrix<6, 6>;
pub type Matrix7x7 = Matrix<7, 7>;
pub type Matrix8x8 = Matrix<8, 8>;

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

// Tensor rank-3 (curated, excludes 1x1x1)
pub type Tensor2x2x2 = Tensor<2, 2, 2>;
pub type Tensor2x2x3 = Tensor<2, 2, 3>;
pub type Tensor2x3x3 = Tensor<2, 3, 3>;
pub type Tensor2x3x4 = Tensor<2, 3, 4>;
pub type Tensor3x3x3 = Tensor<3, 3, 3>;
pub type Tensor3x3x4 = Tensor<3, 3, 4>;
pub type Tensor3x4x4 = Tensor<3, 4, 4>;
pub type Tensor4x4x4 = Tensor<4, 4, 4>;
pub type Tensor2x4x8 = Tensor<2, 4, 8>;
pub type Tensor8x4x2 = Tensor<8, 4, 2>;

pub type NormalTensor2x2x2 = NormalTensor<2, 2, 2>;
pub type NormalTensor2x2x3 = NormalTensor<2, 2, 3>;
pub type NormalTensor2x3x3 = NormalTensor<2, 3, 3>;
pub type NormalTensor2x3x4 = NormalTensor<2, 3, 4>;
pub type NormalTensor3x3x3 = NormalTensor<3, 3, 3>;
pub type NormalTensor3x3x4 = NormalTensor<3, 3, 4>;
pub type NormalTensor3x4x4 = NormalTensor<3, 4, 4>;
pub type NormalTensor4x4x4 = NormalTensor<4, 4, 4>;
pub type NormalTensor2x4x8 = NormalTensor<2, 4, 8>;
pub type NormalTensor8x4x2 = NormalTensor<8, 4, 2>;

pub type UsfTensor2x2x2 = UsfTensor<2, 2, 2>;
pub type UsfTensor2x2x3 = UsfTensor<2, 2, 3>;
pub type UsfTensor2x3x3 = UsfTensor<2, 3, 3>;
pub type UsfTensor2x3x4 = UsfTensor<2, 3, 4>;
pub type UsfTensor3x3x3 = UsfTensor<3, 3, 3>;
pub type UsfTensor3x3x4 = UsfTensor<3, 3, 4>;
pub type UsfTensor3x4x4 = UsfTensor<3, 4, 4>;
pub type UsfTensor4x4x4 = UsfTensor<4, 4, 4>;
pub type UsfTensor2x4x8 = UsfTensor<2, 4, 8>;
pub type UsfTensor8x4x2 = UsfTensor<8, 4, 2>;

// Tensor rank-4 (curated, excludes 1x1x1x1)
pub type Tensor2x2x2x2 = Tensor4<2, 2, 2, 2>;
pub type Tensor2x2x3x4 = Tensor4<2, 2, 3, 4>;
pub type Tensor2x3x3x4 = Tensor4<2, 3, 3, 4>;
pub type Tensor3x3x3x3 = Tensor4<3, 3, 3, 3>;
pub type Tensor4x4x4x4 = Tensor4<4, 4, 4, 4>;
pub type Tensor2x4x4x8 = Tensor4<2, 4, 4, 8>;
pub type Tensor8x4x4x2 = Tensor4<8, 4, 4, 2>;

pub type NormalTensor2x2x2x2 = NormalTensor4<2, 2, 2, 2>;
pub type NormalTensor2x2x3x4 = NormalTensor4<2, 2, 3, 4>;
pub type NormalTensor2x3x3x4 = NormalTensor4<2, 3, 3, 4>;
pub type NormalTensor3x3x3x3 = NormalTensor4<3, 3, 3, 3>;
pub type NormalTensor4x4x4x4 = NormalTensor4<4, 4, 4, 4>;
pub type NormalTensor2x4x4x8 = NormalTensor4<2, 4, 4, 8>;
pub type NormalTensor8x4x4x2 = NormalTensor4<8, 4, 4, 2>;

pub type UsfTensor2x2x2x2 = UsfTensor4<2, 2, 2, 2>;
pub type UsfTensor2x2x3x4 = UsfTensor4<2, 2, 3, 4>;
pub type UsfTensor2x3x3x4 = UsfTensor4<2, 3, 3, 4>;
pub type UsfTensor3x3x3x3 = UsfTensor4<3, 3, 3, 3>;
pub type UsfTensor4x4x4x4 = UsfTensor4<4, 4, 4, 4>;
pub type UsfTensor2x4x4x8 = UsfTensor4<2, 4, 4, 8>;
pub type UsfTensor8x4x4x2 = UsfTensor4<8, 4, 4, 2>;

// ---------------------------------------------------------------------------
// USF wrappers/newtypes (translation, rotation, scale, transform)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfTranslation<const D: usize>(Field<UsfVector<D>, FieldReadWrite>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfRotation(Field<UsfQuaternion, FieldReadWrite>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfScale {
    log_base: Field<UsfScalar, FieldReadWrite>,
    scale_index: Field<i16, FieldReadWrite>,
    fractional_log_offset: Field<UsfScalar, FieldReadWrite>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfTransform {
    translation: Field<UsfTranslation<3>, FieldReadWrite>,
    rotation: Field<UsfRotation, FieldReadWrite>,
    scale: Field<UsfScale, FieldReadWrite>,
}

// ---------------------------------------------------------------------------
// Minimal generic field wrapper (declarations-only sketch)
// ---------------------------------------------------------------------------

pub enum FieldReadOnly {}
pub enum FieldReadWrite {}

pub trait FieldMutabilityType {}
pub trait FieldReadableType: FieldMutabilityType {}
pub trait FieldWritableType: FieldReadableType {}

pub type ReadOnlyField<T> = Field<T, FieldReadOnly>;
pub type ReadWriteField<T> = Field<T, FieldReadWrite>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field<T, MType = FieldReadWrite> {
    value: T,
    mutability: core::marker::PhantomData<MType>,
}

pub trait FieldGetType<T, MType: FieldReadableType> {
    fn get(&self) -> T;
}

pub trait FieldSetType<T, MType: FieldWritableType>: FieldGetType<T, MType> {
    fn set(&mut self, value: T);
}

pub trait FieldType<T, MType: FieldReadableType>: FieldGetType<T, MType> {}

// ---------------------------------------------------------------------------
// Baseline generic operator trait templates (self-only contracts)
// ---------------------------------------------------------------------------

pub trait UnaryOpsType: Sized {
    fn neg(&self) -> Self;
    fn abs(&self) -> Self;
}

pub trait BinaryOpsType: Sized {
    type Output;
    fn add(&self, rhs: Self) -> Self::Output;
    fn sub(&self, rhs: Self) -> Self::Output;
    fn mul(&self, rhs: Self) -> Self::Output;
    fn div(&self, rhs: Self) -> Self::Output;
    fn rem(&self, rhs: Self) -> Self::Output;
    fn min(&self, rhs: Self) -> Self::Output;
    fn max(&self, rhs: Self) -> Self::Output;
}

pub trait ClampOpsType: Sized {
    type Output;
    fn clamp(&self, lo: Self, hi: Self) -> Self::Output;
}

pub trait InterpolateOpsType: Sized {
    type Output;
    fn lerp<TType: NormalFloatType>(&self, rhs: Self, t: TType) -> Self::Output;
    fn smoothstep<TType: NormalFloatType>(&self, rhs: Self, t: TType) -> Self::Output;
}

// ---------------------------------------------------------------------------
// Static-core trait contracts (kind-specialized; declarations only)
// Compatibility is encoded directly in the kind traits below.
// ---------------------------------------------------------------------------

pub trait UsfScalarType:
    Sized
    + UnaryOpsType
    + BinaryOpsType<Output = Self>
    + ClampOpsType<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn two() -> Self;
    fn ten() -> Self;
    fn neg_one() -> Self;
    fn pi() -> Self;
    fn tau() -> Self;
    fn e() -> Self;
    fn nan() -> Self;
    fn infinity() -> Self;
    fn neg_infinity() -> Self;
    fn parse_decimal(text: &str) -> Self;
    fn to_decimal_string(&self) -> String;
    fn to_scientific_string(&self) -> String;
    fn normalize(&self) -> Self;
    fn is_zero(&self) -> bool;
    fn is_one(&self) -> bool;
    fn is_nan(&self) -> bool;
    fn is_infinite(&self) -> bool;
    fn is_finite(&self) -> bool;
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
    fn signum(&self) -> Self;
    fn recip(&self) -> Self;
    fn square(&self) -> Self;
    fn cube(&self) -> Self;
    fn sqrt(&self) -> Self;
    fn cbrt(&self) -> Self;
    fn exp(&self) -> Self;
    fn exp2(&self) -> Self;
    fn exp10(&self) -> Self;
    fn ln(&self) -> Self;
    fn log2(&self) -> Self;
    fn log10(&self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn tan(&self) -> Self;
    fn asin(&self) -> Self;
    fn acos(&self) -> Self;
    fn atan(&self) -> Self;
    fn sinh(&self) -> Self;
    fn cosh(&self) -> Self;
    fn tanh(&self) -> Self;
    fn floor(&self) -> Self;
    fn ceil(&self) -> Self;
    fn round(&self) -> Self;
    fn trunc(&self) -> Self;
    fn fract(&self) -> Self;
    fn pow<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn atan2<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn hypot<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn mod_euclid<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn fma<BType: UsfScalarType, CType: UsfScalarType>(&self, b: BType, c: CType) -> Self;
    fn lerp_f32<RhsType: UsfScalarType>(&self, rhs: RhsType, t: NormalF32) -> Self;
    fn lerp_f64<RhsType: UsfScalarType>(&self, rhs: RhsType, t: NormalF64) -> Self;
    fn smoothstep<E0Type: UsfScalarType, E1Type: UsfScalarType>(
        &self,
        edge0: E0Type,
        edge1: E1Type,
    ) -> Self;
    fn cmp_eq<RhsType: UsfScalarType>(&self, rhs: RhsType) -> bool;
    fn cmp_ne<RhsType: UsfScalarType>(&self, rhs: RhsType) -> bool;
    fn cmp_lt<RhsType: UsfScalarType>(&self, rhs: RhsType) -> bool;
    fn cmp_le<RhsType: UsfScalarType>(&self, rhs: RhsType) -> bool;
    fn cmp_gt<RhsType: UsfScalarType>(&self, rhs: RhsType) -> bool;
    fn cmp_ge<RhsType: UsfScalarType>(&self, rhs: RhsType) -> bool;
    fn from_normal<T: NormalScalarType>(value: T) -> Self;
    fn to_normal<T: NormalScalarType>(&self) -> T;
    fn from_normal_scalar(value: NormalScalar) -> Self;
    fn to_normal_scalar(&self) -> NormalScalar;
    fn from_scalar(value: Scalar) -> Self;
    fn to_scalar(&self) -> Scalar;
    fn value<MType: FieldMutabilityType>(&self) -> Field<UsfScalar, MType>;
}

pub trait UsfVectorType<const D: usize>:
    Sized
    + UnaryOpsType
    + BinaryOpsType<Output = Self>
    + ClampOpsType<Output = Self>
    + InterpolateOpsType<Output = Self>
{
    // CONTRACT: D >= 2.
    fn zero() -> Self;
    fn one() -> Self;
    fn splat<ValueType: UsfScalarType>(value: ValueType) -> Self;
    fn from_lanes(lanes: [UsfScalar; D]) -> Self;
    fn to_lanes(&self) -> [UsfScalar; D];
    fn lane(&self, index: usize) -> Scalar;
    fn set_lane<ValueType: UsfScalarType>(&self, index: usize, lane: ValueType) -> Self;
    fn normalize(&self) -> Self;
    fn floor(&self) -> Self;
    fn ceil(&self) -> Self;
    fn round(&self) -> Self;
    fn fract(&self) -> Self;
    fn dot<RhsType: UsfVectorType<D>>(&self, rhs: RhsType) -> Scalar;
    fn distance<RhsType: UsfVectorType<D>>(&self, rhs: RhsType) -> Scalar;
    fn angle_between<RhsType: UsfVectorType<D>>(&self, rhs: RhsType) -> Scalar;
    fn project<RhsType: UsfVectorType<D>>(&self, onto: RhsType) -> Self;
    fn reject<RhsType: UsfVectorType<D>>(&self, onto: RhsType) -> Self;
    fn reflect<RhsType: UsfVectorType<D>>(&self, normal: RhsType) -> Self;
    fn mul_elem<RhsType: UsfVectorType<D>>(&self, rhs: RhsType) -> Self;
    fn div_elem<RhsType: UsfVectorType<D>>(&self, rhs: RhsType) -> Self;
    fn fma<BType: UsfVectorType<D>, CType: UsfVectorType<D>>(&self, b: BType, c: CType) -> Self;
    fn add_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn sub_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn mul_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn div_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn scale<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn lane_access<MType: FieldMutabilityType>(&self, index: usize) -> Field<UsfScalar, MType>;
}

pub trait UsfVector3Type: UsfVectorType<3> {
    fn cross<RhsType: UsfVectorType<3>>(&self, rhs: RhsType) -> Self;
}

pub trait UsfMatrixType<const R: usize, const C: usize>: Sized + BinaryOpsType<Output = Self> {
    // CONTRACT: R >= 2 and C >= 2.
    fn zero() -> Self;
    fn from_rows(rows: [UsfVector<C>; R]) -> Self;
    fn to_rows(&self) -> [UsfVector<C>; R];
    fn row(&self, index: usize) -> Vector<C>;
    fn col(&self, index: usize) -> Vector<R>;
    fn transpose(&self) -> UsfMatrix<C, R>;
    fn mul_elem<RhsType: UsfMatrixType<R, C>>(&self, rhs: RhsType) -> Self;
    fn div_elem<RhsType: UsfMatrixType<R, C>>(&self, rhs: RhsType) -> Self;
    fn add_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn sub_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn mul_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn div_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn mul_vec<RhsType: UsfVectorType<C>>(&self, rhs: RhsType) -> Vector<R>;
    fn lane_access<MType: FieldMutabilityType>(
        &self,
        row: usize,
        col: usize,
    ) -> Field<UsfScalar, MType>;
}

pub trait UsfSquareMatrixType<const D: usize>: UsfMatrixType<D, D> {
    fn identity() -> Self;
    fn determinant(&self) -> Scalar;
    fn inverse(&self) -> Self;
    fn trace(&self) -> Scalar;
    fn powi<T: NormalSignedIntegerType>(&self, exp: T) -> Self;
    fn mul_mat<RhsType: UsfMatrixType<D, D>>(&self, rhs: RhsType) -> Matrix<D, D>;
}

pub trait UsfTensorType<const A: usize, const B: usize, const C: usize>:
    Sized + BinaryOpsType<Output = Self>
{
    // CONTRACT: A,B,C >= 2.
    fn zero() -> Self;
    fn from_slices(slices: [UsfMatrix<B, C>; A]) -> Self;
    fn to_slices(&self) -> [UsfMatrix<B, C>; A];
    fn slice(&self, index: usize) -> Matrix<B, C>;
    fn set_slice<ValueType: UsfMatrixType<B, C>>(&self, index: usize, value: ValueType) -> Self;
    fn add_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn sub_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn mul_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn div_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn lane_access<MType: FieldMutabilityType>(
        &self,
        i: usize,
        j: usize,
        k: usize,
    ) -> Field<UsfScalar, MType>;
}

pub trait UsfTensor4Type<const A: usize, const B: usize, const C: usize, const D: usize>:
    Sized + BinaryOpsType<Output = Self>
{
    // CONTRACT: A,B,C,D >= 2.
    fn zero() -> Self;
    fn from_chunks(chunks: [UsfTensor<B, C, D>; A]) -> Self;
    fn to_chunks(&self) -> [UsfTensor<B, C, D>; A];
    fn chunk(&self, index: usize) -> Tensor<B, C, D>;
    fn set_chunk<ValueType: UsfTensorType<B, C, D>>(&self, index: usize, value: ValueType) -> Self;
    fn add_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn sub_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn mul_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn div_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn lane_access<MType: FieldMutabilityType>(
        &self,
        i: usize,
        j: usize,
        k: usize,
        l: usize,
    ) -> Field<UsfScalar, MType>;
}

pub trait UsfQuaternionType:
    Sized + BinaryOpsType<Output = Self> + InterpolateOpsType<Output = Self>
{
    fn identity() -> Self;
    fn from_xyzw<XType: UsfScalarType, YType: UsfScalarType, ZType: UsfScalarType, WType: UsfScalarType>(
        x: XType,
        y: YType,
        z: ZType,
        w: WType,
    ) -> Self;
    fn to_xyzw(&self) -> [Scalar; 4];
    fn normalize(&self) -> Self;
    fn conjugate(&self) -> Self;
    fn inverse(&self) -> Self;
    fn dot<RhsType: UsfQuaternionType>(&self, rhs: RhsType) -> Scalar;
    fn mul_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn div_scalar<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn rotate_vec3<RhsType: UsfVectorType<3>>(&self, rhs: RhsType) -> Vector<3>;
    fn from_axis_angle<AxisType: UsfVectorType<3>, AngleType: UsfScalarType>(
        axis: AxisType,
        angle_rad: AngleType,
    ) -> Self;
    fn to_axis_angle(&self) -> (Vector<3>, Scalar);
    fn from_euler_xyz<XType: UsfScalarType, YType: UsfScalarType, ZType: UsfScalarType>(
        x_rad: XType,
        y_rad: YType,
        z_rad: ZType,
    ) -> Self;
    fn to_euler_xyz(&self) -> [Scalar; 3];
    fn slerp_f32<RhsType: UsfQuaternionType>(&self, rhs: RhsType, t: NormalF32) -> Self;
    fn slerp_f64<RhsType: UsfQuaternionType>(&self, rhs: RhsType, t: NormalF64) -> Self;
    fn nlerp_f32<RhsType: UsfQuaternionType>(&self, rhs: RhsType, t: NormalF32) -> Self;
    fn nlerp_f64<RhsType: UsfQuaternionType>(&self, rhs: RhsType, t: NormalF64) -> Self;
    fn to_mat3(&self) -> Matrix<3, 3>;
    fn from_mat3<ValueType: UsfMatrixType<3, 3>>(value: ValueType) -> Self;
    fn x<MType: FieldMutabilityType>(&self) -> Field<UsfScalar, MType>;
    fn y<MType: FieldMutabilityType>(&self) -> Field<UsfScalar, MType>;
    fn z<MType: FieldMutabilityType>(&self) -> Field<UsfScalar, MType>;
    fn w<MType: FieldMutabilityType>(&self) -> Field<UsfScalar, MType>;
}

pub trait UsfTranslationType<const D: usize>: Sized {
    fn from_vec<ValueType: UsfVectorType<D>>(value: ValueType) -> Self;
    fn to_vec(&self) -> Vector<D>;
    fn add<RhsType: UsfVectorType<D>>(&self, rhs: RhsType) -> Self;
    fn sub<RhsType: UsfVectorType<D>>(&self, rhs: RhsType) -> Self;
    fn scale<RhsType: UsfScalarType>(&self, rhs: RhsType) -> Self;
    fn vec<MType: FieldMutabilityType>(&self) -> Field<UsfVector<D>, MType>;
}

pub trait UsfRotationType: Sized {
    fn from_quat<ValueType: UsfQuaternionType>(value: ValueType) -> Self;
    fn to_quat(&self) -> Quaternion;
    fn compose<RhsType: UsfQuaternionType>(&self, rhs: RhsType) -> Self;
    fn quat<MType: FieldMutabilityType>(&self) -> Field<UsfQuaternion, MType>;
}

pub trait UsfScaleType: Sized {
    fn make<LogBaseType: UsfScalarType, OffsetType: UsfScalarType>(
        log_base: LogBaseType,
        scale_index: i16,
        fractional_log_offset: OffsetType,
    ) -> Self;
    fn log_base(&self) -> Scalar;
    fn scale_index(&self) -> i16;
    fn fractional_log_offset(&self) -> Scalar;
    fn log_base_access<MType: FieldMutabilityType>(&self) -> Field<UsfScalar, MType>;
    fn fractional_log_offset_access<MType: FieldMutabilityType>(&self) -> Field<UsfScalar, MType>;
}

pub trait UsfTransformType: Sized {
    fn make<TType: UsfTranslationType<3>, RType: UsfRotationType, SType: UsfScaleType>(
        translation: TType,
        rotation: RType,
        scale: SType,
    ) -> Self;
    fn translation(&self) -> UsfTranslation<3>;
    fn rotation(&self) -> UsfRotation;
    fn scale(&self) -> UsfScale;
    fn with_translation<TType: UsfTranslationType<3>>(&self, translation: TType) -> Self;
    fn with_rotation<RType: UsfRotationType>(&self, rotation: RType) -> Self;
    fn with_scale<SType: UsfScaleType>(&self, scale: SType) -> Self;
    fn translation_access<MType: FieldMutabilityType>(&self) -> Field<UsfTranslation<3>, MType>;
    fn rotation_access<MType: FieldMutabilityType>(&self) -> Field<UsfRotation, MType>;
    fn scale_access<MType: FieldMutabilityType>(&self) -> Field<UsfScale, MType>;
}

// Dynamic model intentionally omitted in this static-only sketch.
