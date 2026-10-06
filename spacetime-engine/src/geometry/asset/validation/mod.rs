//! Authored-map validation: map symbols, object rules and scalar constraints.
//!
//! ## Module map
//!
//! - `constraints`: Scalar, polygon and generated-count constraints shared by object rules.
//! - `definitions`: Validation rules owned by each authored geometry definition.
//! - `map`: Map-wide symbols, references and generated-object budget.
//! - `objects`: Object identity and per-kind authored geometry constraints.
//!
//! This module groups the children; follow each child for its concrete implementation.
//!

mod constraints;
mod definitions;
mod map;
mod objects;
