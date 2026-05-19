//! # Scope
//! - Provides math types and functionality in the form of horribly-unergonomic but highly-generalized backends.
//!
//! Currently supports the following mathematical objects:\
//! [`scalar`]\
//! [`vector`]\
//! [`matrix`]\
//! [`tensor`]\
//! [`tensor4`]\
//! [`quaternion`]\
//! [`transform`]
//!
//! # Architecture
//! Each submodule is represented by the following sub-submodules:\
//! - [`aliases`]: Provides type aliases.
//! - [`shared`]: Provides the respective `*Contract` trait, and the "sub traits" it composes.
//! - [`normal`]: Implements the respective `*Contract` trait generically, and monomorphizes it into `normal` type aliases.
//! - [`usf`]: Implements the respective `*Contract` trait generically, and monomorphizes it into `usf` type aliases.
//!
//! ## More on `shared`:
//! - Provides the `*Contract` trait(s) for the respective mathematical object.
//! - This trait is NOT to be implemented manually; it is an implementation detail.
//!
//! ## More on `normal`:
//! - Provides the `Normal*` type for the respective mathematical object.
//! - This type is generic, and is locally "monomorphized" into concrete type aliases.
//! - These type aliases and the respective shared `*Contract` trait(s) compose the `normal` facade-facing API for working with these math internals.
//!
//! ## More on `usf`:
//! - Provides the `Usf*` type for the respective mathematical object.
//! - This type is generic, and is locally "monomorphized" into concrete type aliases.
//! - These type aliases and the respective shared `*Contract` trait(s) are the `usf` backbone for working with this module.
//!
//! The (`normal` and `usf`) type aliases and the `*Contract` traits together make up the "public" API for facades to consume.
//!
//! # Use Cases and Examples
//!

pub mod aliases;
pub mod field;
pub mod matrix;
pub mod op_kind;
pub mod op_mode;
pub mod op_policy;
pub mod quaternion;
pub mod scalar;
pub mod tensor;
pub mod tensor4;
pub mod transform;
pub mod vector;

#[cfg(test)]
mod tests {
    use super::field::Field;
    use super::op_kind::VectorMulKind;
    use super::scalar::aliases::UsfOrNormalScalar;
    use super::scalar::usf::UsfScalar;
    use super::vector::aliases::{UsfOrNormalVector, VectorProductOperand};
    use super::vector::usf::UsfVector3d;
    use crate::math::scalar::shared::ScalarCoreOps;
    use base_mod_shared::utils::one_of::OneOf2;

    #[test]
    fn scalar_core_ops_test() {
        let a = <UsfScalar as ScalarCoreOps>::new(17.3_f64);
        let b = <UsfScalar as ScalarCoreOps>::new(3_i8);
        let b = UsfOrNormalScalar::A(b);
        let sum = a.add(b);
        let expected_sum = <UsfScalar as ScalarCoreOps>::new(20.3_f64);

        assert_eq!(sum, expected_sum);
    }
}
