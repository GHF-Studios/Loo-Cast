//! Authored geometry asset schema, validation and loading.

pub(crate) const MAX_GENERATED_OBJECTS: usize = 20_000;

pub type V3 = (f32, f32, f32);
pub type V2 = (f32, f32);

mod loader;
mod schema;
mod validation;

pub use loader::AuthoredMapLoader;
pub use schema::*;

#[cfg(test)]
mod tests;
