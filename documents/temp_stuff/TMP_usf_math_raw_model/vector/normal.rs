#![allow(dead_code)]

use super::super::field::Field;
use super::super::scalar::normal::NormalScalar;

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

pub type NormalVector2d = NormalVector<2>;
pub type NormalVector3d = NormalVector<3>;
pub type NormalVector4d = NormalVector<4>;

impl<const D: usize> super::shared::VectorCoreOps<D> for NormalVector<D> {}

impl super::shared::Vector2dFieldOps for NormalVector<2> {}
impl super::shared::Vector3dFieldOps for NormalVector<3> {}
impl super::shared::Vector4dFieldOps for NormalVector<4> {}

impl super::shared::Vector2dCoreOps for NormalVector<2> {}
impl super::shared::Vector3dCoreOps for NormalVector<3> {}
impl super::shared::Vector4dCoreOps for NormalVector<4> {}
impl<const D: usize> super::shared::VectorBridgeOps<D> for NormalVector<D> {}
impl super::shared::Vector4dBridgeOps for NormalVector<4> {}
