#![allow(dead_code)]

use crate::utils::one_of::OneOf2;

pub trait TranslationCoreOps<Scalar, Vector, const D: usize>: Clone + Sized {
    fn from_vector<VectorB>(_value: OneOf2<Vector, VectorB>) -> Self {
        todo!()
    }
    fn to_vector<VectorB>(&self) -> OneOf2<Vector, VectorB> {
        todo!()
    }
    fn add<VectorB>(&self, _rhs: OneOf2<Vector, VectorB>) -> Self {
        todo!()
    }
    fn sub<VectorB>(&self, _rhs: OneOf2<Vector, VectorB>) -> Self {
        todo!()
    }
    fn scale<ScalarB>(&self, _rhs: OneOf2<Scalar, ScalarB>) -> Self {
        todo!()
    }
}

pub trait TranslationFieldOps<Scalar, Vector, const D: usize>: TranslationCoreOps<Scalar, Vector, D> {
    fn get_vector(&self) -> Vector {
        todo!()
    }
    fn set_vector(&mut self, _value: Vector) {
        todo!()
    }
}

pub trait TranslationBridgeOps<Scalar, Vector, const D: usize>: TranslationCoreOps<Scalar, Vector, D> {}

pub trait RotationCoreOps<Quaternion>: Clone + Sized {
    fn from_quat<QuaternionB>(_value: OneOf2<Quaternion, QuaternionB>) -> Self {
        todo!()
    }
    fn to_quat<QuaternionB>(&self) -> OneOf2<Quaternion, QuaternionB> {
        todo!()
    }
    fn compose(&self, _rhs: Quaternion) -> Self {
        todo!()
    }
}

pub trait RotationFieldOps<Quaternion>: RotationCoreOps<Quaternion> {
    fn get_quaternion(&self) -> Quaternion {
        todo!()
    }
    fn set_quaternion(&mut self, _value: Quaternion) {
        todo!()
    }
}

pub trait RotationBridgeOps<Quaternion>: RotationCoreOps<Quaternion> {}

pub trait ScaleCoreOps<Scalar>: Clone + Sized {
    fn make<ScalarB>(_log_base: OneOf2<Scalar, ScalarB>, _scale_index: i16, _fractional_log_offset: OneOf2<Scalar, ScalarB>) -> Self {
        todo!()
    }
}

pub trait ScaleFieldOps<Scalar>: ScaleCoreOps<Scalar> {
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

pub trait ScaleBridgeOps<Scalar>: ScaleCoreOps<Scalar> {}

pub trait TransformCoreOps<Translation, Rotation, Scale>: Clone + Sized {
    fn make<TranslationB, RotationB, ScaleB>(
        _translation: OneOf2<Translation, TranslationB>,
        _rotation: OneOf2<Rotation, RotationB>,
        _scale: OneOf2<Scale, ScaleB>,
    ) -> Self {
        todo!()
    }
}

pub trait TransformFieldOps<Translation, Rotation, Scale>: TransformCoreOps<Translation, Rotation, Scale> {
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

pub trait TransformBridgeOps<Translation, Rotation, Scale>: TransformCoreOps<Translation, Rotation, Scale> {}

pub trait TranslationOps<Scalar, Vector, const D: usize>:
    TranslationCoreOps<Scalar, Vector, D> + TranslationFieldOps<Scalar, Vector, D> + TranslationBridgeOps<Scalar, Vector, D>
{
}
impl<T, Scalar, Vector, const D: usize> TranslationOps<Scalar, Vector, D> for T where
    T: TranslationCoreOps<Scalar, Vector, D> + TranslationFieldOps<Scalar, Vector, D> + TranslationBridgeOps<Scalar, Vector, D>
{
}

pub trait RotationOps<Quaternion>: RotationCoreOps<Quaternion> + RotationFieldOps<Quaternion> + RotationBridgeOps<Quaternion> {}
impl<T, Quaternion> RotationOps<Quaternion> for T where T: RotationCoreOps<Quaternion> + RotationFieldOps<Quaternion> + RotationBridgeOps<Quaternion> {}

pub trait ScaleOps<Scalar>: ScaleCoreOps<Scalar> + ScaleFieldOps<Scalar> + ScaleBridgeOps<Scalar> {}
impl<T, Scalar> ScaleOps<Scalar> for T where T: ScaleCoreOps<Scalar> + ScaleFieldOps<Scalar> + ScaleBridgeOps<Scalar> {}

pub trait TransformOps<Translation, Rotation, Scale>:
    TransformCoreOps<Translation, Rotation, Scale> + TransformFieldOps<Translation, Rotation, Scale> + TransformBridgeOps<Translation, Rotation, Scale>
{
}
impl<T, Translation, Rotation, Scale> TransformOps<Translation, Rotation, Scale> for T where
    T: TransformCoreOps<Translation, Rotation, Scale> + TransformFieldOps<Translation, Rotation, Scale> + TransformBridgeOps<Translation, Rotation, Scale>
{
}
