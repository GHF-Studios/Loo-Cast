use super::super::field::Field;

#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct NormalTensor<const A: usize, const B: usize, const C: usize>(pub Field<NormalTensorRepr<A, B, C>>);

#[derive(Clone, Debug, PartialEq)]
pub enum NormalTensorRepr<const A: usize, const B: usize, const C: usize> {
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

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorCoreOps<A, B, C> for NormalTensor<A, B, C> {}

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorFieldOps<A, B, C> for NormalTensor<A, B, C> {}

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorBridgeOps<A, B, C> for NormalTensor<A, B, C> {}

impl<const A: usize, const B: usize, const C: usize> super::shared::TensorProjectionCoreOps<A, B, C> for NormalTensor<A, B, C> {}
