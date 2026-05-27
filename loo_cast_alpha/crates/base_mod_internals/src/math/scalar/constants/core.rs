#![cfg_attr(any(), rustfmt::skip)]

use crate::math::scalar::constants::ScalarCoreConst;

pub trait CoreConstants {
    const SCALAR_EPSILON: ScalarCoreConst = ([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, ], false, 79);
    const SCALAR_MAXIMUM: ScalarCoreConst = ([4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, ], [4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, ], false, 79);
    const SCALAR_MINIMUM: ScalarCoreConst = ([-5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, ], [-5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, ], true, 79);
    const SCALAR_PI: ScalarCoreConst = ([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, ], [1, 4, 2, -4, -1, 3, -3, -5, 4, -4, -1, 0, -2, -1, 3, 2, 4, -1, -5, -4, 3, -4, 4, 3, 4, -2, 3, 3, -2, 0, -5, 0, 3, -1, -2, 4, 2, 0, -3, 2, -3, -1, 4, 0, ], false, 78);
    const SCALAR_TAU: ScalarCoreConst = ([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -4, ], [3, -2, 3, 2, -1, -5, 3, 1, -3, 2, -2, 0, -4, -1, -3, -5, -2, -3, -1, 3, -5, 3, -1, -3, -2, -3, -3, -4, -4, -1, 0, 1, -4, -2, -3, -2, 4, -1, 4, 3, 4, -1, -2, 0, ], false, 78);
    const SCALAR_E: ScalarCoreConst = ([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, ], [-3, 2, -2, 3, -2, 2, -2, 3, -1, -5, -4, -1, 1, -5, -5, 2, 4, -5, 4, -4, 0, 3, -1, -2, -5, -3, 1, 4, -5, 3, -3, -4, 3, -5, 0, -2, -2, -4, -3, 3, -5, -3, 1, -1, ], false, 79);
}

pub struct CoreConstantsConstSet;
impl CoreConstants for CoreConstantsConstSet {}

pub const SCALAR_EPSILON: ScalarCoreConst = <CoreConstantsConstSet as CoreConstants>::SCALAR_EPSILON;
pub const SCALAR_MAXIMUM: ScalarCoreConst = <CoreConstantsConstSet as CoreConstants>::SCALAR_MAXIMUM;
pub const SCALAR_MINIMUM: ScalarCoreConst = <CoreConstantsConstSet as CoreConstants>::SCALAR_MINIMUM;
pub const SCALAR_PI: ScalarCoreConst = <CoreConstantsConstSet as CoreConstants>::SCALAR_PI;
pub const SCALAR_TAU: ScalarCoreConst = <CoreConstantsConstSet as CoreConstants>::SCALAR_TAU;
pub const SCALAR_E: ScalarCoreConst = <CoreConstantsConstSet as CoreConstants>::SCALAR_E;
