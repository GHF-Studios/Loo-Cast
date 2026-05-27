//! `base_mod_macros`
//!
//! Procedural macros used by `base_mod_*` crates.
//!
//! Constants macros
//! - `declare_scalar_constants_trait!([TraitName])`: declares a master constants trait
//!   (default name: `ScalarConstants`) with blanket impl across all scalar
//!   constants trait fragments.
//! - `impl_scalar_constants_for!(Type)`: emits impls for every scalar constants trait
//!   fragment (core + range-sample) for a concrete type.

extern crate proc_macro;

mod constants;

use proc_macro::TokenStream;

/// Declares a master constants trait over all scalar constants traits.
///
/// # Input
/// - Empty: `declare_scalar_constants_trait!()` generates `pub trait ScalarConstants`.
/// - Trait name: `declare_scalar_constants_trait!(MyTrait)` generates `pub trait MyTrait`.
#[proc_macro]
pub fn declare_scalar_constants_trait(input: TokenStream) -> TokenStream {
    constants::declare_scalar_constants_trait(input)
}

/// Emits impls for every scalar constants fragment trait for a concrete type.
///
/// # Input
/// - Required: a target type, for example `impl_scalar_constants_for!(UsfScalar)`.
#[proc_macro]
pub fn impl_scalar_constants_for(input: TokenStream) -> TokenStream {
    constants::impl_scalar_constants_for(input)
}
