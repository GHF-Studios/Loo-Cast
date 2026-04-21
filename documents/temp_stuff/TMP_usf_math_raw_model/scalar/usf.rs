#![allow(dead_code)]

use super::super::field::Field;
pub use super::aliases::{UsfOrNormalDecimalScalar, UsfOrNormalScalar};
use super::normal::{NormalDecimalScalar, NormalScalar};
use super::shared::{FloatType, IntegerType, ScalarContract, ScalarType, SignedIntegerType, UnsignedIntegerType};
use crate::utils::one_of::OneOf2;

pub type UsfDigit = i8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsfScalar {
    pub(super) digits: Field<Vec<UsfDigit>>,
    pub(super) radix_position: Field<i64>,
}

impl ScalarType for UsfScalar {}
impl IntegerType for UsfScalar {}
impl SignedIntegerType for UsfScalar {}
impl UnsignedIntegerType for UsfScalar {}
impl FloatType for UsfScalar {}

impl super::shared::ScalarCoreOps for UsfScalar {}
impl super::shared::ScalarFieldOps for UsfScalar {}
impl super::shared::ScalarBridgeOps for UsfScalar {}
