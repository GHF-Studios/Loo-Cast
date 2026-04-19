//! TEMP SKETCH ONLY - RAW MODEL SURFACE
//! - structs/enums/type aliases + inherent impl API stubs
//! - methods are stubbed with `todo!()`
//! - no free functions

#![allow(dead_code)]

// ---------------------------------------------------------------------------
// Pure math concept modules (inline, collapsible, stubs-first)
// ---------------------------------------------------------------------------

pub mod scalar {
    use super::core::Field;

    pub trait NormalScalarType: Copy + 'static {}
    pub trait NormalIntegerType: NormalScalarType {}
    pub trait NormalSignedIntegerType: NormalIntegerType {}
    pub trait NormalUnsignedIntegerType: NormalIntegerType {}
    pub trait NormalFloatType: NormalScalarType {}

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
    pub struct NormalDecimalScalar(pub(super) Field<NormalDecimalScalarRepr>);
    #[derive(Clone, Debug, PartialEq)]
    pub(super) enum NormalDecimalScalarRepr {
        F32(f32),
        F64(f64),
    }

    pub type UsfDigit = i8;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfScalar {
        pub(super) digits: Field<Vec<UsfDigit>>,
        pub(super) radix_position: Field<i64>,
    }

    impl UsfScalar {
        pub fn zero() -> Self {
            todo!()
        }
        pub fn one() -> Self {
            todo!()
        }
        pub fn two() -> Self {
            todo!()
        }
        pub fn ten() -> Self {
            todo!()
        }
        pub fn neg_one() -> Self {
            todo!()
        }
        pub fn pi() -> Self {
            todo!()
        }
        pub fn tau() -> Self {
            todo!()
        }
        pub fn e() -> Self {
            todo!()
        }
        pub fn nan() -> Self {
            todo!()
        }
        pub fn infinity() -> Self {
            todo!()
        }
        pub fn neg_infinity() -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `text` is not a valid finite decimal literal for `UsfScalar`.
        /// - Panics if the parsed range/precision cannot be represented by the internal digit model.
        pub fn parse_decimal(_text: &str) -> Self {
            todo!()
        }
        pub fn to_decimal_string(&self) -> String {
            todo!()
        }
        pub fn to_scientific_string(&self) -> String {
            todo!()
        }
        pub fn normalize(&self) -> Self {
            todo!()
        }
        pub fn is_zero(&self) -> bool {
            todo!()
        }
        pub fn is_one(&self) -> bool {
            todo!()
        }
        pub fn is_nan(&self) -> bool {
            todo!()
        }
        pub fn is_infinite(&self) -> bool {
            todo!()
        }
        pub fn is_finite(&self) -> bool {
            todo!()
        }
        pub fn is_positive(&self) -> bool {
            todo!()
        }
        pub fn is_negative(&self) -> bool {
            todo!()
        }
        pub fn signum(&self) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is zero.
        pub fn recip(&self) -> Self {
            todo!()
        }
        pub fn square(&self) -> Self {
            todo!()
        }
        pub fn cube(&self) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is negative and real-only sqrt semantics are enforced.
        pub fn sqrt(&self) -> Self {
            todo!()
        }
        pub fn cbrt(&self) -> Self {
            todo!()
        }
        pub fn exp(&self) -> Self {
            todo!()
        }
        pub fn exp2(&self) -> Self {
            todo!()
        }
        pub fn exp10(&self) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is non-positive.
        pub fn ln(&self) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is non-positive.
        pub fn log2(&self) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is non-positive.
        pub fn log10(&self) -> Self {
            todo!()
        }
        pub fn sin(&self) -> Self {
            todo!()
        }
        pub fn cos(&self) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is at a tangent singularity and strict singularity handling is used.
        pub fn tan(&self) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is outside `[-1, 1]` under real-only semantics.
        pub fn asin(&self) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is outside `[-1, 1]` under real-only semantics.
        pub fn acos(&self) -> Self {
            todo!()
        }
        pub fn atan(&self) -> Self {
            todo!()
        }
        pub fn sinh(&self) -> Self {
            todo!()
        }
        pub fn cosh(&self) -> Self {
            todo!()
        }
        pub fn tanh(&self) -> Self {
            todo!()
        }
        pub fn floor(&self) -> Self {
            todo!()
        }
        pub fn ceil(&self) -> Self {
            todo!()
        }
        pub fn round(&self) -> Self {
            todo!()
        }
        pub fn trunc(&self) -> Self {
            todo!()
        }
        pub fn fract(&self) -> Self {
            todo!()
        }
        pub fn neg(&self) -> Self {
            todo!()
        }
        pub fn abs(&self) -> Self {
            todo!()
        }
        pub fn add(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn sub(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn mul(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `rhs` is zero.
        pub fn div(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `rhs` is zero.
        pub fn rem(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn min(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn max(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `lo > hi`.
        pub fn clamp(&self, _lo: UsfScalar, _hi: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics for undefined exponent/base combinations under real-only semantics.
        pub fn pow(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn atan2(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn hypot(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `rhs` is zero.
        pub fn mod_euclid(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn fma(&self, _b: UsfScalar, _c: UsfScalar) -> Self {
            todo!()
        }
        pub fn lerp_normal(&self, _rhs: UsfScalar, _t: NormalDecimalScalar) -> Self {
            todo!()
        }
        pub fn lerp_usf(&self, _rhs: UsfScalar, _t: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if edge ordering is invalid (`edge0 > edge1`) under strict smoothstep semantics.
        pub fn smoothstep(&self, _edge0: UsfScalar, _edge1: UsfScalar) -> Self {
            todo!()
        }
        pub fn cmp_eq(&self, _rhs: UsfScalar) -> bool {
            todo!()
        }
        pub fn cmp_ne(&self, _rhs: UsfScalar) -> bool {
            todo!()
        }
        pub fn cmp_lt(&self, _rhs: UsfScalar) -> bool {
            todo!()
        }
        pub fn cmp_le(&self, _rhs: UsfScalar) -> bool {
            todo!()
        }
        pub fn cmp_gt(&self, _rhs: UsfScalar) -> bool {
            todo!()
        }
        pub fn cmp_ge(&self, _rhs: UsfScalar) -> bool {
            todo!()
        }
        pub fn from_normal<T: NormalScalarType>(_value: T) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if conversion to `T` would overflow, underflow, or lose required domain semantics.
        /// - Panics if `T` is unsupported by the concrete conversion backend.
        pub fn to_normal<T: NormalScalarType>(&self) -> T {
            todo!()
        }
        pub fn from_normal_scalar(_value: NormalScalar) -> Self {
            todo!()
        }
        pub fn to_normal_scalar(&self) -> NormalScalar {
            todo!()
        }
        pub fn get_value(&self) -> UsfScalar {
            todo!()
        }
        pub fn set_value(&mut self, _value: UsfScalar) {
            todo!()
        }
    }
}

pub mod vector {
    use super::core::Field;
    use super::scalar::{NormalDecimalScalar, UsfScalar};

    #[derive(Clone, Debug, PartialEq)]
    #[repr(transparent)]
    pub struct NormalVector<const D: usize>(pub(super) Field<NormalVectorRepr<D>>);
    #[derive(Clone, Debug, PartialEq)]
    pub(super) enum NormalVectorRepr<const D: usize> {
        Generic(NormalVectorGenericRepr<D>),
        Concrete(NormalVectorConcreteRepr),
    }
    #[derive(Clone, Debug, PartialEq)]
    pub(super) enum NormalVectorGenericRepr<const D: usize> {
        // CONTRACT: D >= 2. D == 1 is scalar-equivalent and forbidden by model contract.
        I8([i8; D]),
        I16([i16; D]),
        I32([i32; D]),
        I64([i64; D]),
        I128([i128; D]),
        Isize([isize; D]),
        U8([u8; D]),
        U16([u16; D]),
        U32([u32; D]),
        U64([u64; D]),
        U128([u128; D]),
        Usize([usize; D]),
        F32([f32; D]),
        F64([f64; D]),
    }
    #[derive(Clone, Debug, PartialEq)]
    pub(super) enum NormalVectorConcreteRepr {
        // CONTRACT: Concrete variant dimensionality must match the enclosing `NormalVector<D>`.
        // - Vec2/DVec2/IVec2/I64Vec2/UVec2/U64Vec2 => D = 2
        // - Vec3/Vec3A/DVec3/IVec3/I64Vec3/UVec3/U64Vec3 => D = 3
        // - Vec4/DVec4/IVec4/I64Vec4/UVec4/U64Vec4 => D = 4
        Vec2(bevy::math::Vec2),
        Vec3(bevy::math::Vec3),
        Vec3A(bevy::math::Vec3A),
        Vec4(bevy::math::Vec4),
        DVec2(bevy::math::DVec2),
        DVec3(bevy::math::DVec3),
        DVec4(bevy::math::DVec4),
        IVec2(bevy::math::IVec2),
        IVec3(bevy::math::IVec3),
        IVec4(bevy::math::IVec4),
        I64Vec2(bevy::math::I64Vec2),
        I64Vec3(bevy::math::I64Vec3),
        I64Vec4(bevy::math::I64Vec4),
        UVec2(bevy::math::UVec2),
        UVec3(bevy::math::UVec3),
        UVec4(bevy::math::UVec4),
        U64Vec2(bevy::math::U64Vec2),
        U64Vec3(bevy::math::U64Vec3),
        U64Vec4(bevy::math::U64Vec4),
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfVector<const D: usize> {
        // CONTRACT: D >= 2. D == 1 is scalar-equivalent and forbidden by model contract.
        pub(super) lanes: Field<[UsfScalar; D]>,
    }

    pub type NormalVector2d = NormalVector<2>;
    pub type NormalVector3d = NormalVector<3>;
    pub type NormalVector4d = NormalVector<4>;
    pub type UsfVector2d = UsfVector<2>;
    pub type UsfVector3d = UsfVector<3>;
    pub type UsfVector4d = UsfVector<4>;

    impl<const D: usize> UsfVector<D> {
        pub fn zero() -> Self {
            todo!()
        }
        pub fn one() -> Self {
            todo!()
        }
        pub fn splat(_value: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `D < 2` is rejected by runtime validation.
        pub fn from_lanes(_lanes: [UsfScalar; D]) -> Self {
            todo!()
        }
        pub fn to_lanes(&self) -> [UsfScalar; D] {
            todo!()
        }
        /// # Panics
        /// - Panics if the vector has zero length.
        pub fn normalize(&self) -> Self {
            todo!()
        }
        pub fn floor(&self) -> Self {
            todo!()
        }
        pub fn ceil(&self) -> Self {
            todo!()
        }
        pub fn round(&self) -> Self {
            todo!()
        }
        pub fn fract(&self) -> Self {
            todo!()
        }
        pub fn neg(&self) -> Self {
            todo!()
        }
        pub fn abs(&self) -> Self {
            todo!()
        }
        pub fn add(&self, _rhs: UsfVector<D>) -> Self {
            todo!()
        }
        pub fn sub(&self, _rhs: UsfVector<D>) -> Self {
            todo!()
        }
        pub fn mul(&self, _rhs: UsfVector<D>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any corresponding lane in `rhs` is zero.
        pub fn div(&self, _rhs: UsfVector<D>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any corresponding lane in `rhs` is zero.
        pub fn rem(&self, _rhs: UsfVector<D>) -> Self {
            todo!()
        }
        pub fn min(&self, _rhs: UsfVector<D>) -> Self {
            todo!()
        }
        pub fn max(&self, _rhs: UsfVector<D>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any lane has `lo > hi`.
        pub fn clamp(&self, _lo: UsfVector<D>, _hi: UsfVector<D>) -> Self {
            todo!()
        }
        pub fn lerp_normal(&self, _rhs: UsfVector<D>, _t: NormalDecimalScalar) -> Self {
            todo!()
        }
        pub fn lerp_usf(&self, _rhs: UsfVector<D>, _t: UsfScalar) -> Self {
            todo!()
        }
        pub fn smoothstep_normal(&self, _rhs: UsfVector<D>, _t: NormalDecimalScalar) -> Self {
            todo!()
        }
        pub fn smoothstep_usf(&self, _rhs: UsfVector<D>, _t: UsfScalar) -> Self {
            todo!()
        }
        pub fn dot_usf(&self, _rhs: UsfVector<D>) -> UsfScalar {
            todo!()
        }
        pub fn dot_normal(&self, _rhs: UsfVector<D>) -> NormalDecimalScalar {
            todo!()
        }
        pub fn distance_usf(&self, _rhs: UsfVector<D>) -> UsfScalar {
            todo!()
        }
        pub fn distance_normal(&self, _rhs: UsfVector<D>) -> NormalDecimalScalar {
            todo!()
        }
        /// # Panics
        /// - Panics if either vector has zero length.
        pub fn angle_between_usf(&self, _rhs: UsfVector<D>) -> UsfScalar {
            todo!()
        }
        /// # Panics
        /// - Panics if either vector has zero length.
        pub fn angle_between_normal(&self, _rhs: UsfVector<D>) -> NormalDecimalScalar {
            todo!()
        }
        /// # Panics
        /// - Panics if `onto` is the zero vector.
        pub fn project(&self, _onto: UsfVector<D>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `onto` is the zero vector.
        pub fn reject(&self, _onto: UsfVector<D>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `normal` is the zero vector.
        pub fn reflect(&self, _normal: UsfVector<D>) -> Self {
            todo!()
        }
        pub fn mul_elem(&self, _rhs: UsfVector<D>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any corresponding lane in `rhs` is zero.
        pub fn div_elem(&self, _rhs: UsfVector<D>) -> Self {
            todo!()
        }
        pub fn fma(&self, _b: UsfVector<D>, _c: UsfVector<D>) -> Self {
            todo!()
        }
        pub fn add_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn sub_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn mul_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `rhs` is zero.
        pub fn div_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn scale(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn get_dimension(&self) -> usize {
            todo!()
        }
        pub fn get_length_usf(&self) -> UsfScalar {
            todo!()
        }
        pub fn get_length_normal(&self) -> NormalDecimalScalar {
            todo!()
        }
        pub fn get_length_squared_usf(&self) -> UsfScalar {
            todo!()
        }
        pub fn get_length_squared_normal(&self) -> NormalDecimalScalar {
            todo!()
        }
        /// # Panics
        /// - Panics if `index` is out of bounds.
        pub fn get_lane(&self, _index: usize) -> UsfScalar {
            todo!()
        }
        /// # Panics
        /// - Panics if `index` is out of bounds.
        /// - Panics if the lane is immutable under runtime field mutability policy.
        pub fn set_lane(&mut self, _index: usize, _value: UsfScalar) {
            todo!()
        }
    }

    impl UsfVector<3> {
        pub fn cross(&self, _rhs: UsfVector<3>) -> Self {
            todo!()
        }
    }
}

pub mod matrix {
    use super::core::Field;
    use super::scalar::{NormalDecimalScalar, NormalSignedIntegerType, UsfScalar};
    use super::vector::UsfVector;

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

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfMatrix<const R: usize, const C: usize> {
        // CONTRACT: R >= 2 and C >= 2. 1xN / Nx1 are vector-equivalent and forbidden.
        pub(super) rows: Field<[UsfVector<C>; R]>,
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

    impl<const R: usize, const C: usize> UsfMatrix<R, C> {
        pub fn zero() -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if runtime validation rejects degenerate matrix shape constraints.
        pub fn from_rows(_rows: [UsfVector<C>; R]) -> Self {
            todo!()
        }
        pub fn to_rows(&self) -> [UsfVector<C>; R] {
            todo!()
        }
        pub fn transpose(&self) -> UsfMatrix<C, R> {
            todo!()
        }
        pub fn mul_elem(&self, _rhs: UsfMatrix<R, C>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any corresponding lane in `rhs` is zero.
        pub fn div_elem(&self, _rhs: UsfMatrix<R, C>) -> Self {
            todo!()
        }
        pub fn add_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn sub_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn mul_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `rhs` is zero.
        pub fn div_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn add(&self, _rhs: UsfMatrix<R, C>) -> Self {
            todo!()
        }
        pub fn sub(&self, _rhs: UsfMatrix<R, C>) -> Self {
            todo!()
        }
        pub fn mul(&self, _rhs: UsfMatrix<R, C>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any corresponding lane in `rhs` is zero.
        pub fn div(&self, _rhs: UsfMatrix<R, C>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any corresponding lane in `rhs` is zero.
        pub fn rem(&self, _rhs: UsfMatrix<R, C>) -> Self {
            todo!()
        }
        pub fn min(&self, _rhs: UsfMatrix<R, C>) -> Self {
            todo!()
        }
        pub fn max(&self, _rhs: UsfMatrix<R, C>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any lane has `lo > hi`.
        pub fn clamp(&self, _lo: UsfMatrix<R, C>, _hi: UsfMatrix<R, C>) -> Self {
            todo!()
        }
        pub fn mul_vec(&self, _rhs: UsfVector<C>) -> UsfVector<R> {
            todo!()
        }
        pub fn get_row_count(&self) -> usize {
            todo!()
        }
        pub fn get_col_count(&self) -> usize {
            todo!()
        }
        pub fn get_shape(&self) -> (usize, usize) {
            todo!()
        }
        pub fn get_element_count(&self) -> usize {
            todo!()
        }
        /// # Panics
        /// - Panics if `index` is out of bounds.
        pub fn get_row(&self, _index: usize) -> UsfVector<C> {
            todo!()
        }
        /// # Panics
        /// - Panics if `index` is out of bounds.
        pub fn get_col(&self, _index: usize) -> UsfVector<R> {
            todo!()
        }
        /// # Panics
        /// - Panics if `row` or `col` is out of bounds.
        pub fn get_lane(&self, _row: usize, _col: usize) -> UsfScalar {
            todo!()
        }
        /// # Panics
        /// - Panics if `row` or `col` is out of bounds.
        /// - Panics if the target lane is immutable under runtime field mutability policy.
        pub fn set_lane(&mut self, _row: usize, _col: usize, _value: UsfScalar) {
            todo!()
        }
    }

    impl<const D: usize> UsfMatrix<D, D> {
        pub fn identity() -> Self {
            todo!()
        }
        pub fn determinant_usf(&self) -> UsfScalar {
            todo!()
        }
        pub fn determinant_normal(&self) -> NormalDecimalScalar {
            todo!()
        }
        /// # Panics
        /// - Panics if the matrix is singular or numerically non-invertible.
        pub fn inverse(&self) -> Self {
            todo!()
        }
        pub fn trace_usf(&self) -> UsfScalar {
            todo!()
        }
        pub fn trace_normal(&self) -> NormalDecimalScalar {
            todo!()
        }
        pub fn powi<T: NormalSignedIntegerType>(&self, _exp: T) -> Self {
            todo!()
        }
        pub fn mul_mat(&self, _rhs: UsfMatrix<D, D>) -> UsfMatrix<D, D> {
            todo!()
        }
    }
}

pub mod tensor {
    use super::core::Field;
    use super::matrix::UsfMatrix;
    use super::scalar::UsfScalar;
    use super::vector::UsfVector;

    #[derive(Clone, Debug, PartialEq)]
    #[repr(transparent)]
    pub struct NormalTensor<const A: usize, const B: usize, const C: usize>(pub(super) Field<NormalTensorRepr<A, B, C>>);
    #[derive(Clone, Debug, PartialEq)]
    pub(super) enum NormalTensorRepr<const A: usize, const B: usize, const C: usize> {
        // CONTRACT: A,B,C >= 2. Any axis == 1 is reducible and forbidden.
        I8([[[i8; C]; B]; A]),
        I16([[[i16; C]; B]; A]),
        I32([[[i32; C]; B]; A]),
        I64([[[i64; C]; B]; A]),
        I128([[[i128; C]; B]; A]),
        Isize([[[isize; C]; B]; A]),
        U8([[[u8; C]; B]; A]),
        U16([[[u16; C]; B]; A]),
        U32([[[u32; C]; B]; A]),
        U64([[[u64; C]; B]; A]),
        U128([[[u128; C]; B]; A]),
        Usize([[[usize; C]; B]; A]),
        F32([[[f32; C]; B]; A]),
        F64([[[f64; C]; B]; A]),
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfTensor<const A: usize, const B: usize, const C: usize> {
        // CONTRACT: A,B,C >= 2. Any axis == 1 is reducible and forbidden.
        pub(super) slices: Field<[UsfMatrix<B, C>; A]>,
    }

    pub type Tensor2x2x2 = UsfTensor<2, 2, 2>;
    pub type Tensor2x2x3 = UsfTensor<2, 2, 3>;
    pub type Tensor2x3x3 = UsfTensor<2, 3, 3>;
    pub type Tensor2x3x4 = UsfTensor<2, 3, 4>;
    pub type Tensor3x3x3 = UsfTensor<3, 3, 3>;
    pub type Tensor3x3x4 = UsfTensor<3, 3, 4>;
    pub type Tensor3x4x4 = UsfTensor<3, 4, 4>;
    pub type Tensor4x4x4 = UsfTensor<4, 4, 4>;
    pub type Tensor2x4x8 = UsfTensor<2, 4, 8>;
    pub type Tensor8x4x2 = UsfTensor<8, 4, 2>;

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

    impl<const A: usize, const B: usize, const C: usize> UsfTensor<A, B, C> {
        pub fn zero() -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if runtime validation rejects degenerate tensor shape constraints.
        pub fn from_slices(_slices: [UsfMatrix<B, C>; A]) -> Self {
            todo!()
        }
        pub fn to_slices(&self) -> [UsfMatrix<B, C>; A] {
            todo!()
        }
        pub fn add_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn sub_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn mul_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `rhs` is zero.
        pub fn div_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn add(&self, _rhs: UsfTensor<A, B, C>) -> Self {
            todo!()
        }
        pub fn sub(&self, _rhs: UsfTensor<A, B, C>) -> Self {
            todo!()
        }
        pub fn mul(&self, _rhs: UsfTensor<A, B, C>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any corresponding lane in `rhs` is zero.
        pub fn div(&self, _rhs: UsfTensor<A, B, C>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any corresponding lane in `rhs` is zero.
        pub fn rem(&self, _rhs: UsfTensor<A, B, C>) -> Self {
            todo!()
        }
        pub fn min(&self, _rhs: UsfTensor<A, B, C>) -> Self {
            todo!()
        }
        pub fn max(&self, _rhs: UsfTensor<A, B, C>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any lane has `lo > hi`.
        pub fn clamp(&self, _lo: UsfTensor<A, B, C>, _hi: UsfTensor<A, B, C>) -> Self {
            todo!()
        }
        pub fn get_dimensions(&self) -> (usize, usize, usize) {
            todo!()
        }
        pub fn get_element_count(&self) -> usize {
            todo!()
        }
        /// # Panics
        /// - Panics if `index` is out of bounds.
        pub fn get_slice(&self, _index: usize) -> UsfMatrix<B, C> {
            todo!()
        }
        /// # Panics
        /// - Panics if `index` is out of bounds.
        /// - Panics if the target slice is immutable under runtime field mutability policy.
        pub fn set_slice(&mut self, _index: usize, _value: UsfMatrix<B, C>) {
            todo!()
        }
        /// # Panics
        /// - Panics if `i` or `j` is out of bounds.
        pub fn get_vector(&self, _i: usize, _j: usize) -> UsfVector<C> {
            todo!()
        }
        /// # Panics
        /// - Panics if `i` or `j` is out of bounds.
        /// - Panics if the target vector is immutable under runtime field mutability policy.
        pub fn set_vector(&mut self, _i: usize, _j: usize, _value: UsfVector<C>) {
            todo!()
        }
        /// # Panics
        /// - Panics if any index is out of bounds.
        pub fn get_lane(&self, _i: usize, _j: usize, _k: usize) -> UsfScalar {
            todo!()
        }
        /// # Panics
        /// - Panics if any index is out of bounds.
        /// - Panics if the target lane is immutable under runtime field mutability policy.
        pub fn set_lane(&mut self, _i: usize, _j: usize, _k: usize, _value: UsfScalar) {
            todo!()
        }
    }
}

pub mod tensor4 {
    use super::core::Field;
    use super::matrix::UsfMatrix;
    use super::scalar::UsfScalar;
    use super::tensor::UsfTensor;
    use super::vector::UsfVector;

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

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfTensor4<const A: usize, const B: usize, const C: usize, const D: usize> {
        // CONTRACT: A,B,C,D >= 2. Any axis == 1 is reducible and forbidden.
        pub(super) chunks: Field<[UsfTensor<B, C, D>; A]>,
    }

    pub type Tensor2x2x2x2 = UsfTensor4<2, 2, 2, 2>;
    pub type Tensor2x2x3x4 = UsfTensor4<2, 2, 3, 4>;
    pub type Tensor2x3x3x4 = UsfTensor4<2, 3, 3, 4>;
    pub type Tensor3x3x3x3 = UsfTensor4<3, 3, 3, 3>;
    pub type Tensor4x4x4x4 = UsfTensor4<4, 4, 4, 4>;
    pub type Tensor2x4x4x8 = UsfTensor4<2, 4, 4, 8>;
    pub type Tensor8x4x4x2 = UsfTensor4<8, 4, 4, 2>;

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

    impl<const A: usize, const B: usize, const C: usize, const D: usize> UsfTensor4<A, B, C, D> {
        pub fn zero() -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if runtime validation rejects degenerate tensor shape constraints.
        pub fn from_chunks(_chunks: [UsfTensor<B, C, D>; A]) -> Self {
            todo!()
        }
        pub fn to_chunks(&self) -> [UsfTensor<B, C, D>; A] {
            todo!()
        }
        pub fn add_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn sub_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn mul_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `rhs` is zero.
        pub fn div_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn add(&self, _rhs: UsfTensor4<A, B, C, D>) -> Self {
            todo!()
        }
        pub fn sub(&self, _rhs: UsfTensor4<A, B, C, D>) -> Self {
            todo!()
        }
        pub fn mul(&self, _rhs: UsfTensor4<A, B, C, D>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any corresponding lane in `rhs` is zero.
        pub fn div(&self, _rhs: UsfTensor4<A, B, C, D>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any corresponding lane in `rhs` is zero.
        pub fn rem(&self, _rhs: UsfTensor4<A, B, C, D>) -> Self {
            todo!()
        }
        pub fn min(&self, _rhs: UsfTensor4<A, B, C, D>) -> Self {
            todo!()
        }
        pub fn max(&self, _rhs: UsfTensor4<A, B, C, D>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any lane has `lo > hi`.
        pub fn clamp(&self, _lo: UsfTensor4<A, B, C, D>, _hi: UsfTensor4<A, B, C, D>) -> Self {
            todo!()
        }
        pub fn get_dimensions(&self) -> (usize, usize, usize, usize) {
            todo!()
        }
        pub fn get_element_count(&self) -> usize {
            todo!()
        }
        /// # Panics
        /// - Panics if `index` is out of bounds.
        pub fn get_chunk(&self, _index: usize) -> UsfTensor<B, C, D> {
            todo!()
        }
        /// # Panics
        /// - Panics if `index` is out of bounds.
        /// - Panics if the target chunk is immutable under runtime field mutability policy.
        pub fn set_chunk(&mut self, _index: usize, _value: UsfTensor<B, C, D>) {
            todo!()
        }
        /// # Panics
        /// - Panics if `i` or `j` is out of bounds.
        pub fn get_matrix(&self, _i: usize, _j: usize) -> UsfMatrix<C, D> {
            todo!()
        }
        /// # Panics
        /// - Panics if `i` or `j` is out of bounds.
        /// - Panics if the target matrix is immutable under runtime field mutability policy.
        pub fn set_matrix(&mut self, _i: usize, _j: usize, _value: UsfMatrix<C, D>) {
            todo!()
        }
        /// # Panics
        /// - Panics if `i`, `j`, or `k` is out of bounds.
        pub fn get_vector(&self, _i: usize, _j: usize, _k: usize) -> UsfVector<D> {
            todo!()
        }
        /// # Panics
        /// - Panics if `i`, `j`, or `k` is out of bounds.
        /// - Panics if the target vector is immutable under runtime field mutability policy.
        pub fn set_vector(&mut self, _i: usize, _j: usize, _k: usize, _value: UsfVector<D>) {
            todo!()
        }
        /// # Panics
        /// - Panics if any index is out of bounds.
        pub fn get_lane(&self, _i: usize, _j: usize, _k: usize, _l: usize) -> UsfScalar {
            todo!()
        }
        /// # Panics
        /// - Panics if any index is out of bounds.
        /// - Panics if the target lane is immutable under runtime field mutability policy.
        pub fn set_lane(&mut self, _i: usize, _j: usize, _k: usize, _l: usize, _value: UsfScalar) {
            todo!()
        }
    }
}

pub mod quaternion {
    use super::core::Field;
    use super::matrix::{NormalMatrix, UsfMatrix};
    use super::scalar::{NormalDecimalScalar, UsfScalar};
    use super::vector::{NormalVector, UsfVector};

    #[derive(Clone, Debug, PartialEq)]
    #[repr(transparent)]
    pub struct NormalQuaternion(pub(super) Field<NormalQuaternionRepr>);
    #[derive(Clone, Debug, PartialEq)]
    pub(super) enum NormalQuaternionRepr {
        Generic(NormalQuaternionGenericRepr),
        Concrete(NormalQuaternionConcreteRepr),
    }
    #[derive(Clone, Debug, PartialEq)]
    pub(super) enum NormalQuaternionGenericRepr {
        // Rotation-quaternion contract:
        // - unit quaternion for valid rotation state (norm == 1)
        // - components are typically in [-1, 1], not [0, 1]
        I8([i8; 4]),
        I16([i16; 4]),
        I32([i32; 4]),
        I64([i64; 4]),
        I128([i128; 4]),
        Isize([isize; 4]),
        U8([u8; 4]),
        U16([u16; 4]),
        U32([u32; 4]),
        U64([u64; 4]),
        U128([u128; 4]),
        Usize([usize; 4]),
        F32([f32; 4]),
        F64([f64; 4]),
    }
    #[derive(Clone, Debug, PartialEq)]
    pub(super) enum NormalQuaternionConcreteRepr {
        // Bevy/glam concrete quaternions for normal-space runtime interop.
        F32(bevy::math::Quat),
        F64(bevy::math::DQuat),
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfQuaternion {
        // High-precision quaternion representation for cross-scale/ultra-precision workflows.
        // Rotation usage still expects unit normalization semantics.
        pub(super) x: Field<UsfScalar>,
        pub(super) y: Field<UsfScalar>,
        pub(super) z: Field<UsfScalar>,
        pub(super) w: Field<UsfScalar>,
    }

    impl UsfQuaternion {
        pub fn identity() -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if the input quaternion cannot be normalized into a valid rotation state.
        pub fn from_xyzw_usf(_x: UsfScalar, _y: UsfScalar, _z: UsfScalar, _w: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if the input quaternion cannot be normalized into a valid rotation state.
        pub fn from_xyzw_normal(_x: NormalDecimalScalar, _y: NormalDecimalScalar, _z: NormalDecimalScalar, _w: NormalDecimalScalar) -> Self {
            todo!()
        }
        pub fn to_xyzw_usf(&self) -> [UsfScalar; 4] {
            todo!()
        }
        pub fn to_xyzw_normal(&self) -> [NormalDecimalScalar; 4] {
            todo!()
        }
        /// # Panics
        /// - Panics if quaternion norm is zero.
        pub fn normalize(&self) -> Self {
            todo!()
        }
        pub fn conjugate(&self) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if quaternion norm is zero.
        pub fn inverse(&self) -> Self {
            todo!()
        }
        pub fn add(&self, _rhs: UsfQuaternion) -> Self {
            todo!()
        }
        pub fn sub(&self, _rhs: UsfQuaternion) -> Self {
            todo!()
        }
        pub fn mul(&self, _rhs: UsfQuaternion) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `rhs` represents a zero-norm divisor under quaternion division semantics.
        pub fn div(&self, _rhs: UsfQuaternion) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if remainder semantics are undefined for the operand pair.
        pub fn rem(&self, _rhs: UsfQuaternion) -> Self {
            todo!()
        }
        pub fn min(&self, _rhs: UsfQuaternion) -> Self {
            todo!()
        }
        pub fn max(&self, _rhs: UsfQuaternion) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any lane has `lo > hi`.
        pub fn clamp(&self, _lo: UsfQuaternion, _hi: UsfQuaternion) -> Self {
            todo!()
        }
        pub fn lerp_normal(&self, _rhs: UsfQuaternion, _t: NormalDecimalScalar) -> Self {
            todo!()
        }
        pub fn lerp_usf(&self, _rhs: UsfQuaternion, _t: UsfScalar) -> Self {
            todo!()
        }
        pub fn smoothstep_normal(&self, _rhs: UsfQuaternion, _t: NormalDecimalScalar) -> Self {
            todo!()
        }
        pub fn smoothstep_usf(&self, _rhs: UsfQuaternion, _t: UsfScalar) -> Self {
            todo!()
        }
        pub fn dot_usf(&self, _rhs: UsfQuaternion) -> UsfScalar {
            todo!()
        }
        pub fn dot_normal(&self, _rhs: UsfQuaternion) -> NormalDecimalScalar {
            todo!()
        }
        pub fn mul_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `rhs` is zero.
        pub fn div_scalar(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is not a valid normalized rotation quaternion.
        pub fn rotate_vec3(&self, _rhs: UsfVector<3>) -> UsfVector<3> {
            todo!()
        }
        /// # Panics
        /// - Panics if `axis` is zero-length.
        pub fn from_axis_angle_usf(_axis: UsfVector<3>, _angle_rad: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `axis` is zero-length.
        pub fn from_axis_angle_normal(_axis: NormalVector<3>, _angle_rad: NormalDecimalScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is not a valid normalized rotation quaternion.
        pub fn to_axis_angle_usf(&self) -> (UsfVector<3>, UsfScalar) {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is not a valid normalized rotation quaternion.
        pub fn to_axis_angle_normal(&self) -> (NormalVector<3>, NormalDecimalScalar) {
            todo!()
        }
        pub fn from_euler_xyz_usf(_x_rad: UsfScalar, _y_rad: UsfScalar, _z_rad: UsfScalar) -> Self {
            todo!()
        }
        pub fn from_euler_xyz_normal(_x_rad: NormalDecimalScalar, _y_rad: NormalDecimalScalar, _z_rad: NormalDecimalScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is not a valid normalized rotation quaternion.
        pub fn to_euler_xyz_usf(&self) -> [UsfScalar; 3] {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is not a valid normalized rotation quaternion.
        pub fn to_euler_xyz_normal(&self) -> [NormalDecimalScalar; 3] {
            todo!()
        }
        /// # Panics
        /// - Panics if interpolation endpoints are invalid rotation quaternions.
        /// - Panics if interpolation path is undefined for the endpoint pair.
        pub fn slerp_normal(&self, _rhs: UsfQuaternion, _t: NormalDecimalScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if interpolation endpoints are invalid rotation quaternions.
        /// - Panics if interpolation path is undefined for the endpoint pair.
        pub fn slerp_usf(&self, _rhs: UsfQuaternion, _t: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if interpolation endpoints are invalid rotation quaternions.
        /// - Panics if normalized interpolation produces a zero norm.
        pub fn nlerp_normal(&self, _rhs: UsfQuaternion, _t: NormalDecimalScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if interpolation endpoints are invalid rotation quaternions.
        /// - Panics if normalized interpolation produces a zero norm.
        pub fn nlerp_usf(&self, _rhs: UsfQuaternion, _t: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is not a valid normalized rotation quaternion.
        pub fn to_mat3_usf(&self) -> UsfMatrix<3, 3> {
            todo!()
        }
        /// # Panics
        /// - Panics if `self` is not a valid normalized rotation quaternion.
        pub fn to_mat3_normal(&self) -> NormalMatrix<3, 3> {
            todo!()
        }
        /// # Panics
        /// - Panics if `value` is not a valid rotation matrix under strict rotation-matrix validation.
        pub fn from_mat3_usf(_value: UsfMatrix<3, 3>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `value` is not a valid rotation matrix under strict rotation-matrix validation.
        pub fn from_mat3_normal(_value: NormalMatrix<3, 3>) -> Self {
            todo!()
        }
        pub fn get_x(&self) -> UsfScalar {
            todo!()
        }
        pub fn get_y(&self) -> UsfScalar {
            todo!()
        }
        pub fn get_z(&self) -> UsfScalar {
            todo!()
        }
        pub fn get_w(&self) -> UsfScalar {
            todo!()
        }
        pub fn set_x(&mut self, _value: UsfScalar) {
            todo!()
        }
        pub fn set_y(&mut self, _value: UsfScalar) {
            todo!()
        }
        pub fn set_z(&mut self, _value: UsfScalar) {
            todo!()
        }
        pub fn set_w(&mut self, _value: UsfScalar) {
            todo!()
        }
    }
}

pub mod ray {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalRay;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfRay;
    // TODO: define canonical representation + operation contract.
}

pub mod line {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalLine;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfLine;
    // TODO: define canonical representation + operation contract.
}

pub mod segment {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalSegment;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfSegment;
    // TODO: define canonical representation + operation contract.
}

pub mod plane {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalPlane;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfPlane;
    // TODO: define canonical representation + operation contract.
}

pub mod triangle {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalTriangle;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfTriangle;
    // TODO: define canonical representation + operation contract.
}

pub mod quad {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalQuad;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfQuad;
    // TODO: define canonical representation + operation contract.
}

pub mod rect {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalRect;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfRect;
    // TODO: define canonical representation + operation contract.
}

pub mod polygon {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalPolygon;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfPolygon;
    // TODO: define canonical representation + operation contract.
}

pub mod aabb {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalAabb;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfAabb;
    // TODO: define canonical representation + operation contract.
}

pub mod obb {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalObb;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfObb;
    // TODO: define canonical representation + operation contract.
}

pub mod sphere {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalSphere;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfSphere;
    // TODO: define canonical representation + operation contract.
}

pub mod capsule {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalCapsule;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfCapsule;
    // TODO: define canonical representation + operation contract.
}

pub mod frustum {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalFrustum;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfFrustum;
    // TODO: define canonical representation + operation contract.
}

pub mod bezier {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalBezier;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfBezier;
    // TODO: define canonical representation + operation contract.
}

pub mod spline {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalSpline;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfSpline;
    // TODO: define canonical representation + operation contract.
}

pub mod extents {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalExtents;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfExtents;
    // TODO: define canonical representation + operation contract.
}

pub mod transform {
    use super::core::Field;
    use super::quaternion::{NormalQuaternion, UsfQuaternion};
    use super::scalar::{NormalDecimalScalar, UsfScalar};
    use super::vector::{NormalVector, UsfVector};

    pub type NormalTranslation3f32 = bevy::math::Vec3;
    pub type NormalRotationf32 = bevy::math::Quat;
    pub type NormalScalef32 = bevy::math::Vec3;
    pub type NormalTransformf32 = bevy::prelude::Transform;

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct UsfTranslation<const D: usize>(pub(super) Field<UsfVector<D>>);

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct UsfRotation(pub(super) Field<UsfQuaternion>);

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfScale {
        pub(super) log_base: Field<UsfScalar>,
        pub(super) scale_index: Field<i16>,
        pub(super) fractional_log_offset: Field<UsfScalar>,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfTransform {
        pub(super) translation: Field<UsfTranslation<3>>,
        pub(super) rotation: Field<UsfRotation>,
        pub(super) scale: Field<UsfScale>,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfIsometry<const D: usize>;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfProjection<const D: usize>;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfFrame<const D: usize>;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfVelocity<const D: usize>;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfAngularVelocity;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfAcceleration<const D: usize>;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct UsfAngularAcceleration;

    impl<const D: usize> UsfTranslation<D> {
        /// # Panics
        /// - Panics if runtime validation rejects translation dimensionality constraints.
        pub fn from_vec_usf(_value: UsfVector<D>) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if runtime validation rejects translation dimensionality constraints.
        pub fn from_vec_normal(_value: NormalVector<D>) -> Self {
            todo!()
        }
        pub fn to_vec_usf(&self) -> UsfVector<D> {
            todo!()
        }
        pub fn to_vec_normal(&self) -> NormalVector<D> {
            todo!()
        }
        pub fn add(&self, _rhs: UsfVector<D>) -> Self {
            todo!()
        }
        pub fn sub(&self, _rhs: UsfVector<D>) -> Self {
            todo!()
        }
        pub fn scale(&self, _rhs: UsfScalar) -> Self {
            todo!()
        }
        pub fn get_vector(&self) -> UsfVector<D> {
            todo!()
        }
        pub fn set_vector(&mut self, _value: UsfVector<D>) {
            todo!()
        }
    }

    impl UsfRotation {
        /// # Panics
        /// - Panics if `value` is not a valid normalized rotation quaternion.
        pub fn from_quat_usf(_value: UsfQuaternion) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `value` is not a valid normalized rotation quaternion.
        pub fn from_quat_normal(_value: NormalQuaternion) -> Self {
            todo!()
        }
        pub fn to_quat_usf(&self) -> UsfQuaternion {
            todo!()
        }
        pub fn to_quat_normal(&self) -> NormalQuaternion {
            todo!()
        }
        /// # Panics
        /// - Panics if either operand is not a valid normalized rotation quaternion.
        pub fn compose(&self, _rhs: UsfQuaternion) -> Self {
            todo!()
        }
        pub fn get_quaternion(&self) -> UsfQuaternion {
            todo!()
        }
        pub fn set_quaternion(&mut self, _value: UsfQuaternion) {
            todo!()
        }
    }

    impl UsfScale {
        /// # Panics
        /// - Panics if `log_base <= 0` or `log_base == 1`.
        /// - Panics if any scalar component is non-finite under finite-only scale semantics.
        pub fn make_usf(_log_base: UsfScalar, _scale_index: i16, _fractional_log_offset: UsfScalar) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if `log_base <= 0` or `log_base == 1`.
        /// - Panics if any scalar component is non-finite under finite-only scale semantics.
        pub fn make_normal(_log_base: NormalDecimalScalar, _scale_index: i16, _fractional_log_offset: NormalDecimalScalar) -> Self {
            todo!()
        }
        pub fn get_log_base(&self) -> UsfScalar {
            todo!()
        }
        pub fn get_scale_index(&self) -> i16 {
            todo!()
        }
        pub fn get_fractional_log_offset(&self) -> UsfScalar {
            todo!()
        }
        /// # Panics
        /// - Panics if `value <= 0` or `value == 1`.
        /// - Panics if `value` is non-finite under finite-only scale semantics.
        pub fn set_log_base(&mut self, _value: UsfScalar) {
            todo!()
        }
        pub fn set_scale_index(&mut self, _value: i16) {
            todo!()
        }
        pub fn set_fractional_log_offset(&mut self, _value: UsfScalar) {
            todo!()
        }
    }

    impl UsfTransform {
        /// # Panics
        /// - Panics if any component violates transform invariants (invalid rotation or scale state).
        pub fn make_usf(_translation: UsfTranslation<3>, _rotation: UsfRotation, _scale: UsfScale) -> Self {
            todo!()
        }
        /// # Panics
        /// - Panics if any component violates transform invariants (invalid rotation or scale state).
        /// - Panics if normal inputs are non-finite under finite-only transform semantics.
        pub fn make_normal(_translation: NormalTranslation3f32, _rotation: NormalRotationf32, _scale: NormalScalef32) -> Self {
            todo!()
        }
        pub fn get_translation(&self) -> UsfTranslation<3> {
            todo!()
        }
        pub fn get_rotation(&self) -> UsfRotation {
            todo!()
        }
        pub fn get_scale(&self) -> UsfScale {
            todo!()
        }
        pub fn set_translation(&mut self, _translation: UsfTranslation<3>) {
            todo!()
        }
        pub fn set_rotation(&mut self, _rotation: UsfRotation) {
            todo!()
        }
        pub fn set_scale(&mut self, _scale: UsfScale) {
            todo!()
        }
    }
    // TODO: flesh out transform wrappers/contracts after pure-domain contracts stabilize.
}

// ---------------------------------------------------------------------------
// Core field wrapper and value-semantics substrate
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Minimal generic field wrapper (declarations-only sketch)
// ---------------------------------------------------------------------------
pub mod core {

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FieldMutability {
        Immutable,
        Mutable,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(i64)]
    pub enum FieldTryReadState {
        Ready = 0,
        Busy = 1,
    }

    impl FieldTryReadState {
        #[inline]
        pub fn ready() -> Self {
            Self::Ready
        }

        #[inline]
        pub fn busy() -> Self {
            Self::Busy
        }

        #[inline]
        pub fn is_ready(&self) -> bool {
            matches!(self, Self::Ready)
        }

        #[inline]
        pub fn is_busy(&self) -> bool {
            matches!(self, Self::Busy)
        }

        #[inline]
        pub fn code(&self) -> i64 {
            *self as i64
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(i64)]
    pub enum FieldTryWriteState {
        Ready = 0,
        Busy = 1,
    }

    impl FieldTryWriteState {
        #[inline]
        pub fn ready() -> Self {
            Self::Ready
        }

        #[inline]
        pub fn busy() -> Self {
            Self::Busy
        }

        #[inline]
        pub fn is_ready(&self) -> bool {
            matches!(self, Self::Ready)
        }

        #[inline]
        pub fn is_busy(&self) -> bool {
            matches!(self, Self::Busy)
        }

        #[inline]
        pub fn code(&self) -> i64 {
            *self as i64
        }
    }

    pub struct Field<T> {
        value: std::sync::RwLock<T>,
        // Runtime mutability declaration.
        // Default semantic is Immutable unless explicitly constructed as Mutable.
        mutability: FieldMutability,
    }

    struct FieldReadGuard<'a, T> {
        guard: std::sync::RwLockReadGuard<'a, T>,
    }

    struct FieldWriteGuard<'a, T> {
        guard: std::sync::RwLockWriteGuard<'a, T>,
    }

    impl<T: Clone> Clone for Field<T> {
        fn clone(&self) -> Self {
            let value = self.get();
            match self.mutability {
                FieldMutability::Immutable => Self::new(value),
                FieldMutability::Mutable => Self::new_mut(value),
            }
        }
    }

    impl<T: core::fmt::Debug> core::fmt::Debug for Field<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let guard = self.read();
            f.debug_struct("Field").field("value", &*guard).field("mutability", &self.mutability).finish()
        }
    }

    impl<T: PartialEq> PartialEq for Field<T> {
        fn eq(&self, other: &Self) -> bool {
            let lhs = self.read();
            let rhs = other.read();
            *lhs == *rhs && self.mutability == other.mutability
        }
    }

    impl<T: Eq> Eq for Field<T> {}

    impl<T> Field<T> {
        #[inline]
        pub fn new(value: T) -> Self {
            Self {
                value: std::sync::RwLock::new(value),
                mutability: FieldMutability::Immutable,
            }
        }

        #[inline]
        pub fn new_mut(value: T) -> Self {
            Self {
                value: std::sync::RwLock::new(value),
                mutability: FieldMutability::Mutable,
            }
        }

        #[inline]
        pub fn mutability(&self) -> FieldMutability {
            self.mutability
        }

        #[inline]
        pub fn is_mutable(&self) -> bool {
            matches!(self.mutability, FieldMutability::Mutable)
        }

        /// # Panics
        /// - Panics if the lock is currently held by a writer.
        /// - Panics if the lock is poisoned.
        #[inline]
        fn read(&self) -> FieldReadGuard<'_, T> {
            match self.try_read() {
                Some(guard) => guard,
                None => panic!("Field::read lock contention"),
            }
        }

        #[inline]
        fn try_read(&self) -> Option<FieldReadGuard<'_, T>> {
            match self.value.try_read() {
                Ok(guard) => Some(FieldReadGuard { guard }),
                Err(std::sync::TryLockError::WouldBlock) => None,
                Err(std::sync::TryLockError::Poisoned(_)) => panic!("Field::try_read lock poisoned"),
            }
        }

        /// # Panics
        /// - Panics if the lock is poisoned.
        #[inline]
        pub fn read_state(&self) -> FieldTryReadState {
            match self.value.try_read() {
                Ok(_guard) => FieldTryReadState::ready(),
                Err(std::sync::TryLockError::WouldBlock) => FieldTryReadState::busy(),
                Err(std::sync::TryLockError::Poisoned(_)) => panic!("Field::read_state lock poisoned"),
            }
        }

        /// # Panics
        /// - Panics if field mutability is `Immutable`.
        /// - Panics if the lock is currently held by another reader/writer.
        /// - Panics if the lock is poisoned.
        #[inline]
        fn write(&self) -> FieldWriteGuard<'_, T> {
            match self.try_write() {
                Some(guard) => guard,
                None => panic!("Field::write lock contention"),
            }
        }

        /// # Panics
        /// - Panics if field mutability is `Immutable`.
        #[inline]
        fn try_write(&self) -> Option<FieldWriteGuard<'_, T>> {
            if !self.is_mutable() {
                panic!("Field::try_write attempted write on immutable field");
            }

            match self.value.try_write() {
                Ok(guard) => Some(FieldWriteGuard { guard }),
                Err(std::sync::TryLockError::WouldBlock) => None,
                Err(std::sync::TryLockError::Poisoned(_)) => panic!("Field::try_write lock poisoned"),
            }
        }

        /// # Panics
        /// - Panics if the lock is poisoned.
        #[inline]
        pub fn write_state(&self) -> FieldTryWriteState {
            if !self.is_mutable() {
                panic!("Field::write_state called on immutable field");
            }

            match self.value.try_write() {
                Ok(_guard) => FieldTryWriteState::ready(),
                Err(std::sync::TryLockError::WouldBlock) => FieldTryWriteState::busy(),
                Err(std::sync::TryLockError::Poisoned(_)) => panic!("Field::write_state lock poisoned"),
            }
        }

        #[inline]
        pub fn set(&self, value: T) {
            *self.write() = value;
        }
    }

    impl<T: Clone> Field<T> {
        #[inline]
        pub fn get(&self) -> T {
            self.read().clone()
        }
    }

    impl<'a, T> core::ops::Deref for FieldReadGuard<'a, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.guard
        }
    }

    impl<'a, T> core::ops::Deref for FieldWriteGuard<'a, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.guard
        }
    }

    impl<'a, T> core::ops::DerefMut for FieldWriteGuard<'a, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.guard
        }
    }

    pub trait FieldGetType<T> {
        fn get(&self) -> T;
    }

    pub trait FieldSetType<T>: FieldGetType<T> {
        fn set(&self, value: T);
    }

    pub trait FieldType<T>: FieldSetType<T> {}

    impl<T: Clone> FieldGetType<T> for Field<T> {
        fn get(&self) -> T {
            Field::get(self)
        }
    }

    impl<T: Clone> FieldSetType<T> for Field<T> {
        fn set(&self, value: T) {
            Field::set(self, value);
        }
    }

    impl<T: Clone> FieldType<T> for Field<T> {}
}

pub use aabb::*;
pub use bezier::*;
pub use capsule::*;
pub use core::*;
pub use extents::*;
pub use frustum::*;
pub use line::*;
pub use matrix::*;
pub use obb::*;
pub use plane::*;
pub use polygon::*;
pub use quad::*;
pub use quaternion::*;
pub use ray::*;
pub use rect::*;
pub use scalar::*;
pub use segment::*;
pub use sphere::*;
pub use spline::*;
pub use tensor::*;
pub use tensor4::*;
pub use transform::*;
pub use triangle::*;
pub use vector::*;

// Dynamic model intentionally omitted in this static-only sketch.
