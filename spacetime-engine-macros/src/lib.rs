//! Procedural macro entrypoints for Spacetime Engine inspection and conflict registration.
//!
//! ## Module map
//!
//! - `conflict`: Expand component-conflict declarations into registration code.
//! - `inspect`: Expand inspection derives into typed metadata and visitors.
//! - `runtime_crate`: Runtime-crate resolution shared by generated macro output.
//!
//! This module groups the children; follow each child for its concrete implementation.
//!

mod conflict;
mod inspect;
mod runtime_crate;

use conflict::Conflict;
use inspect::Inspect;
use proc_macro::TokenStream;

// ECS

// - Components

#[proc_macro_attribute]
pub fn conflict(attr: TokenStream, item: TokenStream) -> TokenStream {
    match Conflict::parse(attr.into(), item.into()) {
        Ok(conflict) => conflict.generate().into(),
        Err(error) => error.into_compile_error().into(),
    }
}

#[proc_macro_derive(Inspect, attributes(inspect))]
pub fn derive_inspect(input: TokenStream) -> TokenStream {
    match Inspect::parse(input.into()) {
        Ok(inspect) => inspect.generate().into(),
        Err(error) => error.into_compile_error().into(),
    }
}
