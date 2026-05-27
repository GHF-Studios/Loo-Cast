//! Range sample constant helpers.
//!
//! Canonical scalar range-sample constants are grouped by sign and magnitude
//! into submodules, then re-exported from the parent `math::scalar::constants`
//! module.

pub mod positive_billions;
pub mod positive_decillions;
pub mod positive_millions;
pub mod positive_nonillions;
pub mod positive_octillions;
pub mod positive_one_over_billions;
pub mod positive_one_over_decillions;
pub mod positive_one_over_millions;
pub mod positive_one_over_nonillions;
pub mod positive_one_over_octillions;
pub mod positive_one_over_peanuts;
pub mod positive_one_over_quadrillions;
pub mod positive_one_over_quintillions;
pub mod positive_one_over_septillions;
pub mod positive_one_over_sextillions;
pub mod positive_one_over_thousands;
pub mod positive_one_over_trillions;
pub mod positive_peanuts;
pub mod positive_quadrillions;
pub mod positive_quintillions;
pub mod positive_septillions;
pub mod positive_sextillions;
pub mod positive_thousands;
pub mod positive_trillions;

pub mod negative_billion;
pub mod negative_decillion;
pub mod negative_million;
pub mod negative_nonillion;
pub mod negative_octillion;
pub mod negative_one_over_billions;
pub mod negative_one_over_decillions;
pub mod negative_one_over_millions;
pub mod negative_one_over_nonillions;
pub mod negative_one_over_octillions;
pub mod negative_one_over_peanuts;
pub mod negative_one_over_quadrillions;
pub mod negative_one_over_quintillions;
pub mod negative_one_over_septillions;
pub mod negative_one_over_sextillions;
pub mod negative_one_over_thousands;
pub mod negative_one_over_trillions;
pub mod negative_peanuts;
pub mod negative_quadrillion;
pub mod negative_quintillion;
pub mod negative_septillion;
pub mod negative_sextillion;
pub mod negative_thousand;
pub mod negative_trillion;

// Declares `ScalarConstants` as the master trait over core + range-sample trait fragments.
base_mod_macros::declare_scalar_constants_trait!();
