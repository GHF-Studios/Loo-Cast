#![allow(dead_code)]

use crate::utils::one_of::OneOf2;

pub type UsfOrNormal<UsfT, NormalT> = OneOf2<UsfT, NormalT>;
