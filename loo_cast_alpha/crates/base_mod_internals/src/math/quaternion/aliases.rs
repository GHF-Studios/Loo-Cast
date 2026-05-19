use super::super::aliases::UsfOrNormal;
use super::normal::NormalQuaternion;
use super::usf::UsfQuaternion;

use super::super::matrix::{normal::NormalMatrix, usf::UsfMatrix};

pub type UsfOrNormalQuaternion = UsfOrNormal<UsfQuaternion, NormalQuaternion>;
pub type UsfOrNormalMat3 = UsfOrNormal<UsfMatrix<3, 3>, NormalMatrix<3, 3>>;
