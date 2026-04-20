#![allow(dead_code)]

use super::super::aliases::OutputMode;
use super::super::quaternion::shared::QuaternionAnyContract;
use super::super::scalar::aliases::{UsfOrNormalDecimalScalar, UsfOrNormalScalar};
use super::super::vector::shared::VectorContract;
use crate::utils::one_of::OneOf2;

pub trait TranslationCoreOps<Vector: VectorContract<D>, const D: usize>: Clone + Sized {
    /// Builds translation from vector input.
    fn from_vector<VectorB: VectorContract<D>>(_value: OneOf2<Vector, VectorB>) -> Self {
        todo!()
    }
    /// Returns translation as vector in either domain.
    fn to_vector<VectorB: VectorContract<D>>(&self) -> OneOf2<Vector, VectorB> {
        todo!()
    }
    /// Adds translation delta.
    fn add<VectorB: VectorContract<D>>(&self, _rhs: OneOf2<Vector, VectorB>) -> Self {
        todo!()
    }
    /// Subtracts translation delta.
    fn sub<VectorB: VectorContract<D>>(&self, _rhs: OneOf2<Vector, VectorB>) -> Self {
        todo!()
    }
    /// Scales translation by scalar from either domain.
    fn scale(&self, _rhs: UsfOrNormalScalar) -> Self {
        todo!()
    }
}

pub trait TranslationFieldOps<Vector: VectorContract<D>, const D: usize>: TranslationCoreOps<Vector, D> {
    /// Returns wrapped vector.
    fn get_vector(&self) -> Vector {
        todo!()
    }
    /// Sets wrapped vector.
    fn set_vector(&mut self, _value: Vector) {
        todo!()
    }
}

pub trait TranslationBridgeOps<Vector: VectorContract<D>, const D: usize>: TranslationCoreOps<Vector, D> {}

pub trait RotationCoreOps<Quaternion: QuaternionAnyContract>: Clone + Sized {
    /// Builds rotation from quaternion.
    fn from_quat<QuaternionB: QuaternionAnyContract>(_value: OneOf2<Quaternion, QuaternionB>) -> Self {
        todo!()
    }
    /// Returns wrapped quaternion in either domain.
    fn to_quat<QuaternionB: QuaternionAnyContract>(&self) -> OneOf2<Quaternion, QuaternionB> {
        todo!()
    }
    /// Composes two rotations.
    fn compose(&self, _rhs: Quaternion) -> Self {
        todo!()
    }
}

pub trait RotationFieldOps<Quaternion: QuaternionAnyContract>: RotationCoreOps<Quaternion> {
    /// Returns wrapped quaternion.
    fn get_quaternion(&self) -> Quaternion {
        todo!()
    }
    /// Sets wrapped quaternion.
    fn set_quaternion(&mut self, _value: Quaternion) {
        todo!()
    }
}

pub trait RotationBridgeOps<Quaternion: QuaternionAnyContract>: RotationCoreOps<Quaternion> {}

pub trait ScaleCoreOps: Clone + Sized {
    /// Builds logarithmic scale descriptor.
    fn make(_log_base: UsfOrNormalDecimalScalar, _scale_index: i16, _fractional_log_offset: UsfOrNormalDecimalScalar) -> Self {
        todo!()
    }
}

pub trait ScaleFieldOps: ScaleCoreOps {
    /// Returns logarithmic base in requested output mode.
    fn get_log_base(&self, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }
    /// Returns integer scale index.
    fn get_scale_index(&self) -> i16 {
        todo!()
    }
    /// Returns fractional offset in requested output mode.
    fn get_fractional_log_offset(&self, _output_mode: OutputMode) -> UsfOrNormalDecimalScalar {
        todo!()
    }
    /// Sets logarithmic base.
    fn set_log_base(&mut self, _value: UsfOrNormalDecimalScalar) {
        todo!()
    }
    /// Sets integer scale index.
    fn set_scale_index(&mut self, _value: i16) {
        todo!()
    }
    /// Sets fractional offset.
    fn set_fractional_log_offset(&mut self, _value: UsfOrNormalDecimalScalar) {
        todo!()
    }
}

pub trait ScaleBridgeOps: ScaleCoreOps {}

pub trait TransformCoreOps<Translation: TranslationAnyContract, Rotation: RotationAnyContract, Scale: ScaleAnyContract>: Clone + Sized {
    /// Builds transform tuple `(translation, rotation, scale)`.
    fn make<TranslationB: TranslationAnyContract, RotationB: RotationAnyContract, ScaleB: ScaleAnyContract>(
        _translation: OneOf2<Translation, TranslationB>,
        _rotation: OneOf2<Rotation, RotationB>,
        _scale: OneOf2<Scale, ScaleB>,
    ) -> Self {
        todo!()
    }
}

pub trait TransformFieldOps<Translation: TranslationAnyContract, Rotation: RotationAnyContract, Scale: ScaleAnyContract>:
    TransformCoreOps<Translation, Rotation, Scale>
{
    /// Returns translation component in requested domain.
    fn get_translation<TranslationB: TranslationAnyContract>(&self) -> OneOf2<Translation, TranslationB> {
        todo!()
    }
    /// Returns rotation component in requested domain.
    fn get_rotation<RotationB: RotationAnyContract>(&self) -> OneOf2<Rotation, RotationB> {
        todo!()
    }
    /// Returns scale component in requested domain.
    fn get_scale<ScaleB: ScaleAnyContract>(&self) -> OneOf2<Scale, ScaleB> {
        todo!()
    }
    /// Sets translation component from either domain.
    fn set_translation<TranslationB: TranslationAnyContract>(&mut self, _translation: OneOf2<Translation, TranslationB>) {
        todo!()
    }
    /// Sets rotation component from either domain.
    fn set_rotation<RotationB: RotationAnyContract>(&mut self, _rotation: OneOf2<Rotation, RotationB>) {
        todo!()
    }
    /// Sets scale component from either domain.
    fn set_scale<ScaleB: ScaleAnyContract>(&mut self, _scale: OneOf2<Scale, ScaleB>) {
        todo!()
    }
}

pub trait TransformBridgeOps<Translation: TranslationAnyContract, Rotation: RotationAnyContract, Scale: ScaleAnyContract>:
    TransformCoreOps<Translation, Rotation, Scale>
{
}

pub trait TranslationContract<Vector: VectorContract<D>, const D: usize>:
    TranslationCoreOps<Vector, D> + TranslationFieldOps<Vector, D> + TranslationBridgeOps<Vector, D>
{
}
impl<T, Vector: VectorContract<D>, const D: usize> TranslationContract<Vector, D> for T where
    T: TranslationCoreOps<Vector, D> + TranslationFieldOps<Vector, D> + TranslationBridgeOps<Vector, D>
{
}

pub trait RotationContract<Quaternion: QuaternionAnyContract>:
    RotationCoreOps<Quaternion> + RotationFieldOps<Quaternion> + RotationBridgeOps<Quaternion>
{
}
impl<T, Quaternion: QuaternionAnyContract> RotationContract<Quaternion> for T where
    T: RotationCoreOps<Quaternion> + RotationFieldOps<Quaternion> + RotationBridgeOps<Quaternion>
{
}

pub trait ScaleContract: ScaleCoreOps + ScaleFieldOps + ScaleBridgeOps {}
impl<T> ScaleContract for T where T: ScaleCoreOps + ScaleFieldOps + ScaleBridgeOps {}

pub trait TransformContract<Translation: TranslationAnyContract, Rotation: RotationAnyContract, Scale: ScaleAnyContract>:
    TransformCoreOps<Translation, Rotation, Scale> + TransformFieldOps<Translation, Rotation, Scale> + TransformBridgeOps<Translation, Rotation, Scale>
{
}
impl<T, Translation: TranslationAnyContract, Rotation: RotationAnyContract, Scale: ScaleAnyContract> TransformContract<Translation, Rotation, Scale> for T where
    T: TransformCoreOps<Translation, Rotation, Scale> + TransformFieldOps<Translation, Rotation, Scale> + TransformBridgeOps<Translation, Rotation, Scale>
{
}

pub trait TranslationAnyContract: Clone + Sized {}
impl<T, Vector: VectorContract<D>, const D: usize> TranslationAnyContract for T where T: TranslationContract<Vector, D> {}

pub trait RotationAnyContract: Clone + Sized {}
impl<T, Quaternion: QuaternionAnyContract> RotationAnyContract for T where T: RotationContract<Quaternion> {}

pub trait ScaleAnyContract: Clone + Sized {}
impl<T> ScaleAnyContract for T where T: ScaleContract {}
