use super::super::field::Field;
pub use super::aliases::{UsfOrNormalFractionalScalar, UsfOrNormalScalar};
use super::shared::{FloatType, IntegerType, ScalarType, SignedIntegerType, UnsignedIntegerType};

pub type UsfDigit = i8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfScalar {
    pub digits: Field<Vec<UsfDigit>>,
    pub radix_position: Field<i64>,
}

impl ScalarType for UsfScalar {}
impl IntegerType for UsfScalar {}
impl SignedIntegerType for UsfScalar {}
impl UnsignedIntegerType for UsfScalar {}
impl FloatType for UsfScalar {}

impl super::shared::ScalarCoreOps for UsfScalar {}
impl super::shared::ScalarFieldOps for UsfScalar {}
impl super::shared::ScalarBridgeOps for UsfScalar {}
impl super::shared::FractionalScalarContract for UsfScalar {}
