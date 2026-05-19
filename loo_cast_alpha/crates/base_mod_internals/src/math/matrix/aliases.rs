use super::super::aliases::UsfOrNormal;
use super::normal::NormalMatrix;
use super::usf::UsfMatrix;

pub type UsfOrNormalMatrix<const R: usize, const C: usize> = UsfOrNormal<UsfMatrix<R, C>, NormalMatrix<R, C>>;
