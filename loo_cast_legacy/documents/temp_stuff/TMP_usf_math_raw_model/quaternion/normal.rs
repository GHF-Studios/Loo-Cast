#![allow(dead_code)]

use super::super::field::Field;
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

impl super::shared::QuaternionCoreOps for NormalQuaternion {}

impl super::shared::QuaternionFieldOps for NormalQuaternion {}

impl super::shared::QuaternionBridgeOps for NormalQuaternion {}
