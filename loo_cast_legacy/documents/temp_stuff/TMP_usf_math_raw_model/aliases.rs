#![allow(dead_code)]

use crate::utils::one_of::OneOf2;

/// Repr union helper used across math contracts.
pub type UsfOrNormal<UsfT, NormalT> = OneOf2<UsfT, NormalT>;
