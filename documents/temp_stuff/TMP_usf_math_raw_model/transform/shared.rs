#![allow(dead_code)]

use super::super::quaternion::shared::QuaternionAnyContract;
use super::super::scalar::shared::ScalarContract;
use super::super::vector::shared::VectorContract;
use crate::utils::one_of::OneOf2;

pub trait TranslationCoreOps<Scalar: ScalarContract, Vector: VectorContract<Scalar, D>, const D: usize>: Clone + Sized {
    fn from_vector<VectorB: VectorContract<Scalar, D>>(_value: OneOf2<Vector, VectorB>) -> Self {
        todo!()
    }
    fn to_vector<VectorB: VectorContract<Scalar, D>>(&self) -> OneOf2<Vector, VectorB> {
        todo!()
    }
    fn add<VectorB: VectorContract<Scalar, D>>(&self, _rhs: OneOf2<Vector, VectorB>) -> Self {
        todo!()
    }
    fn sub<VectorB: VectorContract<Scalar, D>>(&self, _rhs: OneOf2<Vector, VectorB>) -> Self {
        todo!()
    }
    fn scale<ScalarB: ScalarContract>(&self, _rhs: OneOf2<Scalar, ScalarB>) -> Self {
        todo!()
    }
}

pub trait TranslationFieldOps<Scalar: ScalarContract, Vector: VectorContract<Scalar, D>, const D: usize>: TranslationCoreOps<Scalar, Vector, D> {
    fn get_vector(&self) -> Vector {
        todo!()
    }
    fn set_vector(&mut self, _value: Vector) {
        todo!()
    }
}

pub trait TranslationBridgeOps<Scalar: ScalarContract, Vector: VectorContract<Scalar, D>, const D: usize>: TranslationCoreOps<Scalar, Vector, D> {}

pub trait RotationCoreOps<Quaternion: QuaternionAnyContract>: Clone + Sized {
    fn from_quat<QuaternionB: QuaternionAnyContract>(_value: OneOf2<Quaternion, QuaternionB>) -> Self {
        todo!()
    }
    fn to_quat<QuaternionB: QuaternionAnyContract>(&self) -> OneOf2<Quaternion, QuaternionB> {
        todo!()
    }
    fn compose(&self, _rhs: Quaternion) -> Self {
        todo!()
    }
}

pub trait RotationFieldOps<Quaternion: QuaternionAnyContract>: RotationCoreOps<Quaternion> {
    fn get_quaternion(&self) -> Quaternion {
        todo!()
    }
    fn set_quaternion(&mut self, _value: Quaternion) {
        todo!()
    }
}

pub trait RotationBridgeOps<Quaternion: QuaternionAnyContract>: RotationCoreOps<Quaternion> {}

pub trait ScaleCoreOps<Scalar: ScalarContract>: Clone + Sized {
    fn make<ScalarB: ScalarContract>(_log_base: OneOf2<Scalar, ScalarB>, _scale_index: i16, _fractional_log_offset: OneOf2<Scalar, ScalarB>) -> Self {
        todo!()
    }
}

pub trait ScaleFieldOps<Scalar: ScalarContract>: ScaleCoreOps<Scalar> {
    fn get_log_base(&self) -> Scalar {
        todo!()
    }
    fn get_scale_index(&self) -> i16 {
        todo!()
    }
    fn get_fractional_log_offset(&self) -> Scalar {
        todo!()
    }
    fn set_log_base(&mut self, _value: Scalar) {
        todo!()
    }
    fn set_scale_index(&mut self, _value: i16) {
        todo!()
    }
    fn set_fractional_log_offset(&mut self, _value: Scalar) {
        todo!()
    }
}

pub trait ScaleBridgeOps<Scalar: ScalarContract>: ScaleCoreOps<Scalar> {}

pub trait TransformCoreOps<Translation: TranslationAnyContract, Rotation: RotationAnyContract, Scale: ScaleAnyContract>: Clone + Sized {
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
    fn get_translation(&self) -> Translation {
        todo!()
    }
    fn get_rotation(&self) -> Rotation {
        todo!()
    }
    fn get_scale(&self) -> Scale {
        todo!()
    }
    fn set_translation(&mut self, _translation: Translation) {
        todo!()
    }
    fn set_rotation(&mut self, _rotation: Rotation) {
        todo!()
    }
    fn set_scale(&mut self, _scale: Scale) {
        todo!()
    }
}

pub trait TransformBridgeOps<Translation: TranslationAnyContract, Rotation: RotationAnyContract, Scale: ScaleAnyContract>:
    TransformCoreOps<Translation, Rotation, Scale>
{
}

pub trait TranslationContract<Scalar: ScalarContract, Vector: VectorContract<Scalar, D>, const D: usize>:
    TranslationCoreOps<Scalar, Vector, D> + TranslationFieldOps<Scalar, Vector, D> + TranslationBridgeOps<Scalar, Vector, D>
{
}
impl<T, Scalar: ScalarContract, Vector: VectorContract<Scalar, D>, const D: usize> TranslationContract<Scalar, Vector, D> for T where
    T: TranslationCoreOps<Scalar, Vector, D> + TranslationFieldOps<Scalar, Vector, D> + TranslationBridgeOps<Scalar, Vector, D>
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

pub trait ScaleContract<Scalar: ScalarContract>: ScaleCoreOps<Scalar> + ScaleFieldOps<Scalar> + ScaleBridgeOps<Scalar> {}
impl<T, Scalar: ScalarContract> ScaleContract<Scalar> for T where T: ScaleCoreOps<Scalar> + ScaleFieldOps<Scalar> + ScaleBridgeOps<Scalar> {}

pub trait TransformContract<Translation: TranslationAnyContract, Rotation: RotationAnyContract, Scale: ScaleAnyContract>:
    TransformCoreOps<Translation, Rotation, Scale> + TransformFieldOps<Translation, Rotation, Scale> + TransformBridgeOps<Translation, Rotation, Scale>
{
}
impl<T, Translation: TranslationAnyContract, Rotation: RotationAnyContract, Scale: ScaleAnyContract> TransformContract<Translation, Rotation, Scale> for T where
    T: TransformCoreOps<Translation, Rotation, Scale> + TransformFieldOps<Translation, Rotation, Scale> + TransformBridgeOps<Translation, Rotation, Scale>
{
}

pub trait TranslationAnyContract: Clone + Sized {}
impl<T, Scalar: ScalarContract, Vector: VectorContract<Scalar, D>, const D: usize> TranslationAnyContract for T where T: TranslationContract<Scalar, Vector, D> {}

pub trait RotationAnyContract: Clone + Sized {}
impl<T, Quaternion: QuaternionAnyContract> RotationAnyContract for T where T: RotationContract<Quaternion> {}

pub trait ScaleAnyContract: Clone + Sized {}
impl<T, Scalar: ScalarContract> ScaleAnyContract for T where T: ScaleContract<Scalar> {}
