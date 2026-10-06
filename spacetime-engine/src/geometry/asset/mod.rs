//! Authored geometry asset schema, validation and loading.
//!
//! ## Module map
//!
//! - `loader`: Bevy asset-loader adapter for `.spacemap` authored maps.
//! - `schema`: Deserializable schema for human-authored geometry maps.
//! - `validation`: Authored-map validation: map symbols, object rules and scalar constraints.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

pub(crate) const MAX_GENERATED_OBJECTS: usize = 20_000;

pub type V3 = (f32, f32, f32);
pub type V2 = (f32, f32);

mod loader;
mod schema;
mod validation;

pub use loader::AuthoredMapLoader;
pub use schema::*;
