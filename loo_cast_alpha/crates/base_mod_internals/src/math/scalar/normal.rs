use super::super::field::Field;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NormalScalar(pub Field<NormalScalarRepr>);

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum NormalScalarRepr {
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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NormalFractionalScalar(pub Field<NormalFractionalScalarRepr>);

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum NormalFractionalScalarRepr {
    F32(f32),
    F64(f64),
}

impl super::shared::ScalarCoreOps for NormalScalar {
    fn from_decimal_u8_digits(_negative: bool, _int_digits: Vec<u8>, _frac_digits: Vec<u8>) -> Self {
        todo!()
    }

    fn to_decimal_u8_digits(&self) -> (bool, Vec<u8>, Vec<u8>) {
        todo!()
    }
}
impl super::shared::ScalarFieldOps for NormalScalar {}
impl super::shared::ScalarBridgeOps for NormalScalar {}

impl super::shared::ScalarCoreOps for NormalFractionalScalar {
    fn from_decimal_u8_digits(_negative: bool, _int_digits: Vec<u8>, _frac_digits: Vec<u8>) -> Self {
        todo!()
    }

    fn to_decimal_u8_digits(&self) -> (bool, Vec<u8>, Vec<u8>) {
        todo!()
    }
}
impl super::shared::ScalarFieldOps for NormalFractionalScalar {}
impl super::shared::ScalarBridgeOps for NormalFractionalScalar {}
impl super::shared::FractionalScalarContract for NormalFractionalScalar {}
