use super::super::aliases::UsfOrNormal;
use super::normal::NormalVector;
use super::usf::UsfVector;

pub type UsfOrNormalVector<const D: usize> = UsfOrNormal<UsfVector<D>, NormalVector<D>>;
